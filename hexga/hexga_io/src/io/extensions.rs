
use super::*;

pub struct Io;

impl Io
{
    /// Used for loading and saving
    pub const MARKUP: &'static [&'static str] =
    &[
        Self::RON,

        Self::JSON,

        // Self::XML,

        Self::TXT,

        /* Not one of them
        Self::QUICK_BIN,
        */
    ];

    pub const TXT: &'static str = "txt";

    pub const RON  : &'static str = "ron";
    pub const JSON : &'static str = "json";
    // pub const XML  : &'static str = "xml";


    /// No garantee about the encoding of this one, can be changed at any time.
    ///
    /// Use it for short duration like data transfer.
    pub const BIN  : &'static str = "bin";
}
