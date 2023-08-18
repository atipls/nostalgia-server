mod connection;

use connection::Connection;
use network::{listener::Listener, NetworkError};
use protocol::Packet;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::{
    mpsc::{channel, Receiver, Sender},
    Mutex,
};
use world::World;

struct Application {
    listener: Listener,
    world: Arc<Mutex<World>>,
    connections: Arc<Mutex<Vec<Arc<Mutex<Connection>>>>>,
    global_packet_sender: Arc<Mutex<Sender<Packet>>>,
    global_packet_receiver: Arc<Mutex<Receiver<Packet>>>,
    disconnection_sender: Arc<Mutex<Sender<Connection>>>,
    disconnection_receiver: Arc<Mutex<Receiver<Connection>>>,
}

impl Application {
    pub fn new(listener: Listener, world: World) -> Self {
        let (global_packet_sender, global_packet_receiver) = channel(64);
        let (disconnection_sender, disconnection_receiver) = channel(64);

        Self {
            listener,
            world: Arc::new(Mutex::new(world)),
            connections: Arc::new(Mutex::new(Vec::new())),
            global_packet_sender: Arc::new(Mutex::new(global_packet_sender)),
            global_packet_receiver: Arc::new(Mutex::new(global_packet_receiver)),
            disconnection_sender: Arc::new(Mutex::new(disconnection_sender)),
            disconnection_receiver: Arc::new(Mutex::new(disconnection_receiver)),
        }
    }

    pub async fn run(&mut self) -> Result<(), NetworkError> {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            let peer = self.listener.accept().await?;

            let world = self.world.clone();
            let connection = Arc::new(Mutex::new(Connection::new(peer, world)));
            let mut connections = self.connections.lock().await;
            connections.push(connection.clone());

            let global_packet_sender = self.global_packet_sender.clone();
            tokio::spawn(async move {
                loop {
                    let global_packet_sender = global_packet_sender.clone();

                    let mut connection = connection.lock().await;
                    match connection.update(global_packet_sender).await {
                        Ok(_) => {}
                        Err(NetworkError::ConnectionClosed) => {
                            println!("Connection closed (disconnected)");
                            break;
                        }
                        Err(error) => println!("Connection closed ({:#?})", error),
                    }
                }
            });

            let global_packet_receiver = self.global_packet_receiver.clone();
            let disconnection_receiver = self.disconnection_receiver.clone();
            let connections = self.connections.clone();
            tokio::spawn(async move {
                loop {
                    let mut global_packet_receiver = global_packet_receiver.lock().await;
                    let mut disconnection_receiver = disconnection_receiver.lock().await;
                    let connections = connections.lock().await;
                    tokio::select! {
                        packet = global_packet_receiver.recv() => {
                            if let Some(packet) = packet {
                                for connection in connections.iter() {
                                    let mut connection = connection.lock().await;
                                    connection.send_packet(packet.clone()).await.expect("Failed to send a global packet");
                                }
                            }
                        }
                        connection = disconnection_receiver.recv() => {
                            let mut new_connections = vec![];
                            if let Some(disconnected_connection) = connection {
                                for other_connection in connections.iter() {
                                    let locked_connection = other_connection.lock().await;
                                    if locked_connection.peer() == disconnected_connection.peer() {
                                        continue;
                                    }

                                    new_connections.push(other_connection);
                                }
                            }
                        }
                    }
                }
            });
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Test {
    value: i32,
}

impl Test {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn update(mut self) {
        self.value += 1;
    }

    fn call_update(self) {
        { self }.update();
    }
}

#[tokio::main]
async fn main() {
    let mut test = Test::new();
    if true {
        test.call_update();
    }
    test.call_update();

    println!("{:#?}", test);

    let address = "0.0.0.0:19132".parse().expect("Address is already in use");
    let listener = Listener::started(&address, "Nostalgia Server".to_string())
        .await
        .expect("Failed to start the server");

    let world_path = PathBuf::from("assets/MainWorld");
    let world = world::World::from_file(world_path).expect("Failed to load the world");

    let mut application = Application::new(listener, world);

    match application.run().await {
        Ok(_) => println!("Server closed"),
        Err(error) => println!("Server closed ({:#?})", error),
    }
}
