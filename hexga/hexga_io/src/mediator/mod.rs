use crate::*;

mod direct;
pub use direct::*;

pub trait IoError {}
impl IoError for () {}
impl IoError for IoDiskError {}

pub trait IoOk : Default {}
impl IoOk for () {}

pub trait IoMediator
{
    type Ok  : IoOk;
    type Err : IoError;

    fn read_cache(&mut self, path : &str) -> Result<Self::Ok,Self::Err>;
    fn read(&mut self, path : &str) -> Result<Vec<u8>,Self::Err>;

    /// Write the data to the file. Previous data is erased. Will be done when applying at most
    fn write(&mut self, path : &str, data : &[u8]) -> Result<Self::Ok,Self::Err>;
    /// Append some data to the file after the previous data
    fn append(&mut self, path : &str, data : &[u8]) -> Result<Self::Ok,Self::Err>;
    fn remove(&mut self, path : &str) -> Result<Self::Ok,Self::Err>;

    fn apply(&mut self) -> Result<Self::Ok,Self::Err>;
}