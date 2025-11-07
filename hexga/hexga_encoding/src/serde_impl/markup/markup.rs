use super::*;


/// /!\ The number of genenric parameter may vary at any time. Avoid using it for stability.
// #[derive(Serialize, Deserialize)]
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub enum Markup<Ron,Json,Xml>
// {
//     Ron(Ron),
//     Json(Json),
//     Xml(Xml),
// }

/// Markup Langaga capable of serialize / deserializer almost everythings
#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FormatMarkup
{
    // TODO: change it to Ron by default when ron Serializer::into_inner method will be added https://github.com/ron-rs/ron/pull/588
    #[default]
    Ron,
    Json,
    Xml,
}


impl FormatMarkup
{
    // Todo: add a flag
    pub const PREFERED : Self=Self::Ron;
    pub const ALL: &'static [Self] = &[Self::Ron, Self::Json, Self::Xml];

    pub const fn extension(self) -> &'static str
    {
        match self
        {
            FormatMarkup::Ron => Extension::RON,
            FormatMarkup::Json => Extension::JSON,
            FormatMarkup::Xml => Extension::XML,
        }
    }

    pub fn encode<T>(self, value: &T) -> EncodeResult<String>
        where T: Serialize
    {
        match self
        {
            FormatMarkup::Ron => value.to_ron(),
            FormatMarkup::Json => value.to_json(),
            FormatMarkup::Xml => value.to_xml(),
        }
    }

    pub fn encode_with_writer<T, W>(self, value: &T, writer: W) -> EncodeResult
        where T: Serialize, W: Write
    {
        match self
        {
            FormatMarkup::Ron => value.to_ron_with_writer(writer),
            FormatMarkup::Json => value.to_json_with_writer(writer),
            FormatMarkup::Xml => value.to_xml_with_writer(writer),
        }
    }

    pub fn from_str<T>(self, markup: &str) -> EncodeResult<T>
        where T: for<'de> Deserialize<'de>
    {
        match self
        {
            FormatMarkup::Ron => T::from_ron(markup),
            FormatMarkup::Json => T::from_json(markup),
            FormatMarkup::Xml => T::from_xml(markup),
        }
    }

    pub fn from_bytes<T>(self, bytes: &[u8]) -> EncodeResult<T>
        where
        T: for<'de> Deserialize<'de>
    {
        match self
        {
            FormatMarkup::Ron => T::from_ron_bytes(bytes),
            FormatMarkup::Json => T::from_json_bytes(bytes),
            FormatMarkup::Xml => T::from_xml_bytes(bytes),
        }
    }

    pub fn from_reader<T, R>(self, reader: R) -> EncodeResult<T>
        where
        T: for<'de> Deserialize<'de>, R: Read
    {
        match self
        {
            FormatMarkup::Ron => T::from_ron_with_reader(reader),
            FormatMarkup::Json => T::from_json_with_reader(reader),
            FormatMarkup::Xml => T::from_xml_with_reader(reader),
        }
    }
}
impl Display for FormatMarkup
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl<'a> TryFrom<&'a str> for FormatMarkup
{
    type Error=();
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value
        {
            Extension::RON => Ok(Self::Ron),
            Extension::JSON => Ok(Self::Json),
            Extension::XML => Ok(Self::Xml),
            _ => Err(())
        }
    }
}







#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FormatSpecial
{
    Txt,
    TmpBin,
}


impl FormatSpecial
{
    pub const ALL: &'static [Self] = &[Self::Txt, Self::TmpBin];

    pub const fn extension(self) -> &'static str
    {
        match self
        {
            FormatSpecial::Txt => Extension::TXT,
            FormatSpecial::TmpBin => Extension::TMP_BIN,
        }
    }

    pub fn encode<T>(self, value: &T) -> EncodeResult<Vec<u8>>
        where T: Serialize
    {
        match self
        {
            FormatSpecial::Txt =>
            {
                let mut txt = String::with_capacity(1024);
                value.serialize(SerializerTxt::new(&mut txt))?;
                Ok(txt.into_bytes())
            },
            FormatSpecial::TmpBin =>
            {
                let mut bytes = Vec::with_capacity(256);
                value.serialize(SerializerTmpBin::new(&mut bytes))?;
                Ok(bytes)
            },
        }
    }

