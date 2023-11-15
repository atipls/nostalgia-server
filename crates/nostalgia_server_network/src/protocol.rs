use crate::Result;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::{
    io::{Cursor, Seek, SeekFrom, Write},
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

const RAKNET_MAGIC: &[u8; 16] = &[
    0x00, 0xFF, 0xFF, 0x00, 0xFE, 0xFE, 0xFE, 0xFE, 0xFD, 0xFD, 0xFD, 0xFD, 0x12, 0x34, 0x56, 0x78,
];

mod extensions {
    use crate::Result;
    use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
    use std::{
        io::{Cursor, Read, Write},
        net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    };

    #[inline]
    fn flip_the_bits_v4(bits: [u8; 4]) -> [u8; 4] {
        (u32::from_be_bytes(bits) ^ 0xFFFFFFFFu32).to_be_bytes()
    }

    #[inline]
    fn flip_the_bits_v6(bits: [u8; 16]) -> [u8; 16] {
        (u128::from_be_bytes(bits) ^ 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFu128).to_be_bytes()
    }

    pub fn read_string(cursor: &mut Cursor<Vec<u8>>) -> Result<String> {
        let length = cursor.read_u16::<BigEndian>()?;
        let mut buffer = vec![0u8; length as usize];
        cursor.read_exact(&mut buffer)?;
        Ok(unsafe { String::from_utf8_unchecked(buffer) })
    }

    pub fn read_address(cursor: &mut Cursor<Vec<u8>>) -> Result<SocketAddr> {
        let address = if cursor.read_u8()? == 4 {
            let mut buffer = [0u8; 4];
            cursor.read_exact(&mut buffer)?;
            buffer = flip_the_bits_v4(buffer);
            IpAddr::V4(Ipv4Addr::from(buffer))
        } else {
            let mut buffer = [0u8; 16];
            cursor.read_exact(&mut buffer)?;
            buffer = flip_the_bits_v6(buffer);
            IpAddr::V6(Ipv6Addr::from(buffer))
        };

        let port = cursor.read_u16::<BigEndian>()?;
        Ok(SocketAddr::new(address, port))
    }

    pub fn write_string(cursor: &mut Cursor<Vec<u8>>, string: &str) -> Result<()> {
        cursor.write_u16::<BigEndian>(string.len() as u16)?;
        cursor.write_all(string.as_bytes())?;
        Ok(())
    }

    pub fn write_address(cursor: &mut Cursor<Vec<u8>>, address: &SocketAddr) -> Result<()> {
        match address.ip() {
            IpAddr::V4(ip) => {
                cursor.write_u8(4)?;
                cursor.write_all(&flip_the_bits_v4(ip.octets()))?;
            }
            IpAddr::V6(ip) => {
                cursor.write_u8(6)?;
                cursor.write_all(&flip_the_bits_v6(ip.octets()))?;
            }
        }
        cursor.write_u16::<BigEndian>(address.port())?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum UnconnectedPacket {
    Ping {
        timestamp: u64,
    },

    Pong {
        timestamp: u64,
        server_guid: u64,
        motd: String,
    },

    ConnectionRequest {
        protocol_version: u8,
    },

    ConnectionReply {
        server_guid: u64,
        mtu_size: u16,
        use_encryption: bool,
    },

    ConnectionEstablish {
        server_address: SocketAddr,
        client_guid: u64,
        mtu_size: u16,
    },

    ConnectionEstablished {
        address: SocketAddr,
        server_guid: u64,
        mtu_size: u16,
        use_encryption: bool,
    },
}

impl UnconnectedPacket {
    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Option<UnconnectedPacket>> {
        Ok(match cursor.read_u8()? {
            0x01 => Some(UnconnectedPacket::Ping {
                timestamp: cursor.read_u64::<BigEndian>()?,
            }),
            0x1C => {
                let timestamp = cursor.read_u64::<BigEndian>()?;
                let server_guid = cursor.read_u64::<BigEndian>()?;
                cursor.seek(SeekFrom::Current(16))?; // Magic
                let motd = extensions::read_string(&mut cursor)?;

                Some(UnconnectedPacket::Pong {
                    timestamp,
                    server_guid,
                    motd,
                })
            }
            0x05 => {
                cursor.seek(SeekFrom::Current(16))?; // Magic
                let protocol_version = cursor.read_u8()?;

                Some(UnconnectedPacket::ConnectionRequest { protocol_version })
            }
            0x06 => {
                cursor.seek(SeekFrom::Current(16))?; // Magic
                let server_guid = cursor.read_u64::<BigEndian>()?;
                let use_encryption = cursor.read_u8()? == 1;
                let mtu_size = cursor.read_u16::<BigEndian>()?;

                Some(UnconnectedPacket::ConnectionReply {
                    server_guid,
                    mtu_size,
                    use_encryption,
                })
            }
            0x07 => {
                cursor.seek(SeekFrom::Current(16))?; // Magic
                let server_address = extensions::read_address(&mut cursor)?;
                let mtu_size = cursor.read_u16::<BigEndian>()?;
                let client_guid = cursor.read_u64::<BigEndian>()?;

                Some(UnconnectedPacket::ConnectionEstablish {
                    server_address,
                    client_guid,
                    mtu_size,
                })
            }
            0x08 => {
                cursor.seek(SeekFrom::Current(16))?; // Magic
                let server_guid = cursor.read_u64::<BigEndian>()?;
                let address = extensions::read_address(&mut cursor)?;
                let mtu_size = cursor.read_u16::<BigEndian>()?;
                let use_encryption = cursor.read_u8()? == 1;

                Some(UnconnectedPacket::ConnectionEstablished {
                    address,
                    server_guid,
                    mtu_size,
                    use_encryption,
                })
            }
            _ => None,
        })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        match self {
            UnconnectedPacket::Ping { timestamp } => {
                cursor.write_u8(0x01)?;
                cursor.write_u64::<BigEndian>(*timestamp)?;
            }
            UnconnectedPacket::Pong {
                timestamp,
                server_guid,
                motd,
            } => {
                cursor.write_u8(0x1C)?;
                cursor.write_u64::<BigEndian>(*timestamp)?;
                cursor.write_u64::<BigEndian>(*server_guid)?;
                cursor.write_all(RAKNET_MAGIC)?;
                extensions::write_string(&mut cursor, motd)?;
            }
            UnconnectedPacket::ConnectionRequest { protocol_version } => {
                cursor.write_u8(0x05)?;
                cursor.write_all(RAKNET_MAGIC)?;
                cursor.write_u8(*protocol_version)?;
            }
            UnconnectedPacket::ConnectionReply {
                server_guid,
                mtu_size,
                use_encryption,
            } => {
                cursor.write_u8(0x06)?;
                cursor.write_all(RAKNET_MAGIC)?;
                cursor.write_u64::<BigEndian>(*server_guid)?;
                cursor.write_u8(*use_encryption as u8)?;
                cursor.write_u16::<BigEndian>(*mtu_size)?;
            }
            UnconnectedPacket::ConnectionEstablish {
                server_address,
                client_guid,
                mtu_size,
            } => {
                cursor.write_u8(0x07)?;
                cursor.write_all(RAKNET_MAGIC)?;
                extensions::write_address(&mut cursor, server_address)?;
                cursor.write_u16::<BigEndian>(*mtu_size)?;
                cursor.write_u64::<BigEndian>(*client_guid)?;
            }
            UnconnectedPacket::ConnectionEstablished {
                address,
                server_guid,
                mtu_size,
                use_encryption,
            } => {
                cursor.write_u8(0x08)?;
                cursor.write_all(RAKNET_MAGIC)?;
                cursor.write_u64::<BigEndian>(*server_guid)?;
                extensions::write_address(&mut cursor, address)?;
                cursor.write_u16::<BigEndian>(*mtu_size)?;
                cursor.write_u8(*use_encryption as u8)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum ConnectedPacket {
    Ping {
        timestamp: u64,
    },
    Pong {
        timestamp: u64,
        server_time: u64,
    },
    ConnectionRequest {
        client_guid: u64,
        timestamp: u64,
        use_encryption: bool,
    },
    ConnectionRequestAccepted {
        client_address: SocketAddr,
        system_index: u16,
        request_timestamp: u64,
        accepted_timestamp: u64,
    },
    NewIncomingConnection {
        server_address: SocketAddr,
        request_timestamp: u64,
        accepted_timestamp: u64,
    },
    DisconnectionNotification,
    Acknowledge {
        length: u16,
        records: Vec<(u32, u32)>,
    },
    NegativeAcknowledge {
        length: u16,
        records: Vec<(u32, u32)>,
    },
}

impl ConnectedPacket {
    fn read_records(mut cursor: &mut Cursor<Vec<u8>>) -> Result<(u16, Vec<(u32, u32)>)> {
        let length = cursor.read_u16::<BigEndian>()?;
        let mut records = Vec::with_capacity(length as usize);
        for _ in 0..length {
            let start_same_as_end = cursor.read_u8()? == 1;
            let start = cursor.read_u24::<LittleEndian>()?;
            let end = if start_same_as_end {
                start
            } else {
                cursor.read_u24::<LittleEndian>()?
            };

            records.push((start, end));
        }

        Ok((length, records))
    }

    fn write_records(
        mut cursor: &mut Cursor<Vec<u8>>,
        length: u16,
        records: &[(u32, u32)],
    ) -> Result<()> {
        cursor.write_u16::<BigEndian>(length)?;
        for (range_start, range_end) in records {
            cursor.write_u8(if range_start == range_end { 0x01 } else { 0x00 })?;
            cursor.write_u24::<LittleEndian>(*range_start)?;
            if range_start != range_end {
                cursor.write_u24::<LittleEndian>(*range_end)?;
            }
        }
        Ok(())
    }

    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Option<ConnectedPacket>> {
        Ok(match cursor.read_u8()? {
            0x00 => Some(ConnectedPacket::Ping {
                timestamp: cursor.read_u64::<BigEndian>()?,
            }),
            0x03 => Some(ConnectedPacket::Pong {
                timestamp: cursor.read_u64::<BigEndian>()?,
                server_time: cursor.read_u64::<BigEndian>()?,
            }),
            0x09 => {
                let client_guid = cursor.read_u64::<BigEndian>()?;
                let timestamp = cursor.read_u64::<BigEndian>()?;
                let use_encryption = cursor.read_u8()? == 1;

                Some(ConnectedPacket::ConnectionRequest {
                    client_guid,
                    timestamp,
                    use_encryption,
                })
            }
            0x10 => {
                let client_address = extensions::read_address(&mut cursor)?;
                let system_index = cursor.read_u16::<BigEndian>()?;
                for _ in 0..10 {
                    _ = extensions::read_address(&mut cursor)?;
                }
                let request_timestamp = cursor.read_u64::<BigEndian>()?;
                let accepted_timestamp = cursor.read_u64::<BigEndian>()?;

                Some(ConnectedPacket::ConnectionRequestAccepted {
                    client_address,
                    system_index,
                    request_timestamp,
                    accepted_timestamp,
                })
            }
            0x13 => {
                let client_address = extensions::read_address(&mut cursor)?;
                for _ in 0..10 {
                    _ = extensions::read_address(&mut cursor)?;
                }
                let request_timestamp = cursor.read_u64::<BigEndian>()?;
                let accepted_timestamp = cursor.read_u64::<BigEndian>()?;

                Some(ConnectedPacket::NewIncomingConnection {
                    server_address: client_address,
                    request_timestamp,
                    accepted_timestamp,
                })
            }
            0x15 => Some(ConnectedPacket::DisconnectionNotification),
            0xC0 => {
                let (length, records) = ConnectedPacket::read_records(&mut cursor)?;
                Some(ConnectedPacket::Acknowledge { length, records })
            }
            0xA0 => {
                let (length, records) = ConnectedPacket::read_records(&mut cursor)?;
                Some(ConnectedPacket::NegativeAcknowledge { length, records })
            }
            _ => None,
        })
    }

    pub fn is_frame_packet(packet_id: u8) -> bool {
        packet_id & 0x80 != 0
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        match self {
            ConnectedPacket::Ping { timestamp } => {
                cursor.write_u8(0x00)?;
                cursor.write_u64::<BigEndian>(*timestamp)?;
            }
            ConnectedPacket::Pong {
                timestamp,
                server_time,
            } => {
                cursor.write_u8(0x03)?;
                cursor.write_u64::<BigEndian>(*timestamp)?;
                cursor.write_u64::<BigEndian>(*server_time)?;
            }
            ConnectedPacket::ConnectionRequest {
                client_guid,
                timestamp,
                use_encryption,
            } => {
                cursor.write_u8(0x05)?;
                cursor.write_u64::<BigEndian>(*client_guid)?;
                cursor.write_u64::<BigEndian>(*timestamp)?;
                cursor.write_u8(*use_encryption as u8)?;
            }
            ConnectedPacket::ConnectionRequestAccepted {
                client_address,
                system_index,
                request_timestamp,
                accepted_timestamp,
            } => {
                cursor.write_u8(0x10)?;
                extensions::write_address(&mut cursor, client_address)?;
                cursor.write_u16::<BigEndian>(*system_index)?;
                let loopback = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0);
                let broadcast = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(255, 255, 255, 255)), 0);

                extensions::write_address(&mut cursor, &loopback)?;
                for _ in 0..9 {
                    extensions::write_address(&mut cursor, &broadcast)?;
                }

                cursor.write_u64::<BigEndian>(*request_timestamp)?;
                cursor.write_u64::<BigEndian>(*accepted_timestamp)?;
            }
            ConnectedPacket::NewIncomingConnection {
                server_address: client_address,
                request_timestamp,
                accepted_timestamp,
            } => {
                cursor.write_u8(0x13)?;
                extensions::write_address(&mut cursor, client_address)?;
                let loopback = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0);
                let broadcast = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(255, 255, 255, 255)), 0);

                extensions::write_address(&mut cursor, &loopback)?;
                for _ in 0..9 {
                    extensions::write_address(&mut cursor, &broadcast)?;
                }

                cursor.write_u64::<BigEndian>(*request_timestamp)?;
                cursor.write_u64::<BigEndian>(*accepted_timestamp)?;
            }
            ConnectedPacket::DisconnectionNotification => {
                cursor.write_u8(0x15)?;
            }
            ConnectedPacket::Acknowledge { length, records } => {
                cursor.write_u8(0xC0)?;
                ConnectedPacket::write_records(&mut cursor, *length, records)?;
            }
            ConnectedPacket::NegativeAcknowledge { length, records } => {
                cursor.write_u8(0xA0)?;
                ConnectedPacket::write_records(&mut cursor, *length, records)?;
            }
        }
        Ok(())
    }
}
