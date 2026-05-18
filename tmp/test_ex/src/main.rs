#![allow(unused)]
use std::{
    num::NonZero,
    ops::{Add, DerefMut},
};

fn add_10<T>(val: T) -> T
where
    T: From<u8> + Add<Output = T>,
{
    val + 10.into()
}

#[repr(C)]
struct Foo
{
    a: u8,
    b: u16,
    c: u8,
}

type CustomError = ();

trait ComputeHash<const N: usize>
{
    fn compute_hash(&self) -> Result<[u8; N], CustomError>;
}

use std::fmt::Debug;

trait ComputeHash2
{
    type Hash: DerefMut<Target = u8>;
    type Error: Debug;
    fn compute_hash(&self) -> Result<Self::Hash, Self::Error>;
}

fn main()
{
    dbg!(add_10(8));
    dbg!(add_10(8.54));

    dbg!(std::mem::size_of::<Option<u32>>());
    dbg!(std::mem::size_of::<Option<NonZero<u32>>>());
    dbg!(std::mem::size_of::<
        Option<Option<Option<Option<Option<Option<Option<Option<Option<Option<Option<Option<u32>>>>>>>>>>>>,
    >());
}
