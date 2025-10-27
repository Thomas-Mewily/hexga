use super::*;


pub type IoResult<T=()> = Result<T,IoError>;



#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum IoMode{Read,Write}

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

// TODO: impl it
#[derive(Default, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IoError
{
    #[default]
    Unknow,
    Unimplemented,
    NotFound,
    Custom(String),
    Std(std::io::ErrorKind),
    Utf8Error { valid_up_to : usize, error_len : Option<usize>},
    Markup { mode: IoMode, typename: String, extension: Extension, reason: String},
    UnsupportedExtension { mode: IoMode, typename : String, got : Extension, expected : Vec<Extension> },
    MissingBase { mode: IoMode, typename_source : String, typename_dest : String },
    Encoding { reason: String }
}

impl IoError
{
    pub fn encoding(reason: String) -> Self { Self::Encoding { reason }}

    pub(crate) fn markup_serializer  <T: ?Sized>(ext : &extension, err : impl std::fmt::Debug) -> Self { IoError::Markup{ mode:IoMode::Write, typename: std::any::type_name::<T>().to_owned(), extension: ext.to_owned(), reason: format!("{:?}", err) } }
    pub(crate) fn markup_deserializer<T: ?Sized>(ext : &extension, err : impl std::fmt::Debug) -> Self { IoError::Markup{ mode:IoMode::Read, typename: std::any::type_name::<T>().to_owned(), extension: ext.to_owned(), reason: format!("{:?}", err) } }
}

impl From<FromUtf8Error> for IoError
{
    fn from(value: FromUtf8Error) -> Self
    {
        value.utf8_error().into()
    }
}

impl From<Utf8Error> for IoError
{
    fn from(value: Utf8Error) -> Self
    {
        Self::Utf8Error { valid_up_to: value.valid_up_to(), error_len: value.error_len() }
    }
}

impl From<std::io::Error> for IoError
{
    fn from(value: std::io::Error) -> Self {
        value.kind().into()
    }
}

impl From<std::io::ErrorKind> for IoError
{
    fn from(kind: std::io::ErrorKind) -> Self {
        Self::Std(kind)
    }
}



impl Display for IoError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self
        {
            IoError::Unknow => f.write_str("unknow"),
            IoError::Unimplemented => f.write_str("unimplemented"),
            IoError::NotFound  => f.write_str("not found"),
            IoError::Custom(reason) => write!(f, "custom: {reason}"),
            IoError::Std(kind) => write!(f, "std: {kind}"),
            IoError::Utf8Error { valid_up_to, error_len }  =>
            {
                // Copied from std Utf8Error
                if let Some(error_len) = error_len {
                    write!(
                        f,
                        "invalid utf-8 sequence of {} bytes from index {}",
                        error_len, valid_up_to
                    )
                } else {
                    write!(f, "incomplete utf-8 byte sequence from index {}", valid_up_to)
                }
            },
            IoError::Markup { mode, typename, extension, reason } => {
                write!(
                    f,
                    "markup error in mode '{}' for type '{}' (extension '{}'): {}",
                    mode, typename, extension, reason
                )
            },
            IoError::UnsupportedExtension { mode, typename, got, expected } =>
            {
                write!(
                    f,
                    "unsupported extension in mode '{}' for type '{}': got '{}', expected one of {:?}",
                    mode, typename, got, expected
                )
            },
            IoError::MissingBase { mode, typename_source, typename_dest } =>
            {
                write!(
                    f,
                    "missing base type in mode '{}': cannot convert from '{}' to '{}'",
                    mode, typename_source, typename_dest
                )
            },
            IoError::Encoding { reason } => write!(f, "encoding error: {}", reason),
        }
    }
}

// Todo: impl it
impl std::error::Error for IoError
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)>
    {
        None
    }
    fn provide<'a>(&'a self, _: &mut std::error::Request<'a>) {}
}

impl serde::ser::Error for IoError
{
    fn custom<T>(msg:T) -> Self where T:std::fmt::Display
    {
        Self::Custom(msg.to_string())
    }
}





#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct AssetError
{
    pub path: Path,
    pub kind: IoError,
    pub childs: Vec<AssetError>
}

impl AssetError
{
    pub fn new<P>(path: P, kind: IoError) -> Self where P: AsRefPath { Self { path: path.as_ref().to_owned(), kind, childs: vec![] }}
}


// TODO: impl it
impl Display for AssetError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        writeln!(f, "Asset error at '{}': {}", self.path, self.kind)?;

        fn fmt_children(children: &[AssetError], f: &mut std::fmt::Formatter<'_>, indent: usize) -> std::fmt::Result {
            let indent_str = "  ".repeat(indent);
            for child in children {
                writeln!(f, "{}- {}: {}", indent_str, child.path, child.kind)?;
                fmt_children(&child.childs, f, indent + 1)?;
            }
            Ok(())
        }

        fmt_children(&self.childs, f, 1)
    }
}

// TODO: impl it
impl std::error::Error for AssetError
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)>
    {
        None
    }
    fn provide<'a>(&'a self, _: &mut std::error::Request<'a>) {}
}

impl serde::ser::Error for AssetError
{
    fn custom<T>(msg:T) -> Self where T:std::fmt::Display
    {
        Self { kind: IoError::custom(msg), ..Default::default() }
    }
}