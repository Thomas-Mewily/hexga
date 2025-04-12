use crate::*;

pub type Path = String;
#[allow(non_camel_case_types)]
pub type path = str;

pub type Extension = String;
#[allow(non_camel_case_types)]
pub type extension = str;

/* 
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
pub trait IoSave where Self: Sized + Serialize + for<'de> Deserialize<'de>
{
    /// Set this to Never if you are not based on another type
    /// 
    /// Useful if the current type is similar to another savable type
    /// ex : `Image` and `Texture`
    type BasedOn : IoSave; // = Never;

    /// Dedicated file extension to load the value. ex `png`, `jpeg` for image
    /// 
    /// Don't include the markup language extension like `json` or `ron`
    fn open_file_custom_extension() -> impl Iterator<Item=&'static extension> { Self::BasedOn::open_file_custom_extension() }

    /// Dedicated file extension to load the value. ex `png`, `jpeg` for image
    /// 
    /// Don't include the markup language extension like `json` or `ron`
    fn can_be_open_from_custom_extension(extension : &extension) -> bool { Self::open_file_custom_extension().position(|e| e == extension).is_some() }

    /// Also include the markup language extension like `json` or `ron`
    fn open_file_extension() -> impl Iterator<Item=&'static extension> { Self::open_file_custom_extension().chain(Io::MARKUP_LANGAGE_EXTENSION) }
    /// Also include the markup language extension like `json` or `ron`
    fn can_be_open_from_extension(extension : &path) -> bool { Self::open_file_extension().position(|e| e == extension).is_some() }

    async fn from_file_composite(path : &path, m : &mut M) -> Result<Self,M::Err>
    {
        match Box::pin(Self::BasedOn::from_file_composite(path, m)).await
        {
            Ok(base) => Self::from_based_on(base),
            Err(e) => Err(e),
        }
    }

    
    fn from_bytes_with_extension(data : &[u8], extension : &extension) -> Result<Self,M::Err> 
    {
        todo!("markup langage + do somethings cleaner with M");
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

    fn from_bytes_with_own_extension(raw : &[u8], extension : &extension) -> Result<Self,M::Err> 
    {
        Self::result_err(format!("can't open composite {} from bytes", std::any::type_name::<Self>()))
    }

    fn from_based_on(base : Self::BasedOn) -> Result<Self, M::Err> { Self::result_err(format!("Can't open composite {} from bytes", std::any::type_name::<Self>())) }




    /// Dedicated file extension to load the value. ex `png`, `jpeg` for image
    /// 
    /// Don't include the markup language extension like `json` or `ron`
    fn save_file_custom_extension() -> impl Iterator<Item=&'static extension> { Self::BasedOn::save_file_custom_extension() }
    /// Don't include the markup language extension like `json` or `ron`
    fn can_be_save_with_custom_extension(extension : &extension) -> bool { Self::save_file_custom_extension().position(|e| e == extension).is_some() }

    /// Also include the markup language extension like `json` or `ron`
    fn save_file_extension_with_markup() -> impl Iterator<Item=&'static extension> { Self::save_file_custom_extension().chain(Io::MARKUP_LANGAGE_EXTENSION) }
    /// Also include the markup language extension like `json` or `ron`
    fn can_be_save_with_extension(extension : &extension) -> bool { Self::save_file_extension_with_markup().position(|e| e == extension).is_some() }


    async fn save_file_composite(&self, path : &path, m : &mut M) -> Result<Self,M::Err> 
    {
        match self.get_based_on()
        {
            Some(v) => Box::pin(v.save_file_composite(path, m)).await,
            None => match self.get_based_on_by_value()
            {
                Some(v) => Box::pin(v.save_file_composite(path, m)).await,
                None => Self::err_with_path(path.to_owned(), "composite".to_owned()),
            }
        }
    }

    
    fn save_bytes_with_extension<'a, W: IoWrite>(&self, bytes : &mut W, extension : &str, m : &mut M) -> Result<(), IoRawError> 
    {
        
        match extension.is_markup_extension()
        {
            true => 
            {
                let str = match extension
                {
                    "ron"  => self.to_ron ()?,
                    "json" => self.to_json()?,
                    "xml"  => self.to_xml ()?,
                    "yaml" | "yml" => self.to_yaml()?,
                    _ => unimplemented!(),
                };

                write!(bytes, "{}", str).map_err(|e| e.to_debug())
            },
            false => match self.get_based_on()
            {
                Some(v) => v.save_bytes_with_extension(bytes, extension, m),
                None => match self.get_based_on_by_value()
                {
                    Some(v) => v.save_bytes_with_extension(bytes, extension, m),
                    None => match Self::can_save_own_extension(extension)
                    {
                        true => self.save_bytes_with_custom_extension(bytes, extension),
                        false => Err(format!("Can't save {} with extension .{}", std::any::type_name::<Self>(), extension)),
                    },
                }
            },
        }
    }

    fn save_bytes_with_custom_extension<'a, W: IoWrite>(&self, raw : &mut W, extension : &str) -> Result<(), IoRawError> 
    {
        Err(Self::open_file_extension_not_supported(extension))
    }
    
    fn get_based_on_by_value(&self) -> Option<Self::BasedOn> { None }
    fn get_based_on(&self) -> Option<&Self::BasedOn> { None }




    /// A single file
    fn is_file_terminal() -> bool { Self::open_file_own_extension().any(|_| true) || Self::save_file_own_extension().any(|_| true) }
    /// Composed of multiple file
    fn is_file_composite() -> bool { !Self::is_file_terminal() }

    fn can_open_with_file_extension(extension : &str) -> bool { Self::open_file_own_extension().any(|e| e == extension) }
    fn default_open_file_extension() -> Option<&'static str> { Self::open_file_own_extension().next() }
    fn open_file_extension_not_supported(extension : &str) -> IoRawError { format!("{} can't open .{}", std::any::type_name::<Self>(), extension) }

    /// The main way to load a file
    /// 
    /// Also correct the path if it don't have any extension
    /// 
    /// This allow to be independant over file extension like
    /// `Image::from_file("landscape")` instead of `Image::from_file("landscape.png")`
    async fn from_file(path : &path) -> IoResult<Self>
    {
        let mut err : Option<IoError> = None;
        for extension in Self::open_file_extension_with_markup()
        {
            let path_corrected = path.with_extension(extension);
            match Self::from_file_no_correction(&path_corrected, &extension).await
            {
                Ok(v) => { return Ok(v) },
                Err(e) => 
                {
                    match &mut err
                    {
                        Some(already_err) => 
                        {
                            if e.priority() >= already_err.priority() { err = Some(e); }
                        }
                        None => { err = Some(e); },
                    }
                },
            }

        }
        Self::from_file_composite(path).await.map_err(|e|
        {
            if e.is_never_error()
            {
                err.unwrap_or(e)
            }else
            {
                e
            }
        })
    }

    /// Won't complete/correct the path if the extension file is missing
    async fn from_file_no_correction(path : &path, extension : &str) -> IoResult<Self> 
    {
        match IO::read(path).await 
        {
            Ok(raw) => Self::from_raw_with_extension(&raw, extension).map_err(|e| IoError::to_io_err(path.to_owned(), IoErrorReason::Raw(e))),
            Err(e) =>  { Err(e) },
        }
    }
    async fn from_file_or_default(path : &path) -> Self where Self : Default { Self::from_file(path).await.unwrap_or_else(|_| ___()) }

    fn can_save_file_with_extension(extension : &str) -> bool { Self::save_file_own_extension().any(|e| e == extension) }
    fn default_save_file_extension() -> Option<&'static str> { Self::save_file_own_extension().next() }
    fn save_file_extension_not_supported(extension : &str) -> IoRawError { format!("{} can't be saved to .{}", std::any::type_name::<Self>(), extension) }

    /// Also override the file if it already exists
    async fn save_file_with_extension(&self, path : &path, extension : &str) -> IoResult<Path>
    {
        let mut raw = Vec::with_capacity(8192);
        assert!(!extension.contains('.'), "File extension should not contains a '.' / dot");
        self.save_bytes_with_extension(&mut raw, extension).map_err(|r| IoError::to_io_err(path.to_owned(), IoErrorReason::Raw(r)))?;

        let path = path.with_extension(extension);
        IO::write(&path, &raw).await?;
        Ok(path)
    }

    /// The main way to save a file
    /// 
    /// Also override the file if it already exists
    async fn save_file(&self, path : &path) -> IoResult<Path>
    {
        match path.extension()
        {
            Some(ex) => self.save_file_with_extension(path, ex).await,
            None => match Self::default_save_file_extension()
            {
                Some(ex) => self.save_file_with_extension(path, ex).await,
                None => self.save_file_composite(path).await,
            }
        }
    }

    async fn save_file_to_json(&self, path : &path) -> IoResult<Path> { self.save_file(&path.with_extension("json")).await }
    async fn save_file_to_ron (&self, path : &path) -> IoResult<Path> { self.save_file(&path.with_extension("ron" )).await }
    async fn save_file_to_xml (&self, path : &path) -> IoResult<Path> { self.save_file(&path.with_extension("xml" )).await }
    async fn save_file_to_yaml(&self, path : &path) -> IoResult<Path> { self.save_file(&path.with_extension("yaml")).await }

    /// return the extension of the file
    fn save_raw<'a, W: IoWrite>(&self, raw : &mut W) -> Result<&str, IoRawError>
    {
        for extension in Self::save_file_extension_with_markup()
        {
            let s = self.save_bytes_with_extension(raw, extension);
            if s.is_ok() { return Ok(extension); }
        }
        Err("unknow error while saving".to_owned())
    }
}

pub(crate) trait InnerSave<M> where Self: Sized + Serialize + for<'de> Deserialize<'de>, M : IoMediator
{
    fn err(reason : Reason) -> M::Err where M : IoMediator { M::Err::to_io_err(reason) }
    fn err_with_path(path : Path, reason : Reason) -> M::Err where M : IoMediator { M::Err::to_io_err_with_path(path, reason) }
    fn result_err<T>(reason : Reason) -> Result<T,M::Err> where M : IoMediator { Err(Self::err(reason)) }
    fn result_err_with_path<T>(path : Path, reason : Reason) -> Result<T,M::Err> where M : IoMediator { Err(Self::err_with_path(path, reason)) }
}

impl<T,M> InnerSave<M> for T where T : IoSave<M>, M : IoMediator {}
*/