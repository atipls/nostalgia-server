use crate::{
    fragment::FragmentQueue,
    {NetworkError, Result},
};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::{
    collections::{hash_map, HashMap},
    io::{Cursor, Read, Write},
    net::SocketAddr,
};

#[derive(Clone, Copy, Debug)]
pub enum Reliability {
    Unreliable = 0,
    UnreliableSequenced = 1,
    Reliable = 2,
    ReliableOrdered = 3,
    ReliableSequenced = 4,
}

impl Reliability {
    pub fn from(value: u8) -> Result<Self> {
        match value {
            0 => Ok(Reliability::Unreliable),
            1 => Ok(Reliability::UnreliableSequenced),
            2 => Ok(Reliability::Reliable),
            3 => Ok(Reliability::ReliableOrdered),
            4 => Ok(Reliability::ReliableSequenced),
            _ => Err(NetworkError::InvalidReliability),
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Reliability::Unreliable => 0,
            Reliability::UnreliableSequenced => 1,
            Reliability::Reliable => 2,
            Reliability::ReliableOrdered => 3,
            Reliability::ReliableSequenced => 4,
        }
    }
}

const NEEDS_B_AND_AS_FLAG: u8 = 0x4;
const CONTINUOUS_SEND_FLAG: u8 = 0x8;

#[derive(Clone, Debug)]
pub struct Frame {
    pub id: u8,
    pub sequence_number: u32,
    pub flags: u8,
    pub length_in_bytes: u16,
    pub reliable_frame_index: u32,
    pub sequenced_frame_index: u32,
    pub ordered_frame_index: u32,
    pub order_channel: u8,
    pub compound_size: u32,
    pub compound_id: u16,
    pub fragment_index: u32,
    pub data: Vec<u8>,
}

impl Frame {
    pub fn new(reliability: Reliability, data: Vec<u8>) -> Self {
        Self {
            id: 0,
            sequence_number: 0,
            flags: reliability.to_u8() << 5,
            length_in_bytes: data.len() as u16,
            reliable_frame_index: 0,
            sequenced_frame_index: 0,
            ordered_frame_index: 0,
            order_channel: 0,
            compound_size: 0,
            compound_id: 0,
            fragment_index: 0,
            data,
        }
    }

    pub fn serialize(&self, cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        // 16 = is fragmented
        let mut id = 0x80 | NEEDS_B_AND_AS_FLAG;
        if (self.flags & 16) != 0 && self.fragment_index != 0 {
            id |= CONTINUOUS_SEND_FLAG;
        }

        cursor.write_u8(id)?;
        cursor.write_u24::<LittleEndian>(self.sequence_number)?;
        cursor.write_u8(self.flags)?;
        cursor.write_u16::<BigEndian>(self.length_in_bytes * 8)?;

        if self.is_reliable()? {
            cursor.write_u24::<LittleEndian>(self.reliable_frame_index)?;
        }

        if self.is_sequenced()? {
            cursor.write_u24::<LittleEndian>(self.sequenced_frame_index)?;
        }

        if self.is_ordered()? {
            cursor.write_u24::<LittleEndian>(self.ordered_frame_index)?;
            cursor.write_u8(self.order_channel)?;
        }

        if (self.flags & 16) != 0 {
            cursor.write_u32::<LittleEndian>(self.compound_size)?;
            cursor.write_u16::<BigEndian>(self.compound_id)?;
            cursor.write_u32::<LittleEndian>(self.fragment_index)?;
        }

        cursor.write_all(&self.data)?;

        Ok(())
    }

    pub fn is_fragment(&self) -> bool {
        (self.flags & 16) != 0
    }

    pub fn is_reliable(&self) -> Result<bool> {
        Ok(match self.reliability()? {
            Reliability::Reliable
            | Reliability::ReliableOrdered
            | Reliability::ReliableSequenced => true,
            _ => false,
        })
    }

    pub fn is_ordered(&self) -> Result<bool> {
        Ok(match self.reliability()? {
            Reliability::UnreliableSequenced
            | Reliability::ReliableOrdered
            | Reliability::ReliableSequenced => true,
            _ => false,
        })
    }

    pub fn is_sequenced(&self) -> Result<bool> {
        Ok(match self.reliability()? {
            Reliability::UnreliableSequenced | Reliability::ReliableSequenced => true,
            _ => false,
        })
    }

