//! Original file: doomtype.h
//! 
//! Description: Simple basic typedefs, isolated here to make it
//! easier separating modules.

#![allow(overflowing_literals)]

pub type Byte = u8;

pub const MAX_CHAR: i8 = 0x7F;
pub const MAX_SHORT: i16 = 0x7FFF;

// Max pos 32-bit int.
pub const MAX_INT: i32 = 0x7FFFFFFF;
pub const MAX_LONG: i32 = 0x7FFFFFFF;
pub const MIN_CHAR: i8 = 0x80;
pub const MIN_SHORT: i16 = 0x8000;

// Max negative 32-bit integer.
pub const MIN_INT: i32 = 0x80000000;
pub const MIN_LONG: i32 = 0x80000000;
