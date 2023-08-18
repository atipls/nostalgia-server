use crate::{
    reliability::{Frame, Reliability},
    Result,
};
use std::collections::HashMap;

#[derive(Debug)]
struct Fragment {
    pub flags: u8,
    pub compound_size: u32,
    pub ordered_frame_index: u32,
    pub frames: HashMap<u32, Frame>,
}

impl Fragment {
    pub fn new(flags: u8, compound_size: u32, ordered_frame_index: u32) -> Self {
        Self {
            flags,
            compound_size,
            ordered_frame_index,
            frames: HashMap::new(),
        }
    }

    pub fn full(&self) -> bool {
        self.frames.len() == self.compound_size as usize
    }

    pub fn insert(&mut self, frame: Frame) {
        if self.full() || self.frames.contains_key(&frame.fragment_index) {
            return;
        }

        self.frames.insert(frame.fragment_index, frame);
    }

    pub fn merge(&mut self) -> Result<Frame> {
        let mut keys: Vec<u32> = self.frames.keys().cloned().collect();
        keys.sort_unstable();

        let sequence_number = self.frames[keys.last().unwrap()].sequence_number;

        let mut buffer = vec![];
        for key_index in keys {
            buffer.append(&mut self.frames[&key_index].data.clone());
        }

        let mut frame_set = Frame::new(Reliability::from((self.flags & 0xE0) >> 5)?, buffer);
        frame_set.ordered_frame_index = self.ordered_frame_index;
        frame_set.sequence_number = sequence_number;

        Ok(frame_set)
    }
}

#[derive(Debug)]
pub struct FragmentQueue {
    fragments: HashMap<u16, Fragment>,
}

impl FragmentQueue {
    pub fn new() -> Self {
        Self {
            fragments: HashMap::new(),
        }
    }

    pub fn insert(&mut self, frame: Frame) {
        if self.fragments.contains_key(&frame.compound_id) {
            self.fragments
                .get_mut(&frame.compound_id)
                .unwrap()
                .insert(frame);
        } else {
            let mut v = Fragment::new(frame.flags, frame.compound_size, frame.ordered_frame_index);
            let k = frame.compound_id;
            v.insert(frame);
            self.fragments.insert(k, v);
        }
    }

    pub fn flush(&mut self) -> Result<Vec<Frame>> {
        let mut ret = vec![];

        let keys: Vec<u16> = self.fragments.keys().cloned().collect();

        for i in keys {
            let a = self.fragments.get_mut(&i).unwrap();
            if a.full() {
                ret.push(a.merge()?);
                self.fragments.remove(&i);
            }
        }

        Ok(ret)
    }

    pub fn size(&self) -> usize {
        self.fragments.len()
    }
}
