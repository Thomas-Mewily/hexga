use std::{collections::HashMap, ops::{Deref, DerefMut}};

use crate::*;


/* 
pub trait IoError
{
    fn to_io_err(err : Reason) -> Self;

    fn to_io_err_with_path(path : Path, err : Reason) -> Self;
    #[allow(unused_variables)]
    fn io_err_with_path(self, path : Path) -> Self where Self : Sized { self } 
}

impl IoError for String 
{ 
    fn to_io_err(err : Reason) -> Self { err }
    
    fn to_io_err_with_path(path : Path, err : Reason) -> Self {
        format!("{path} : {err}")
    } 
}
impl IoError for () 
{ 
    fn to_io_err(_err : Reason) -> Self { () }
    fn to_io_err_with_path(path : Path, err : Reason) -> Self { () } 
}

impl IoError for IoDiskError 
{
    fn to_io_err(err : Reason) -> Self {
        IoDiskError::new(std::io::ErrorKind::Other, err)
    }
    
    fn to_io_err_with_path(path : Path, err : Reason) -> Self {
        IoDiskError::new(std::io::ErrorKind::Other, String::to_io_err_with_path(path, err))
    }
}

pub trait IoOk : Default {}
impl IoOk for () {}


#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct IoCache
{
    // root : Path ?
    node : IoNode,
}
impl Deref for IoCache { type Target=IoNode; fn deref(&self) -> &Self::Target { &self.node } }
impl DerefMut for IoCache { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.node } }

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct IoNode
{
    kind : IoNodeKind,
    /// If a file `x.png` have some childrens,
    /// it will be moved inside a module `x/mod.png`
    /// `mod` is a reserved name for file and folder
    children : HashMap<String, IoNode>,
    mode : IoNodeMode,
}

impl IoNode
{
    fn get_child_mut<'a>(&'a mut self, path : &'a[String]) -> Option<(&'a mut IoNode, &'a [String])>
    {
        let name = path.first()?;
        self.children.get_mut(name).map(|v| (v, &path[1..]))
    }

    fn read(mut self, path : &[String]) -> IoCache
    {
        if self.mode.is_unedited()
        {
            self.mode = IoNodeMode::Read;
        }else
        {
            return Err(IoError::new(IoErrorKind::InvalidInput, "node can't be read".to_owned()));
        }
        let Some((node, path)) = self.get_child_mut(path) else { return Ok(self); };
        node.read(path).map(|v| { self.children = v; self })
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, Default)]
pub enum IoNodeMode
{
    #[default]
    Unedited,
    Read,
    Write,
    Delete,
}

impl IoNodeMode
{
    pub fn is_unedited(self) -> bool { matches!(self, Self::Unedited) }
}

impl IoNode
{
    pub fn new() -> Self { Self::default() }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IoNodeKind
{
    File(IoFile),
    Folder(IoFolder),
}
impl Default for IoNodeKind
{
    fn default() -> Self {
        Self::Folder(IoFolder)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct IoFile
{
    data : Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct IoFolder;

//pub type IoCache = Result<IoNode,IoError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IoMediator<Ctx>
{
    context : Ctx,
    state   : IoCache,
    read    : fn(&path, &mut Vec<u8>, &mut Ctx) -> IoResult,
    write   : fn(&path, &[u8], &mut Ctx) -> IoResult,
    remove  : fn(&path, &mut Ctx) -> IoResult,
}

impl Default for IoMediator<()>
{
    fn default() -> Self 
    {
        Self::new((), 
            |path,buf,_| Io::disk_read_buf(path, buf), 
            |path,data,_| Io::disk_write(path, data), 
            |path,_| Io::disk_remove(path),
        )
    }
}

impl<Ctx> IoMediator<Ctx>
{
    pub fn new(
        context: Ctx,
        read: fn(&path, &mut Vec<u8>, &mut Ctx) -> IoResult,
        write: fn(&path, &[u8], &mut Ctx) -> IoResult,
        remove: fn(&path, &mut Ctx) -> IoResult,
    ) -> Self 
    {
        Self {
            context,
            state: Ok(IoNode::new()),
            read,
            write,
            remove,
        }
    }

    //fn read_buf(&mut self, path : &path) -> Result<Self::Ok,Self::Err>;
    //fn read(&mut self, path : &path) -> Result<Vec<u8>,Self::Err>;
    fn read(&mut self, path : &path) -> bool
    {
        if self.state.is_err() { return false; }

        let path = path.path_split();

        self.state.unwrap().read(path)
        self.state.read(&path);
    }

    /// Write the data to the file. Previous data is erased. Will be done when applying at most
    fn write(&mut self, path : &path, writer : impl FnOnce(&mut Vec<u8>));

    // Append some data to the file after the previous data
    // fn append(&mut self, path : &path, data : &[u8]) -> Result<Self::Ok,Self::Err>;
    
    fn remove(&mut self, path : &path);

    fn apply(&mut self) -> Result<IoNode,IoError>;
}
*/