use super::*;


pub type IoResult<T=()> = Result<T,IoError>;


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum IoMode { Read, Write }

impl Display for IoMode
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            IoMode::Read => f.write_str("Read"),
            IoMode::Write => f.write_str("Write"),
        }
    }
}

impl IoMode
{
    pub const fn is_read(self) -> bool { matches!(self, Self::Read) }
    pub const fn is_write(self) -> bool { matches!(self, Self::Write) }
}

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct IoError
{
    pub path: Path,
    pub mode: Option<IoMode>,
    pub kind: FileError,
}
impl IoError
{
    pub fn new(path: impl Into<Path>, kind: impl Into<FileError>) -> Self
    {
        Self { path: path.into(), kind: kind.into(), mode: None }
    }
    pub fn with_mode(self, mode: Option<IoMode>) -> Self { Self { mode, ..self }}
    pub fn when_reading(self) -> Self { Self { mode: Some(IoMode::Read), ..self }}
    pub fn when_writing(self) -> Self { Self { mode: Some(IoMode::Write), ..self }}
}
impl Display for IoError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IO error at {} ", self.path)?;
        if let Some(mode) = self.mode
        {
            match mode
            {
                IoMode::Read => write!(f, "when reading")?,
                IoMode::Write => write!(f, "when writing")?,
            }
        }
        write!(f, ": {}", self.kind)
    }
}
impl std::error::Error for IoError
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> { None }
    fn provide<'a>(&'a self, _: &mut std::error::Request<'a>) {}
}

#[cfg(feature = "serde")]
impl serde::ser::Error for IoError
{
    fn custom<T>(msg:T) -> Self where T:std::fmt::Display { Self { kind: FileError::Custom(msg.to_string()), ..Default::default() } }
}
#[cfg(feature = "serde")]
impl serde::de::Error for IoError
{
    fn custom<T>(msg:T) -> Self where T:std::fmt::Display { Self { kind: FileError::Custom(msg.to_string()), ..Default::default() } }
}






// #[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
// pub struct AssetError
// {
//     pub path: Path,
//     pub kind: IoErrorKind,
//     pub childs: Vec<AssetError>
// }

// impl AssetError
// {
//     pub fn new<P>(path: P, kind: IoErrorKind) -> Self where P: AsRefPath { Self { path: path.as_ref().to_owned(), kind, childs: vec![] }}
// }


// // TODO: impl it
// impl Display for AssetError
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
//     {
//         writeln!(f, "Asset error at '{}': {}", self.path, self.kind)?;

//         fn fmt_children(children: &[AssetError], f: &mut std::fmt::Formatter<'_>, indent: usize) -> std::fmt::Result {
//             let indent_str = "  ".repeat(indent);
//             for child in children {
//                 writeln!(f, "{}- {}: {}", indent_str, child.path, child.kind)?;
//                 fmt_children(&child.childs, f, indent + 1)?;
//             }
//             Ok(())
//         }

//         fmt_children(&self.childs, f, 1)
//     }
// }

// // TODO: impl it
// impl std::error::Error for AssetError
// {
//     fn source(&self) -> Option<&(dyn std::error::Error + 'static)>
//     {
//         None
//     }
//     fn provide<'a>(&'a self, _: &mut std::error::Request<'a>) {}
// }

// impl serde::ser::Error for AssetError
// {
//     fn custom<T>(msg:T) -> Self where T:std::fmt::Display
//     {
//         Self { kind: IoErrorKind::custom(msg), ..Default::default() }
//     }
// }