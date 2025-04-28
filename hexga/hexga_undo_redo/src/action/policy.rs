use crate::*;

pub trait Policy : Debug {}

pub struct Try;
pub struct Normal;
pub struct Unsafe;