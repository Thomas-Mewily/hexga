
use super::*;

pub struct Io;

impl Io
{
    /// Used for loading and saving
    pub const MARKUP: &'static [&'static str] =
    &[
        Self::RON,

        Self::JSON,

        Self::XML,

        Self::TXT,

        /* Not one of them
        Self::QUICK_BIN,
        */
    ];

    pub const TXT: &'static str = "txt";

    pub const RON  : &'static str = "ron";
    pub const JSON : &'static str = "json";
    pub const XML  : &'static str = "xml";


    /// No garantee about the encoding of this one, can be changed at any time.
    ///
    /// Use it for short duration like data transfer.
    pub const QUICK_BIN  : &'static str = "bin";


    #[cfg(feature = "serde")]
    pub fn save_with_param<T,F,P>(self, value: &T, fs: &mut F, path: P, param: SaveParam) -> IoResult
    where F: FsWrite, P: AsRefPath, T: Serialize + ?Sized
    {
        let path = path.as_ref();
        let mut ser = SerializerSaveTxtOrBinOrMarkup::new(fs, path.to_owned(), param);
        value.serialize(&mut ser)?;
        ser.save()
    }

    // #[cfg(feature = "serde")]
    // pub(crate) fn save_with_param_and_serializer<T,F,P>(self, value: &T, fs: &mut F, path: P, param: SaveParam, serializer: SerializerMarkup) -> IoResult
    // where F: FsWrite, P: AsRefPath, T: Serialize + ?Sized
    // {
    //     let path = path.as_ref();
    //     let mut ser = SerializerSaveTxtOrBinOrMarkup::new_full(fs, path.to_owned(), param, serializer);
    //     value.serialize(&mut ser)?;
    //     ser.save()
    // }
}