    pub fn reliability(&self) -> Result<Reliability> {
        Reliability::from((self.flags & 0xE0) >> 5)
    }
}

pub struct FrameVec {
    pub id: u8,
    pub sequence_number: u32,
    pub frames: Vec<Frame>,
}

impl FrameVec {
    pub fn new(buffer: Vec<u8>) -> Result<Self> {
        let length = buffer.len() as u64;
        let mut cursor = Cursor::new(buffer);

        let id = cursor.read_u8()?;
        let sequence_number = cursor.read_u24::<LittleEndian>()?;
        let mut frames = vec![];

        while cursor.position() < length {
            let mut frame = Frame {
                id,
                sequence_number,
                flags: 0,
                length_in_bytes: 0,
                reliable_frame_index: 0,
                sequenced_frame_index: 0,
                ordered_frame_index: 0,
                order_channel: 0,
                compound_size: 0,
                compound_id: 0,
                fragment_index: 0,
                data: vec![],
            };

            frame.flags = cursor.read_u8()?;
            frame.length_in_bytes = cursor.read_u16::<BigEndian>()? / 8;

            if frame.is_reliable()? {
                frame.reliable_frame_index = cursor.read_u24::<LittleEndian>()?;
            }

            if frame.is_sequenced()? {
                frame.sequenced_frame_index = cursor.read_u24::<LittleEndian>()?;
            }

            if frame.is_ordered()? {
                frame.ordered_frame_index = cursor.read_u24::<LittleEndian>()?;
                frame.order_channel = cursor.read_u8()?;
            }

            if frame.is_fragment() {
                frame.compound_size = cursor.read_u32::<LittleEndian>()?;
                frame.compound_id = cursor.read_u16::<BigEndian>()?;
                frame.fragment_index = cursor.read_u32::<LittleEndian>()?;
            }

            let mut data = vec![0u8; frame.length_in_bytes as usize].into_boxed_slice();
            cursor.read(&mut data)?;
            frame.data = data.to_vec();

            frames.push(frame);
        }

        Ok(Self {
            id,
            sequence_number,
            frames,
        })
    }
}

#[derive(Debug)]
pub struct ACKSet {
    ack: Vec<(u32, u32)>,
    nack: Vec<(u32, u32)>,
    last_max: u32,
}

impl ACKSet {
    pub fn new() -> Self {
        ACKSet {
            ack: vec![],
            nack: vec![],
            last_max: 0,
        }
    }
    pub fn insert(&mut self, s: u32) {
        if s != 0 {
            if s > self.last_max && s != self.last_max + 1 {
                self.nack.push((self.last_max + 1, s - 1));
            }

            if s > self.last_max {
                self.last_max = s;
            }
        }

        for i in 0..self.ack.len() {
            let a = self.ack[i];
            if a.0 != 0 && s == a.0 - 1 {
                self.ack[i].0 = s;
                return;
            }
            if s == a.1 + 1 {
                self.ack[i].1 = s;
                return;
            }
        }
        self.ack.push((s, s));
    }

    pub fn get_ack(&mut self) -> Vec<(u32, u32)> {
        let ret = self.ack.clone();
        self.ack.clear();
        ret
    }

    pub fn get_nack(&mut self) -> Vec<(u32, u32)> {
        let ret = self.nack.clone();
        self.nack.clear();
        ret
    }
}

#[derive(Debug)]
pub struct RecvQueue {
    sequenced_frame_index: u32,
    last_ordered_index: u32,
    sequence_number_ackset: ACKSet,
    packets: HashMap<u32, Frame>,
    ordered_packets: HashMap<u32, Frame>,
    fragment_queue: FragmentQueue,
}

impl RecvQueue {
    pub fn new() -> Self {
        Self {
            sequence_number_ackset: ACKSet::new(),
            packets: HashMap::new(),
            fragment_queue: FragmentQueue::new(),
            ordered_packets: HashMap::new(),
            sequenced_frame_index: 0,
            last_ordered_index: 0,
        }
    }

