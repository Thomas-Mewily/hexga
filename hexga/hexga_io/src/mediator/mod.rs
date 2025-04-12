use crate::*;

mod direct;
pub use direct::*;

pub type Reason = String;

pub trait IoError 
{
    fn new(err : Reason) -> Self;

    #[allow(unused_variables)]
    fn with_path(self, path : Path) -> Self where Self : Sized { self } 
}

impl IoError for String { fn new(err : Reason) -> Self { err } }
impl IoError for () { fn new(_err : Reason) -> Self { () } }
impl IoError for IoDiskError {
    fn new(err : Reason) -> Self {
        IoDiskError::new(std::io::ErrorKind::Other, err)
    }
}

pub trait IoOk : Default {}
impl IoOk for () {}

pub trait IoMediator
{
    type Ok  : IoOk;
    type Err : IoError;

    fn read_cache(&mut self, path : &path) -> Result<Self::Ok,Self::Err>;
    fn read(&mut self, path : &path) -> Result<Vec<u8>,Self::Err>;

    /// Write the data to the file. Previous data is erased. Will be done when applying at most
    fn write(&mut self, path : &path, data : &[u8]) -> Result<Self::Ok,Self::Err>;
    /// Append some data to the file after the previous data
    fn append(&mut self, path : &path, data : &[u8]) -> Result<Self::Ok,Self::Err>;
    fn remove(&mut self, path : &path) -> Result<Self::Ok,Self::Err>;

    fn apply(&mut self) -> Result<Self::Ok,Self::Err>;
}