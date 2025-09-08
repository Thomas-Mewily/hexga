
use super::*;

pub struct Io;


impl Io
{
    /// Used for loading and saving
    pub const MARKUP_EXTENSIONS: &'static [&'static str] =
    &[
        #[cfg(feature = "serde_ron")]
        Self::RON_EXTENSION,
        #[cfg(feature = "serde_json")]
        Self::JSON_EXTENSION,
        #[cfg(feature = "serde_xml")]
        Self::XML_EXTENSION,

        /* Not one of them
        #[cfg(feature = "serde_quick_bin")]
        Self::QUICK_BIN_EXTENSION,
        */
    ];

    #[cfg(feature = "serde_json")]
    pub const JSON_EXTENSION : &'static str = "json";
    #[cfg(feature = "serde_ron")]
    pub const RON_EXTENSION  : &'static str = "ron";
    #[cfg(feature = "serde_xml")]
    pub const XML_EXTENSION  : &'static str = "xml";


    #[cfg(feature = "serde_quick_bin")]
    pub const QUICK_BIN_EXTENSION  : &'static str = "bin";
}
