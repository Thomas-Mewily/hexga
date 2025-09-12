use super::*;


// Float
pub trait ToFloat { fn to_float(self) -> float; }
impl<T> ToFloat for T where T: CastInto<float> { fn to_float(self) -> float { self.cast_into() } }

pub trait ToF32 { fn to_f32(self) -> f32; }
impl<T> ToF32 for T where T: CastInto<f32> { fn to_f32(self) -> f32 { self.cast_into() } }

pub trait ToF64 { fn to_f64(self) -> f64; }
impl<T> ToF64 for T where T: CastInto<f64> { fn to_f64(self) -> f64 { self.cast_into() } }



// iX
pub trait ToInt { fn to_int(self) -> int; }
impl<T> ToInt for T where T: CastInto<int> { fn to_int(self) -> int { self.cast_into() } }

pub trait ToI8 { fn to_i8(self) -> i8; }
impl<T> ToI8 for T where T: CastInto<i8> { fn to_i8(self) -> i8 { self.cast_into() } }

pub trait ToI16 { fn to_i16(self) -> i16; }
impl<T> ToI16 for T where T: CastInto<i16> { fn to_i16(self) -> i16 { self.cast_into() } }

pub trait ToI32 { fn to_i32(self) -> i32; }
impl<T> ToI32 for T where T: CastInto<i32> { fn to_i32(self) -> i32 { self.cast_into() } }

pub trait ToI64 { fn to_i64(self) -> i64; }
impl<T> ToI64 for T where T: CastInto<i64> { fn to_i64(self) -> i64 { self.cast_into() } }

pub trait ToISize { fn to_isize(self) -> isize; }
impl<T> ToISize for T where T: CastInto<isize> { fn to_isize(self) -> isize { self.cast_into() } }


// uX
pub trait ToUInt { fn to_uint(self) -> uint; }
impl<T> ToUInt for T where T: CastInto<uint> { fn to_uint(self) -> uint { self.cast_into() } }

pub trait ToU8 { fn to_u8(self) -> u8; }
impl<T> ToU8 for T where T: CastInto<u8> { fn to_u8(self) -> u8 { self.cast_into() } }

pub trait ToU16 { fn to_u16(self) -> u16; }
impl<T> ToU16 for T where T: CastInto<u16> { fn to_u16(self) -> u16 { self.cast_into() } }

pub trait ToU32 { fn to_u32(self) -> u32; }
impl<T> ToU32 for T where T: CastInto<u32> { fn to_u32(self) -> u32 { self.cast_into() } }

pub trait ToU64 { fn to_u64(self) -> u64; }
impl<T> ToU64 for T where T: CastInto<u64> { fn to_u64(self) -> u64 { self.cast_into() } }

pub trait ToUSize { fn to_usize(self) -> usize; }
impl<T> ToUSize for T where T: CastInto<usize> { fn to_usize(self) -> usize { self.cast_into() } }
