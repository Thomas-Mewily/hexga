#![feature(new_range_api)]

use hexga_number::*;

pub mod prelude
{
    pub use super::*;
}

/// Not approved for Cryptography use
pub trait RandomSource
{
    fn next_u32(&mut self) -> u32 { self.next_u64() as _ }
    fn next_u64(&mut self) -> u64;
}
pub trait RandomSourceExtension: RandomSource
{
    fn random<T>(&mut self) -> T where T: FromRandomSource { T::rand(self) }

    /// Return a random sign: `-1` or `1`
    fn sign<T : One + MinusOne>(&mut self) -> T where { if self.random() { T::ONE } else { T::MINUS_ONE } }

    /// Return a random unsigned sign: `0` or `1`
    fn usign<T : One + Zero>(&mut self) -> T  {  if self.random() { T::ONE } else { T::ZERO } }
}
impl<R: RandomSource +?Sized> RandomSourceExtension for R {}

pub trait FromRandomSource { fn rand<R: RandomSource +?Sized>(r : &mut R) -> Self; }

impl FromRandomSource for bool  { fn rand<R: RandomSource +?Sized>(r : &mut R) -> Self { r.next_u32() % 2 == 0 }}
impl FromRandomSource for u64   { fn rand<R: RandomSource +?Sized>(r : &mut R) -> Self { r.next_u64() }}
impl FromRandomSource for u32   { fn rand<R: RandomSource +?Sized>(r : &mut R) -> Self { r.next_u32() }}
impl FromRandomSource for u16   { fn rand<R: RandomSource +?Sized>(r : &mut R) -> Self { r.next_u32() as _ }}
impl FromRandomSource for u8    { fn rand<R: RandomSource +?Sized>(r : &mut R) -> Self { r.next_u32() as _ }}

impl FromRandomSource for i64   { fn rand<R: RandomSource +?Sized>(r : &mut R) -> Self { i64::from_ne_bytes(r.next_u64().to_ne_bytes()) }}
impl FromRandomSource for i32   { fn rand<R: RandomSource +?Sized>(r : &mut R) -> Self { i64::rand(r) as i32 }}
impl FromRandomSource for i16   { fn rand<R: RandomSource +?Sized>(r : &mut R) -> Self { i64::rand(r) as i16 }}
impl FromRandomSource for i8    { fn rand<R: RandomSource +?Sized>(r : &mut R) -> Self { i64::rand(r) as i8 }}

impl FromRandomSource for usize  { fn rand<R: RandomSource +?Sized>(r : &mut R) -> Self { r.next_u64() as usize }}
impl FromRandomSource for isize  { fn rand<R: RandomSource +?Sized>(r : &mut R) -> Self { isize::from_ne_bytes(usize::rand(r).to_ne_bytes()) }}

impl<T, const N:usize> FromRandomSource for [T;N] where T: FromRandomSource
{
    fn rand<R: RandomSource +?Sized>(r : &mut R) -> Self {
        std::array::from_fn(|_| r.random())
    }
}


pub trait RandomInside
{
    type Output;
    fn random_in<R: RandomSource>(&self, r : &mut R) -> Option<Self::Output>;
}
impl<T> RandomInside for std::range::Range<T> where T:Number + Abs<Output=T> + FromRandomSource
{
    type Output=T;
    fn random_in<R: RandomSource>(&self, r :  &mut R) -> Option<Self::Output> 
    {
        if self.start >= self.end { return None; }
        let len = self.end - self.start;
        // TODO : won't work with signed value
        Some(self.start + r.random::<T>().abs() % len)
    }
}