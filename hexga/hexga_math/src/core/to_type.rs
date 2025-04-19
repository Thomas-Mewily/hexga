use crate::*;

// Coef
pub trait ToCoef
{ 
    type Output;
    fn to_coef(self) -> Self::Output; 
}
impl<T> ToCoef for T where T : CastIntoComposite<CoefWrapper>
{
    type Output=T::Output;
    fn to_coef(self) -> Self::Output { self.cast_into_composite() }
}

pub trait FromCoef
{ 
    fn from_coef(coef : Coef) -> Self;
}
impl<T> FromCoef for T where T : DefaultRange + ToFloat<Output = float>, float : CastInto<T>
{
    fn from_coef(coef : Coef) -> Self { (Self::RANGE.to_float() * coef + Self::MIN_RANGE.to_float()).cast_into_composite() }
}

// Float
pub trait ToFloat
{ 
    type Output;
    fn to_float(self) -> Self::Output; 
}
impl<T> ToFloat for T where T : CastIntoComposite<float>
{
    type Output=T::Output;
    fn to_float(self) -> Self::Output { self.cast_into_composite() }
}

pub trait ToF32
{ 
    type Output;
    fn to_f32(self) -> Self::Output; 
}
impl<T> ToF32 for T where T : CastIntoComposite<f32>
{
    type Output=T::Output;
    fn to_f32(self) -> Self::Output { self.cast_into_composite() }
}

pub trait ToF64
{ 
    type Output;
    fn to_f64(self) -> Self::Output; 
}
impl<T> ToF64 for T where T : CastIntoComposite<f64>
{
    type Output=T::Output;
    fn to_f64(self) -> Self::Output { self.cast_into_composite() }
}


// iX
pub trait ToInt  
{ 
    type Output;
    fn to_int(self) -> Self::Output; 
}
impl<T> ToInt for T where T : CastIntoComposite<int>
{
    type Output=T::Output;
    fn to_int(self) -> Self::Output { self.cast_into_composite() }
}

pub trait ToI8  
{ 
    type Output;
    fn to_i8(self) -> Self::Output; 
}
impl<T> ToI8 for T where T : CastIntoComposite<i8>
{
    type Output=T::Output;
    fn to_i8(self) -> Self::Output { self.cast_into_composite() }
}

pub trait ToI16  
{ 
    type Output;
    fn to_i16(self) -> Self::Output; 
}
impl<T> ToI16 for T where T : CastIntoComposite<i16>
{
    type Output=T::Output;
    fn to_i16(self) -> Self::Output { self.cast_into_composite() }
}

pub trait ToI32  
{ 
    type Output;
    fn to_i32(self) -> Self::Output; 
}
impl<T> ToI32 for T where T : CastIntoComposite<i32>
{
    type Output=T::Output;
    fn to_i32(self) -> Self::Output { self.cast_into_composite() }
}

pub trait ToI64  
{ 
    type Output;
    fn to_i64(self) -> Self::Output; 
}
impl<T> ToI64 for T where T : CastIntoComposite<i64>
{
    type Output=T::Output;
    fn to_i64(self) -> Self::Output { self.cast_into_composite() }
}

pub trait ToISize  
{ 
    type Output;
    fn to_isize(self) -> Self::Output; 
}
impl<T> ToISize for T where T : CastIntoComposite<isize>
{
    type Output=T::Output;
    fn to_isize(self) -> Self::Output { self.cast_into_composite() }
}

// uX
pub trait ToUInt  
{ 
    type Output;
    fn to_uint(self) -> Self::Output; 
}
impl<T> ToUInt for T where T : CastIntoComposite<uint>
{
    type Output=T::Output;
    fn to_uint(self) -> Self::Output { self.cast_into_composite() }
}

pub trait ToU8  
{ 
    type Output;
    fn to_u8(self) -> Self::Output; 
}
impl<T> ToU8 for T where T : CastIntoComposite<u8>
{
    type Output=T::Output;
    fn to_u8(self) -> Self::Output { self.cast_into_composite() }
}

pub trait ToU16  
{ 
    type Output;
    fn to_u16(self) -> Self::Output; 
}
impl<T> ToU16 for T where T : CastIntoComposite<u16>
{
    type Output=T::Output;
    fn to_u16(self) -> Self::Output { self.cast_into_composite() }
}

pub trait ToU32  
{ 
    type Output;
    fn to_u32(self) -> Self::Output; 
}
impl<T> ToU32 for T where T : CastIntoComposite<u32>
{
    type Output=T::Output;
    fn to_u32(self) -> Self::Output { self.cast_into_composite() }
}

pub trait ToU64  
{ 
    type Output;
    fn to_u64(self) -> Self::Output; 
}
impl<T> ToU64 for T where T : CastIntoComposite<u64>
{
    type Output=T::Output;
    fn to_u64(self) -> Self::Output { self.cast_into_composite() }
}

pub trait ToUSize  
{ 
    type Output;
    fn to_usize(self) -> Self::Output; 
}
impl<T> ToUSize for T where T : CastIntoComposite<usize>
{
    type Output=T::Output;
    fn to_usize(self) -> Self::Output { self.cast_into_composite() }
}