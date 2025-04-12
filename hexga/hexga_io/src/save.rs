use crate::*;

impl Io
{
    /// Used for loading and saving
    pub const MARKUP_LANGAGE_EXTENSION : [&'static str; 5] =
    [
        "ron",
        "json",
        "xml",
        "yaml", "yml",
    ];
}

// An abstraction over Serde tat support custom file format
pub trait IoSave<M> where Self: Sized + Serialize + for<'de> Deserialize<'de>, M : IoMediator
{
    /// Set this to Never if you are not based on another type
    /// 
    /// Useful if the current type is similar to another savable type
    /// ex : `Image` and `Texture`
    type BasedOn : IoSave<M>; // = Never;

    /// Dedicated file extension to load the value. ex `png`, `jpeg` for image
    /// 
    /// Don't include the markup language extension like `json` or `ron`
    fn io_open_own_extension() -> impl Iterator<Item=&'static str> { Self::BasedOn::io_open_own_extension() }

    /// Dedicated file extension to load the value. ex `png`, `jpeg` for image
    /// 
    /// Don't include the markup language extension like `json` or `ron`
    fn io_can_open_own_extension(extension : &str) -> bool { Self::io_open_own_extension().position(|e| e == extension).is_some() }

    /// Also include the markup language extension like `json` or `ron`
    fn io_open_extension() -> impl Iterator<Item=&'static str> { Self::io_open_own_extension().chain(Io::MARKUP_LANGAGE_EXTENSION) }
    /// Also include the markup language extension like `json` or `ron`
    fn io_can_open_extension(extension : &str) -> bool { Self::io_open_extension().position(|e| e == extension).is_some() }

}