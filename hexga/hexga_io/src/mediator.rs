use std::{collections::HashMap, ops::{Deref, DerefMut}};
use std::io::Write;

use crate::*;


#[allow(unused_variables)]
pub trait IoSave where Self: Sized + Serialize + for<'de> Deserialize<'de>
{
    /// Set this to () if you are not based on another type
    /// 
    /// Useful if the current type is similar to another savable type
    /// ex : `Image` and `Texture`
    type IoBasedOn : IoSave;

    /// Dedicated file extension to load the value. ex `png`, `jpeg` for image
    /// 
    /// Don't include the markup language extension like `json` or `ron`
    fn open_file_custom_extension() -> impl Iterator<Item=&'static str> { Self::IoBasedOn::open_file_custom_extension() }
    /// Dedicated file extension to load the value. ex `png`, `jpeg` for image
    /// 
    /// Don't include the markup language extension like `json` or `ron`
    fn can_open_custom_extension(extension : &str) -> bool { Self::open_file_custom_extension().position(|e| e == extension).is_some() }

    /// Also include the markup language extension like `json` or `ron`
    fn open_file_extension() -> impl Iterator<Item=&'static str> { Self::open_file_custom_extension().chain(Io::ALL_MARKUP_LANGAGE_EXTENSION.iter().cloned()) }
    /// Also include the markup language extension like `json` or `ron`
    fn can_open_extension(extension : &str) -> bool { Self::open_file_extension().position(|e| e == extension).is_some() }

    fn from_bytes_with_extension(bytes : &[u8], extension : &str) -> IoResult<Self> 
    {
        match extension
        {
            #[cfg(feature = "serde_ron")]
            Io::RON_EXTENSION => Self::from_ron_buf(bytes),

            #[cfg(feature = "serde_json")]
            Io::JSON_EXTENSION => Self::from_json_buf(bytes),

            #[cfg(feature = "serde_xml")]
            Io::XML_EXTENSION => Self::from_xml_buf(bytes),

            #[cfg(feature = "serde_quick_bin")]
            Io::QUICK_BIN_EXTENSION => Self::from_quick_bin_buf(bytes),
            
            _ => match Self::IoBasedOn::from_bytes_with_extension(bytes, extension)
            {
                Ok(base) => Self::from_io_based_on(base),
                Err(_) => match Self::can_open_custom_extension(extension)
                {
                    true => Self::from_bytes_with_custom_extension(bytes, extension),
                    false => Err(IoError::new_unsuported_extension::<Self>(extension)),
                },
            }
        }
    }
    fn from_bytes_with_custom_extension(bytes : &[u8], extension : &str) -> IoResult<Self> 
    {
        Err(IoError::default())
    }

    fn from_io_based_on(base : Self::IoBasedOn) -> IoResult<Self> { Err(IoError::___()) }




    
    /// Dedicated file extension to load the value. ex `png`, `jpeg` for image
    /// 
    /// Don't include the markup language extension like `json` or `ron`
    fn save_file_custom_extension() -> impl Iterator<Item=&'static str> { Self::IoBasedOn::save_file_custom_extension() }
    /// Don't include the markup language extension like `json` or `ron`
    fn can_save_custom_extension(extension : &str) -> bool { Self::save_file_custom_extension().position(|e| e == extension).is_some() }

    /// Also include the markup language extension like `json` or `ron`
    fn save_file_extension() -> impl Iterator<Item=&'static str> { Self::save_file_custom_extension().chain(Io::ALL_MARKUP_LANGAGE_EXTENSION.iter().cloned()) }
    /// Also include the markup language extension like `json` or `ron`
    fn can_save_extension(extension : &str) -> bool { Self::save_file_extension().position(|e| e == extension).is_some() }

    fn save_bytes_with_extension<'a, W : Write>(&self, bytes : &mut W, extension : &extension) -> IoResult
    {
        match extension.is_markup_extension()
        {
            true => 
            {
                let str = match extension
                {
                    #[cfg(feature = "serde_ron")]
                    Io::RON_EXTENSION  => self.to_ron()?,

                    #[cfg(feature = "serde_json")]
                    Io::JSON_EXTENSION =>  self.to_json()?,
                    
                    #[cfg(feature = "serde_xml")]
                    Io::XML_EXTENSION => self.to_xml()?,
                    _ => unreachable!(),
                };

                write!(bytes, "{}", str).map_err(|e| IoError::new_serialize::<Self>(extension, e.to_debug()))
            },
            false => match self.io_get_based_on()
            {
                Some(v) => v.save_bytes_with_extension(bytes, extension),
                None => match self.io_get_based_on_by_value()
                {
                    Some(v) => v.save_bytes_with_extension(bytes, extension),

                    None => match Self::can_save_extension(extension)
                    {
                        true => self.save_bytes_with_custom_extension(bytes, extension),
                        false => Err(IoError::new_serialize::<Self>(extension, "Dev impl".to_owned())),
                    },
                }
            },
        }
    }

    fn save_bytes_with_custom_extension<'a, W: Write>(&self, bytes : &mut W, extension : &str) -> IoResult
    {
        Err(IoError::new_serialize::<Self>(extension, "Dev impl".to_owned()))
    }

    fn io_get_based_on_by_value(&self) -> Option<Self::IoBasedOn> { None }
    fn io_get_based_on(&self) -> Option<&Self::IoBasedOn> { None }

    /* 
    fn save(&self, path : &path, mediator : &mut impl IoMediator) -> IoResult { self.save_bytes(path).and_then(|bytes| mediator.save_bytes(bytes, path)) }
    fn save_bytes(&self, extension : &extension) -> IoResult<Vec<u8>>;
    */

}

impl IoSave for Never 
{
    type IoBasedOn = Never;

    fn open_file_custom_extension() -> impl Iterator<Item=&'static str> { std::iter::empty() }
    fn save_file_custom_extension() -> impl Iterator<Item=&'static str> { std::iter::empty() }

    fn from_bytes_with_extension(_ : &[u8], _ : &str) -> IoResult<Self> { Err(___()) }
    fn from_bytes_with_custom_extension(_ : &[u8], _ : &str) -> IoResult<Self> { Err(___()) }

    fn save_bytes_with_extension<'a, W : Write>(&self, _ : &mut W, _ : &extension) -> IoResult { Err(___()) }
    fn save_bytes_with_custom_extension<'a, W: Write>(&self, _ : &mut W, _ : &str) -> IoResult { Err(___()) }
}

impl IoSave for bool 
{
    type IoBasedOn = Never;
}

pub struct IoNodeRoot
{
    pub root : IoNode,
}
impl Deref for IoNodeRoot { type Target = IoNode; fn deref(&self) -> &Self::Target { &self.root } }
impl DerefMut for IoNodeRoot { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.root } }

pub struct IoNode
{
    pub name     : Path,
    pub content  : Vec<u8>,
    pub child    : Vec<IoNode>,
}

pub trait IoMediator
{
    fn save_bytes(&mut self, bytes : Vec<u8>, path : &extension) -> IoResult;
    fn load(&mut self, path : &extension) -> IoResult;
    fn apply(&mut self) -> IoResult<IoNodeRoot>;
}

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