    pub fn insert(&mut self, frame: Frame) -> Result<()> {
        if self.packets.contains_key(&frame.sequence_number) {
            return Ok(());
        }

        self.sequence_number_ackset.insert(frame.sequence_number);

        //The fourth parameter takes one of five major values. Lets say you send data 1,2,3,4,5,6. Here's the order and substance of what you might get back:
        match frame.reliability()? {
            // UNRELIABLE - 5, 1, 6
            Reliability::Unreliable => {
                self.packets.entry(frame.sequence_number).or_insert(frame);
            }
            // UNRELIABLE_SEQUENCED - 5 (6 was lost in transit, 1,2,3,4 arrived later than 5)
            // With the UNRELIABLE_SEQUENCED transmission method, the game data does not need to arrive in every packet to avoid packet loss and retransmission,
            // because the new packet represents the new state, and the new state can be used directly, without waiting for the old packet to arrive.
            Reliability::UnreliableSequenced => {
                let sequenced_frame_index = frame.sequenced_frame_index;
                if sequenced_frame_index >= self.sequenced_frame_index {
                    if let hash_map::Entry::Vacant(e) = self.packets.entry(frame.sequence_number) {
                        e.insert(frame);
                        self.sequenced_frame_index = sequenced_frame_index + 1;
                    }
                }
            }
            // RELIABLE - 5, 1, 4, 6, 2, 3
            Reliability::Reliable => {
                self.packets.insert(frame.sequence_number, frame);
            }
            // RELIABLE_ORDERED - 1, 2, 3, 4, 5, 6
            Reliability::ReliableOrdered => {
                // if remote host not received ack , and local program has flush ordered packet. recvq will insert old packet caused memory leak.
                if frame.ordered_frame_index < self.last_ordered_index {
                    return Ok(());
                }

                if frame.is_fragment() {
                    self.fragment_queue.insert(frame);

                    for i in self.fragment_queue.flush()? {
                        self.ordered_packets
                            .entry(i.ordered_frame_index)
                            .or_insert(i);
                    }
                } else {
                    self.ordered_packets
                        .entry(frame.ordered_frame_index)
                        .or_insert(frame);
                }
            }
            // RELIABLE_SEQUENCED - 5, 6 (1,2,3,4 arrived later than 5)
            Reliability::ReliableSequenced => {
                let sequenced_frame_index = frame.sequenced_frame_index;
                if sequenced_frame_index >= self.sequenced_frame_index {
                    if let hash_map::Entry::Vacant(e) = self.packets.entry(frame.sequence_number) {
                        e.insert(frame);
                        self.sequenced_frame_index = sequenced_frame_index + 1;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn get_ack(&mut self) -> Vec<(u32, u32)> {
        self.sequence_number_ackset.get_ack()
    }

    pub fn get_nack(&mut self) -> Vec<(u32, u32)> {
        self.sequence_number_ackset.get_nack()
    }

    pub fn flush(&mut self, _peer_addr: &SocketAddr) -> Vec<Frame> {
        let mut ret = vec![];
        let mut ordered_keys: Vec<u32> = self.ordered_packets.keys().cloned().collect();

        ordered_keys.sort_unstable();

        for i in ordered_keys {
            if i == self.last_ordered_index {
                let frame = self.ordered_packets[&i].clone();
                ret.push(frame);
                self.ordered_packets.remove(&i);
                self.last_ordered_index = i + 1;
            }
        }

        let mut packets_keys: Vec<u32> = self.packets.keys().cloned().collect();
        packets_keys.sort_unstable();

        for i in packets_keys {
            let v = self.packets.get(&i).unwrap();
            ret.push(v.clone());
        }

        self.packets.clear();
        ret
    }
    pub fn get_ordered_packet(&self) -> usize {
        self.ordered_packets.len()
    }

    pub fn get_fragment_queue_size(&self) -> usize {
        self.fragment_queue.size()
    }

    pub fn get_ordered_keys(&self) -> Vec<u32> {
        self.ordered_packets.keys().cloned().collect()
    }

    pub fn get_size(&self) -> usize {
        self.packets.len()
    }
}

#[derive(Debug)]
pub struct SendQueue {
    mtu: u16,
    ack_sequence_number: u32,
    sequence_number: u32,
    reliable_frame_index: u32,
    sequenced_frame_index: u32,
    ordered_frame_index: u32,
    compound_id: u16,
    packets: Vec<Frame>,
    retransmission_timeout: u64,
    smooth_roundtrip_time: u64,
    sent_packet: Vec<(Frame, bool, u64, u32, Vec<u32>)>,
}

impl SendQueue {
    pub const DEFAULT_TIMEOUT_MILLS: u64 = 50;

    const RTO_UBOUND: u64 = 12000;
    const RTO_LBOUND: u64 = 50;

    pub fn new(mtu: u16) -> Self {
        Self {
            mtu,
            ack_sequence_number: 0,
            sequence_number: 0,
            packets: vec![],
            sent_packet: vec![],
            reliable_frame_index: 0,
            sequenced_frame_index: 0,
            ordered_frame_index: 0,
            compound_id: 0,

            retransmission_timeout: SendQueue::DEFAULT_TIMEOUT_MILLS,
            smooth_roundtrip_time: SendQueue::DEFAULT_TIMEOUT_MILLS,
        }
    }

    pub fn insert(&mut self, reliability: Reliability, buf: &[u8]) -> Result<()> {
        match reliability {
            Reliability::Unreliable => {
                // 60 = max framesetpacket length(27) + udp overhead(28) + 5 ext
                if buf.len() > (self.mtu - 60).into() {
                    return Err(NetworkError::PacketTooLarge);
                }

                let frame = Frame::new(reliability, buf.to_vec());
                self.packets.push(frame);
            }
            Reliability::UnreliableSequenced => {
                // 60 = max framesetpacket length(27) + udp overhead(28) + 5 ext
                if buf.len() > (self.mtu - 60).into() {
                    return Err(NetworkError::PacketTooLarge);
                }

                let mut frame = Frame::new(reliability, buf.to_vec());
                // I dont know why Sequenced packet need Ordered
                // https://wiki.vg/Raknet_Protocol
                frame.ordered_frame_index = self.ordered_frame_index;
                frame.sequenced_frame_index = self.sequenced_frame_index;
                self.packets.push(frame);
                self.sequenced_frame_index += 1;
            }
            Reliability::Reliable => {
                // 60 = max framesetpacket length(27) + udp overhead(28) + 5 ext
                if buf.len() > (self.mtu - 60).into() {
                    return Err(NetworkError::PacketTooLarge);
                }

                let mut frame = Frame::new(reliability, buf.to_vec());
                frame.reliable_frame_index = self.reliable_frame_index;
                self.packets.push(frame);
                self.reliable_frame_index += 1;
            }
            Reliability::ReliableOrdered => {
                // 60 = max framesetpacket length(27) + udp overhead(28) + 5 ext
                if buf.len() < (self.mtu - 60).into() {
                    let mut frame = Frame::new(reliability, buf.to_vec());
                    frame.reliable_frame_index = self.reliable_frame_index;
                    frame.ordered_frame_index = self.ordered_frame_index;
                    self.packets.push(frame);
                    self.reliable_frame_index += 1;
                    self.ordered_frame_index += 1;
                } else {
                    let max = (self.mtu - 60) as usize;
                    let mut compound_size = buf.len() / max;
                    if buf.len() % max != 0 {
                        compound_size += 1;
                    }

                    for i in 0..compound_size {
                        let begin = (max * i) as usize;
                        let end = if i == compound_size - 1 {
                            buf.len()
                        } else {
                            (max * (i + 1)) as usize
                        };

                        let mut frame = Frame::new(reliability, buf[begin..end].to_vec());
                        // set fragment flag
                        frame.flags |= 16;
                        frame.compound_size = compound_size as u32;
                        frame.compound_id = self.compound_id;
                        frame.fragment_index = i as u32;
                        frame.reliable_frame_index = self.reliable_frame_index;
                        frame.ordered_frame_index = self.ordered_frame_index;
                        self.packets.push(frame);
                        self.reliable_frame_index += 1;
                    }
                    self.compound_id += 1;
                    self.ordered_frame_index += 1;
                }
            }
            Reliability::ReliableSequenced => {
                // 60 = max framesetpacket length(27) + udp overhead(28) + 5 ext
                if buf.len() > (self.mtu - 60).into() {
                    return Err(NetworkError::PacketTooLarge);
                }

                let mut frame = Frame::new(reliability, buf.to_vec());
                frame.reliable_frame_index = self.reliable_frame_index;
                frame.sequenced_frame_index = self.sequenced_frame_index;
                // I dont know why Sequenced packet need Ordered
                // https://wiki.vg/Raknet_Protocol
                frame.ordered_frame_index = self.ordered_frame_index;
                self.packets.push(frame);
                self.reliable_frame_index += 1;
                self.sequenced_frame_index += 1;
            }
        };

        Ok(())
    }

    fn update_retransmission_timeout(&mut self, roundtrip_time: u64) {
        self.smooth_roundtrip_time =
            ((self.smooth_roundtrip_time as f64 * 0.8) + (roundtrip_time as f64 * 0.2)) as u64;
        self.retransmission_timeout = match (1.5 * self.smooth_roundtrip_time as f64) as u64 {
            rto if rto < SendQueue::RTO_LBOUND => SendQueue::RTO_LBOUND,
            rto if rto > SendQueue::RTO_UBOUND => SendQueue::RTO_UBOUND,
            rto => rto,
        };
    }

    pub fn get_rto(&self) -> u64 {
        self.retransmission_timeout
    }

    pub fn nack(&mut self, sequence: u32, tick: u64) {
        for i in 0..self.sent_packet.len() {
            let item = &mut self.sent_packet[i];
            if item.1 && item.0.sequence_number == sequence {
                item.0.sequence_number = self.sequence_number;
                self.sequence_number += 1;
                item.2 = tick;
                item.3 += 1;
                item.4.push(item.0.sequence_number);
            }
        }
    }

    pub fn ack(&mut self, sequence: u32, tick: u64) {
        if sequence != 0 && sequence != self.ack_sequence_number + 1 {
            for i in self.ack_sequence_number + 1..sequence {
                self.nack(i, tick);
            }
        }

        self.ack_sequence_number = sequence;

        let mut rtts = vec![];

        for i in 0..self.sent_packet.len() {
            let item = &mut self.sent_packet[i];
            if item.0.sequence_number == sequence || item.4.contains(&sequence) {
                rtts.push(tick - item.2);
                self.sent_packet.remove(i);
                break;
            }
        }

        for i in rtts {
            self.update_retransmission_timeout(i);
        }
    }

    fn tick(&mut self, tick: u64) {
        for i in 0..self.sent_packet.len() {
            let p = &mut self.sent_packet[i];

            let mut cur_rto = self.retransmission_timeout;

            // TCP timeout calculation is RTOx2, so three consecutive packet losses will make it RTOx8, which is very terrible,
            // while rust-raknet it is not x2, but x1.5 (Experimental results show that the value of 1.5 is relatively good), which has improved the transmission speed.
            for _ in 0..p.3 {
                cur_rto = (cur_rto as f64 * 1.5) as u64;
            }

            if p.1 && tick - p.2 >= cur_rto {
                p.0.sequence_number = self.sequence_number;
                self.sequence_number += 1;
                p.1 = false;
                p.4.push(p.0.sequence_number);
            }
        }
    }

    pub fn flush(&mut self, tick: u64, _peer_addr: &SocketAddr) -> Vec<Frame> {
        self.tick(tick);

        let mut ret = vec![];

        if !self.sent_packet.is_empty() {
            self.sent_packet
                .sort_by(|x, y| x.0.sequence_number.cmp(&y.0.sequence_number));

            for i in 0..self.sent_packet.len() {
                let p = &mut self.sent_packet[i];
                if !p.1 {
                    ret.push(p.0.clone());
                    p.1 = true;
                    p.2 = tick;
                    p.3 += 1;
                }
            }
            return ret;
        }

        if !self.packets.is_empty() {
            for i in 0..self.packets.len() {
                self.packets[i].sequence_number = self.sequence_number;
                self.sequence_number += 1;
                ret.push(self.packets[i].clone());
                if self.packets[i].is_reliable().unwrap() {
                    self.sent_packet.push((
                        self.packets[i].clone(),
                        true,
                        tick,
                        0,
                        vec![self.packets[i].sequence_number],
                    ));
                }
            }

            self.packets.clear();
        }

        ret
    }

    pub fn is_empty(&self) -> bool {
        self.packets.is_empty() && self.sent_packet.is_empty()
    }

    pub fn get_reliable_queue_size(&self) -> usize {
        self.packets.len()
    }

    pub fn get_sent_queue_size(&self) -> usize {
        self.sent_packet.len()
    }
}
