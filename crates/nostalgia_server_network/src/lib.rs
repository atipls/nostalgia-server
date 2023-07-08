use std::sync::atomic::AtomicU64;

pub mod fragment;
pub mod listener;
pub mod peer;
pub mod protocol;
pub mod reliability;

pub(crate) const PROTOCOL_VERSION: u8 = 10;

pub type Result<T> = std::result::Result<T, NetworkError>;

#[derive(Debug)]
pub enum NetworkError {
    Io(std::io::Error),

    InvalidReliability,
    InvalidPacketHeader,
    PacketTooLarge,

    NotListening,
    AlreadyListening,

    ConnectionClosed,
    SocketError,
    BindError,
    ReceiveTimeout,
}

impl From<std::io::Error> for NetworkError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

static APPLICATION_STARTED_AT: AtomicU64 = AtomicU64::new(0);

pub(crate) fn current_timestamp_milliseconds() -> u64 {
    fn get_current_unix_time() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }

    if APPLICATION_STARTED_AT.load(std::sync::atomic::Ordering::Relaxed) == 0 {
        APPLICATION_STARTED_AT.store(
            get_current_unix_time(),
            std::sync::atomic::Ordering::Relaxed,
        );
    }

    get_current_unix_time() - APPLICATION_STARTED_AT.load(std::sync::atomic::Ordering::Relaxed)
}
