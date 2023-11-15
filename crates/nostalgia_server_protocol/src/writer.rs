use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use std::io::{Cursor, Result, Write};
use types::{ItemInstance, Vector3};

#[inline]
pub fn write_i8(cursor: &mut Cursor<Vec<u8>>, value: i8) -> Result<()> {
    cursor.write_i8(value)
}

#[inline]
pub fn write_u8(cursor: &mut Cursor<Vec<u8>>, value: u8) -> Result<()> {
    cursor.write_u8(value)
}

#[inline]
pub fn write_i16(cursor: &mut Cursor<Vec<u8>>, value: i16) -> Result<()> {
    cursor.write_i16::<BigEndian>(value)
}

#[inline]
pub fn write_i16_le(cursor: &mut Cursor<Vec<u8>>, value: i16) -> Result<()> {
    cursor.write_i16::<LittleEndian>(value)
}

#[inline]
pub fn write_u16(cursor: &mut Cursor<Vec<u8>>, value: u16) -> Result<()> {
    cursor.write_u16::<BigEndian>(value)
}

#[inline]
pub fn write_i24(cursor: &mut Cursor<Vec<u8>>, value: i32) -> Result<()> {
    cursor.write_i24::<LittleEndian>(value)
}

#[inline]
pub fn write_u24(cursor: &mut Cursor<Vec<u8>>, value: u32) -> Result<()> {
    cursor.write_u24::<LittleEndian>(value)
}

#[inline]
pub fn write_i32(cursor: &mut Cursor<Vec<u8>>, value: i32) -> Result<()> {
    cursor.write_i32::<BigEndian>(value)
}

#[inline]
pub fn write_i32_le(cursor: &mut Cursor<Vec<u8>>, value: i32) -> Result<()> {
    cursor.write_i32::<LittleEndian>(value)
}

#[inline]
pub fn write_u32(cursor: &mut Cursor<Vec<u8>>, value: u32) -> Result<()> {
    cursor.write_u32::<BigEndian>(value)
}

#[inline]
pub fn write_i64(cursor: &mut Cursor<Vec<u8>>, value: i64) -> Result<()> {
    cursor.write_i64::<BigEndian>(value)
}

#[inline]
pub fn write_u64(cursor: &mut Cursor<Vec<u8>>, value: u64) -> Result<()> {
    cursor.write_u64::<BigEndian>(value)
}

#[inline]
pub fn write_f32(cursor: &mut Cursor<Vec<u8>>, value: f32) -> Result<()> {
    cursor.write_f32::<BigEndian>(value)
}

#[inline]
pub fn write_f64(cursor: &mut Cursor<Vec<u8>>, value: f64) -> Result<()> {
    cursor.write_f64::<BigEndian>(value)
}

#[inline]
pub fn write_string(cursor: &mut Cursor<Vec<u8>>, value: &String) -> Result<()> {
    write_u16(cursor, value.len() as u16)?;
    cursor.write_all(value.as_bytes())
}

#[inline]
pub fn write_vector3(cursor: &mut Cursor<Vec<u8>>, value: &Vector3) -> Result<()> {
    write_f32(cursor, value.x)?;
    write_f32(cursor, value.y)?;
    write_f32(cursor, value.z)
}

#[inline]
pub fn write_iteminstance(cursor: &mut Cursor<Vec<u8>>, value: &ItemInstance) -> Result<()> {
    write_i16(cursor, value.id)?;
    write_i8(cursor, value.count)?;
    write_i16(cursor, value.metadata)
}

#[inline]
pub fn write_iteminstance_list(
    cursor: &mut Cursor<Vec<u8>>,
    value: &Vec<ItemInstance>,
) -> Result<()> {
    write_u16(cursor, value.len() as u16)?;
    for item in value {
        write_iteminstance(cursor, item)?;
    }
    Ok(())
}
