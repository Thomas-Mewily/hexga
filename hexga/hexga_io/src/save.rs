use crate::*;

pub type Path = String;
#[allow(non_camel_case_types)]
pub type path = str;

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


#[allow(unused_variables)]
#[allow(async_fn_in_trait)]
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
    fn open_custom_extension() -> impl Iterator<Item=&'static str> { Self::BasedOn::open_custom_extension() }

    /// Dedicated file extension to load the value. ex `png`, `jpeg` for image
    /// 
    /// Don't include the markup language extension like `json` or `ron`
    fn can_be_open_from_custom_extension(extension : &path) -> bool { Self::open_custom_extension().position(|e| e == extension).is_some() }

    /// Also include the markup language extension like `json` or `ron`
    fn open_extension() -> impl Iterator<Item=&'static str> { Self::open_custom_extension().chain(Io::MARKUP_LANGAGE_EXTENSION) }
    /// Also include the markup language extension like `json` or `ron`
    fn can_be_open_from_extension(extension : &path) -> bool { Self::open_extension().position(|e| e == extension).is_some() }

    async fn from_file_composite(path : &path) -> Result<Self,M::Err>
    {
        match Box::pin(Self::BasedOn::from_file_composite(path)).await
        {
            Ok(base) => Self::from_based_on(base),
            Err(e) => Err(e),
        }
    }

    
    fn from_bytes_with_extension(data : &[u8], extension : &str) -> Result<Self,M::Err> 
    {
        match extension
        {
            /* 
            "ron"  => Self::from_ron_buf (data),
            "json" => Self::from_json_buf(data),
            "xml"  => Self::from_xml_buf (data),
            "yaml" | "yml" => Self::from_yaml_buf(data),
            */
            _ => match Self::BasedOn::from_bytes_with_extension(data, extension)
            {
                Ok(base) => Self::from_based_on(base),
                Err(_) => match Self::can_be_open_from_custom_extension(extension)
                {
                    true => Self::from_bytes_with_own_extension(data, extension),
                    false => Self::result_err(format!("Can't load {} from extension .{}", std::any::type_name::<Self>(), extension))
                },
            }
        }
    }

    fn from_bytes_with_own_extension(raw : &[u8], extension : &str) -> Result<Self,M::Err> 
    {
        Self::result_err(format!("can't open composite {} from bytes", std::any::type_name::<Self>()))
    }

    fn from_based_on(base : Self::BasedOn) -> Result<Self, M::Err> { Self::result_err(format!("Can't open composite {} from bytes", std::any::type_name::<Self>())) }





}

pub(crate) trait InnerSave<M> where Self: Sized + Serialize + for<'de> Deserialize<'de>, M : IoMediator
{
    fn err(str : String) -> M::Err where M : IoMediator { M::Err::new(str) }
    fn result_err<T>(str : String) -> Result<T,M::Err> where M : IoMediator { Err(Self::err(str)) }
}

impl<T,M> InnerSave<M> for T where T : IoSave<M>, M : IoMediator {}