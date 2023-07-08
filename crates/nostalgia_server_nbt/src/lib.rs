use std::{
    collections::HashMap,
    io::{self, Cursor, Read},
};

use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug, Clone, PartialEq)]
pub enum Tag {
    End(),
    Byte(u8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    String(String),
    List(Vec<Tag>),
    Compound(HashMap<String, Tag>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

macro_rules! tag_getter {
    { $name:ident, $tag: ident, $result: ty } => {
        pub fn $name(&self, name: &str) -> Option<&$result> {
            let mut tag = self;
            for part in name.split('.') {
                match tag {
                    Tag::Compound(tags) => {
                        tag = tags.get(part)?;
                    }
                    _ => return None,
                }
            }

            match tag {
                Tag::$tag(value) => Some(value),
                _ => None,
            }
        }
    };
}

impl Tag {
    tag_getter! { get_byte, Byte, u8 }
    tag_getter! { get_short, Short, i16 }
    tag_getter! { get_int, Int, i32 }
    tag_getter! { get_long, Long, i64 }
    tag_getter! { get_float, Float, f32 }
    tag_getter! { get_double, Double, f64 }
    tag_getter! { get_byte_array, ByteArray, Vec<u8> }
    tag_getter! { get_string, String, String }
    tag_getter! { get_list, List, Vec<Tag> }
    tag_getter! { get_compound, Compound, HashMap<String, Tag> }
    tag_getter! { get_int_array, IntArray, Vec<i32> }
    tag_getter! { get_long_array, LongArray, Vec<i64> }
}

pub struct Nbt {
    root: Tag,
}

impl Nbt {
    fn read_string(cursor: &mut Cursor<Vec<u8>>) -> io::Result<String> {
        let length = cursor.read_u16::<LittleEndian>()?;
        let mut string = vec![0u8; length as usize];
        cursor.read_exact(&mut string)?;
        Ok(String::from_utf8(string).unwrap())
    }

    fn read_byte_array(cursor: &mut Cursor<Vec<u8>>) -> io::Result<Vec<u8>> {
        let length = cursor.read_i32::<LittleEndian>()?;
        let mut array = vec![0u8; length as usize];
        cursor.read_exact(&mut array)?;
        Ok(array)
    }

    fn read_int_array(cursor: &mut Cursor<Vec<u8>>) -> io::Result<Vec<i32>> {
        let length = cursor.read_i32::<LittleEndian>()?;
        let mut array = vec![0i32; length as usize];
        for i in 0..length {
            array[i as usize] = cursor.read_i32::<LittleEndian>()?;
        }
        Ok(array)
    }

    fn read_long_array(cursor: &mut Cursor<Vec<u8>>) -> io::Result<Vec<i64>> {
        let length = cursor.read_i32::<LittleEndian>()?;
        let mut array = vec![0i64; length as usize];
        for i in 0..length {
            array[i as usize] = cursor.read_i64::<LittleEndian>()?;
        }
        Ok(array)
    }

    fn read_list(cursor: &mut Cursor<Vec<u8>>) -> io::Result<Tag> {
        let tag_type = cursor.read_u8()?;
        let length = cursor.read_i32::<LittleEndian>()?;
        let mut list = Vec::with_capacity(length as usize);

        for _ in 0..length {
            let tag = match tag_type {
                0 => Tag::End(),
                1 => Tag::Byte(cursor.read_u8()?),
                2 => Tag::Short(cursor.read_i16::<LittleEndian>()?),
                3 => Tag::Int(cursor.read_i32::<LittleEndian>()?),
                4 => Tag::Long(cursor.read_i64::<LittleEndian>()?),
                5 => Tag::Float(cursor.read_f32::<LittleEndian>()?),
                6 => Tag::Double(cursor.read_f64::<LittleEndian>()?),
                7 => Tag::ByteArray(Self::read_byte_array(cursor)?),
                8 => Tag::String(Self::read_string(cursor)?),
                9 => Self::read_list(cursor)?,
                10 => Self::read_compound(cursor)?,
                11 => Tag::IntArray(Self::read_int_array(cursor)?),
                12 => Tag::LongArray(Self::read_long_array(cursor)?),
                _ => panic!("Unknown tag type: {}", tag_type),
            };

            list.push(tag);
        }

        Ok(Tag::List(list))
    }

    fn read_named_tag(cursor: &mut Cursor<Vec<u8>>) -> io::Result<(String, Tag)> {
        let tag_type = cursor.read_u8()?;
        if tag_type == 0 {
            return Ok((String::new(), Tag::End()));
        }

        let name = Self::read_string(cursor)?;

        let tag = match tag_type {
            0 => Tag::End(),
            1 => Tag::Byte(cursor.read_u8()?),
            2 => Tag::Short(cursor.read_i16::<LittleEndian>()?),
            3 => Tag::Int(cursor.read_i32::<LittleEndian>()?),
            4 => Tag::Long(cursor.read_i64::<LittleEndian>()?),
            5 => Tag::Float(cursor.read_f32::<LittleEndian>()?),
            6 => Tag::Double(cursor.read_f64::<LittleEndian>()?),
            7 => Tag::ByteArray(Self::read_byte_array(cursor)?),
            8 => Tag::String(Self::read_string(cursor)?),
            9 => Self::read_list(cursor)?,
            10 => Self::read_compound(cursor)?,
            11 => Tag::IntArray(Self::read_int_array(cursor)?),
            12 => Tag::LongArray(Self::read_long_array(cursor)?),
            _ => panic!("Unknown tag type: {}", tag_type),
        };

        Ok((name, tag))
    }

    fn read_compound(cursor: &mut Cursor<Vec<u8>>) -> io::Result<Tag> {
        let mut tags = HashMap::new();
        loop {
            let (tag_name, tag) = Self::read_named_tag(cursor)?;
            match tag {
                Tag::End() => break,
                _ => {
                    tags.insert(tag_name, tag);
                }
            }
        }

        Ok(Tag::Compound(tags))
    }

    pub fn from_bytes(mut cursor: &mut Cursor<Vec<u8>>) -> io::Result<Self> {
        let (.., tag) = Self::read_named_tag(&mut cursor)?;
        match tag {
            Tag::Compound(tags) => Ok(Self {
                root: Tag::Compound(tags),
            }),
            _ => panic!("Root tag must be a compound tag"),
        }
    }

    pub fn root(&self) -> &Tag {
        &self.root
    }

    pub fn root_mut(&mut self) -> &mut Tag {
        &mut self.root
    }
}
