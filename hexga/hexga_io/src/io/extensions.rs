
use super::*;

pub struct Extensions;

impl Extensions
{
    /// Used for loading and saving
    pub const MARKUP: &'static [&'static str] =
    &[
        #[cfg(feature = "serde_ron")]
        Self::RON,

        #[cfg(feature = "serde_json")]
        Self::JSON,

        #[cfg(feature = "serde_xml")]
        Self::XML,

        Self::TXT,

        /* Not one of them
        #[cfg(feature = "serde_quick_bin")]
        Self::QUICK_BIN,
        */
    ];

    pub const TXT: &'static str = "txt";

    pub const RON  : &'static str = "ron";
    #[cfg(feature = "serde_json")]
    pub const JSON : &'static str = "json";
    #[cfg(feature = "serde_xml")]
    pub const XML  : &'static str = "xml";


    /// No garantee about the encoding of this one, can be changed at any time.
    ///
    /// Use it for short duration like data transfer.
    #[cfg(feature = "serde_quick_bin")]
    pub const QUICK_BIN  : &'static str = "bin";
}