    pub fn encode_with_writer<T, W>(self, value: &T, writer: W) -> EncodeResult
        where T: Serialize, W: Write
    {
        match self
        {
            FormatSpecial::Txt =>
            {
                value.serialize(SerializerTxt::new(writer.to_fmt_writer()))?;
                Ok(())
            },
            FormatSpecial::TmpBin =>
            {
                value.serialize(SerializerTmpBin::new(writer))?;
                Ok(())
            },
        }
    }

    pub fn from_bytes<T>(self, bytes: &[u8]) -> EncodeResult<T>
        where T: for<'de> Deserialize<'de>
    {
        match self
        {
            FormatSpecial::Txt => T::deserialize(DeserializerTxt::new(str::from_utf8(bytes)?)),
            FormatSpecial::TmpBin => T::deserialize(DeserializerTmpBin::new(bytes)),
        }
    }

    pub fn from_reader<T, R>(self, mut reader: R) -> EncodeResult<T>
        where T: for<'de> Deserialize<'de>, R: Read
    {
        match self
        {
            FormatSpecial::Txt =>
            {
                let mut txt = String::with_capacity(1024);
                reader.read_to_string(&mut txt)?;
                T::deserialize(DeserializerTxt::new(txt))
            },
            FormatSpecial::TmpBin =>
            {
                let mut bytes = Vec::with_capacity(256);
                reader.read_to_end(&mut bytes)?;
                T::deserialize(DeserializerTmpBin::new(bytes))
            }
        }
    }
}
impl Display for FormatSpecial
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl<'a> TryFrom<&'a str> for FormatSpecial
{
    type Error=();
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value
        {
            Extension::TXT => Ok(Self::Txt),
            Extension::TMP_BIN => Ok(Self::TmpBin),
            _ => Err(())
        }
    }
}



#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnyFormat
{
    Markup(FormatMarkup),
    Special(FormatSpecial),
}
impl Default for AnyFormat
{
    fn default() -> Self {
        Self::Markup(FormatMarkup::default())
    }
}
impl AnyFormat
{
    pub const PREFERED : Self = Self::Markup(FormatMarkup::PREFERED);
    pub const ALL: &'static [Self] = &[
        Self::Markup(FormatMarkup::Ron),
        Self::Markup(FormatMarkup::Json),
        Self::Markup(FormatMarkup::Xml),
        Self::Special(FormatSpecial::Txt),
        Self::Special(FormatSpecial::TmpBin),
    ];

    pub const fn extension(self) -> &'static str
    {
        match self
        {
            AnyFormat::Markup(v) => v.extension(),
            AnyFormat::Special(v) => v.extension(),
        }
    }

    pub fn encode<T>(self, value: &T) -> EncodeResult<Vec<u8>>
        where T: Serialize
    {
        match self
        {
            AnyFormat::Markup(v) => v.encode(value).map(|v| v.into_bytes()),
            AnyFormat::Special(v) => v.encode(value),
        }
    }

    pub fn encode_with_writer<T, W>(self, value: &T, writer: W) -> EncodeResult
        where T: Serialize, W: Write
    {
        match self
        {
            AnyFormat::Markup(v) => v.encode_with_writer(value, writer),
            AnyFormat::Special(v) => v.encode_with_writer(value, writer),
        }
    }

    pub fn from_bytes<T>(self, bytes: &[u8]) -> EncodeResult<T>
        where T: for<'de> Deserialize<'de>
    {
        match self
        {
            AnyFormat::Markup(v) => v.from_bytes(bytes),
            AnyFormat::Special(v) => v.from_bytes(bytes),
        }
    }

    pub fn from_reader<T, R>(self, reader: R) -> EncodeResult<T>
        where T: for<'de> Deserialize<'de>, R: Read
    {
        match self
        {
            AnyFormat::Markup(v) => v.from_reader(reader),
            AnyFormat::Special(v) => v.from_reader(reader),
        }
    }
}
impl Display for AnyFormat
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            AnyFormat::Markup(v) => write!(f, "{:?}", v),
            AnyFormat::Special(v) => write!(f, "{:?}", v),
        }
    }
}
impl<'a> TryFrom<&'a str> for AnyFormat
{
    type Error=();
    fn try_from(value: &'a str) -> Result<Self, Self::Error>
    {
        if let Ok(v) = FormatMarkup::try_from(value) { return Ok(Self::Markup(v)); }
        if let Ok(v) = FormatSpecial::try_from(value) { return Ok(Self::Special(v)); }
        Err(())
    }
}
