use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read, Result};
use types::{ItemInstance, Vector3};

#[inline]
pub fn read_i8(cursor: &mut Cursor<Vec<u8>>) -> Result<i8> {
    cursor.read_i8()
}

#[inline]
pub fn read_u8(cursor: &mut Cursor<Vec<u8>>) -> Result<u8> {
    cursor.read_u8()
}

#[inline]
pub fn read_i16(cursor: &mut Cursor<Vec<u8>>) -> Result<i16> {
    cursor.read_i16::<BigEndian>()
}

#[inline]
pub fn read_u16(cursor: &mut Cursor<Vec<u8>>) -> Result<u16> {
    cursor.read_u16::<BigEndian>()
}

#[inline]
pub fn read_i24(cursor: &mut Cursor<Vec<u8>>) -> Result<i32> {
    cursor.read_i24::<LittleEndian>()
}

#[inline]
pub fn read_u24(cursor: &mut Cursor<Vec<u8>>) -> Result<u32> {
    cursor.read_u24::<LittleEndian>()
}

#[inline]
pub fn read_i32(cursor: &mut Cursor<Vec<u8>>) -> Result<i32> {
    cursor.read_i32::<BigEndian>()
}

#[inline]
pub fn read_u32(cursor: &mut Cursor<Vec<u8>>) -> Result<u32> {
    cursor.read_u32::<BigEndian>()
}

#[inline]
pub fn read_i64(cursor: &mut Cursor<Vec<u8>>) -> Result<i64> {
    cursor.read_i64::<BigEndian>()
}

#[inline]
pub fn read_u64(cursor: &mut Cursor<Vec<u8>>) -> Result<u64> {
    cursor.read_u64::<BigEndian>()
}

#[inline]
pub fn read_f32(cursor: &mut Cursor<Vec<u8>>) -> Result<f32> {
    cursor.read_f32::<BigEndian>()
}

#[inline]
pub fn read_f64(cursor: &mut Cursor<Vec<u8>>) -> Result<f64> {
    cursor.read_f64::<BigEndian>()
}

#[inline]
pub fn read_string(cursor: &mut Cursor<Vec<u8>>) -> Result<String> {
    let length = read_u16(cursor)? as usize;
    let mut buffer = vec![0; length];
    cursor.read_exact(&mut buffer)?;
    Ok(String::from_utf8(buffer).expect("Failed to parse a string"))
}

#[inline]
pub fn read_vector3(cursor: &mut Cursor<Vec<u8>>) -> Result<Vector3> {
    let x = read_f32(cursor)?;
    let y = read_f32(cursor)?;
    let z = read_f32(cursor)?;
    Ok(Vector3 { x, y, z })
}

#[inline]
pub fn read_item_instance(cursor: &mut Cursor<Vec<u8>>) -> Result<ItemInstance> {
    let id = read_i16(cursor)?;
    let count = read_i8(cursor)?;
    let metadata = read_i16(cursor)?;
    Ok(ItemInstance {
        id,
        count,
        metadata,
    })
}

#[inline]
pub fn read_item_instance_list(cursor: &mut Cursor<Vec<u8>>) -> Result<Vec<ItemInstance>> {
    let length = read_u16(cursor)? as usize;
    let mut items = Vec::with_capacity(length);
    for _ in 0..length {
        items.push(read_item_instance(cursor)?);
    }
    Ok(items)
}
