use super::*;

pub trait FsProvider
{
    type Fs: FsWrite;
    fn provide_fs() -> Self::Fs;
}
impl FsProvider for Io
{
    type Fs = Io;
    fn provide_fs() -> Self::Fs { Io }
}

/*
pub trait PersistantValue<T> : Persistant + Into<T>
{
    //fn into_value(&mut self) -> T;
}
*/

pub trait Persistant: GetPath + SetPath + Reload
{
    fn save(&mut self) -> FileResult;
}

pub trait PersistantValue<T>: Persistant + Guarded<T> {}
impl<S, T> PersistantValue<T> for S where S: Persistant + Guarded<T> {}

pub trait FsSave<T> : FsProvider
    where 
    T: Save + ?Sized,
{
    /// Encode the value using the provided extension and write it to a file.
    fn save<P : AsRef<Path>>(value: &T, path: P) -> FileResult<PathBuf>
    {
        value.save_to_fs(&mut Self::provide_fs(), path)
    }
}
impl<S,T> FsSave<T> for S 
    where S: FsProvider, 
    T: Save + ?Sized,
{}

pub trait FsLoad<T, FS>
where
    T: Load,
    FS: FsProvider,
{
    type Output: PersistantValue<T>;
    fn from_path_and_value(path: Option<PathBuf>, value: T) -> Self::Output;
    fn from_value(value: T) -> Self::Output { Self::from_path_and_value(None, value) }

    /// Read and decode the value using the provided extension.
    fn load<P: AsRef<Path>>(path: P) -> FileResult<Self::Output>
    {
        let path = path.as_ref();
        let mut path = FS::provide_fs().resolve_path(path).map_err(|e| FileError::new(e).with_path(Some(path.to_path_buf())))?;
        if path.extension().is_none()
        {
            path.set_extension(T::load_prefered_extension());
        }
        let value = T::load_from_fs(&mut FS::provide_fs(), &path)?;
        Ok(Self::from_path_and_value(Some(path), value))
    }

    /// Read and decode the value using the provided extension.
    /// If the file don't exist, the value is created, saved, and returned.
    /// It's ok if saving fail.
    /// 
    /// If the file exists but is malformed/corrupted/badly encoded, return an error and don't override the file.
    fn try_load_or_create<P: AsRef<Path>, F>(path: P, init: F) -> FileResult<Self::Output>
    where
        F: FnOnce() -> T,
    {
        let path = path.as_ref();
        match Self::load(&path)
        {
            Ok(v) => Ok(v),
            Err(e) =>
            {
                if e.is_encode() 
                {
                    // Badly encoded
                    return Err(e);
                }
                let value = init();

                let mut path = FS::provide_fs().resolve_path(path).unwrap_or_else(|_| path.to_path_buf());
                if path.extension().is_none()
                {
                    path.set_extension(T::load_prefered_extension());
                }

                let mut fs_value = Self::from_path_and_value(Some(path), value);
                let _ = fs_value.save();
                Ok(fs_value)
            }
        }
    }

    /// Read and decode the value using the provided extension.
    /// If the file don't exist, the value is created, saved, and returned.
    /// It's ok if saving fail.
    /// 
    /// **Important:** If the file exists but is malformed/corrupted/badly encoded, it will be **silently overwritten** with the init value. 
    /// Use [`Self::try_load_or_create`] to avoid silently override a badly encoded file.
    fn load_or_create<P: AsRef<Path>, F>(path: P, init: F) -> Self::Output
    where
        F: FnOnce() -> T,
    {
        let path = path.as_ref();
        match Self::load(&path)
        {
            Ok(v) => v,
            Err(_) =>
            {
                let value = init();

                let mut path = FS::provide_fs().resolve_path(path).unwrap_or_else(|_| path.to_path_buf());
                if path.extension().is_none()
                {
                    path.set_extension(T::load_prefered_extension());
                }

                let mut fs_value = Self::from_path_and_value(Some(path), value);
                let _ = fs_value.save();
                fs_value
            }
        }
    }

    /// Read the file content and load a decode it using the provided extension.
    /// If the file don't exist, the value is created using [`Default`], saved, and returned.
    /// It's ok if saving fail.
    ///
    /// **Important:** If the file exists but is malformed/corrupted/badly encoded, it will be **silently overwritten** with the init value. 
    /// Use [`Self::try_load_or_default`] to avoid silently override a badly encoded file.
    fn load_or_default<P: AsRef<Path>>(path: P) -> Self::Output
    where
        P: AsRef<Path>,
        T: Default,
    {
        Self::load_or_create(path, || ___())
    }


    /// Read the file content and load a decode it using the provided extension.
    /// If the file don't exist, the value is created using [`Default`], saved, and returned.
    /// It's ok if saving fail.
    ///
    /// If the file exists but is malformed/corrupted/badly encoded, return an error and don't override the file.
    fn try_load_or_default<P: AsRef<Path>>(path: P) -> FileResult<Self::Output>
    where
        P: AsRef<Path>,
        T: Default,
    {
        Self::try_load_or_create(path, || ___())
    }
}

impl<T> FsLoad<T, Io> for Io
where
    T: Load + Save,
{
    type Output = FileDataIn<T, Io>;
    fn from_path_and_value(path: Option<PathBuf>, value: T) -> Self::Output { FileDataIn::<T, Io>::from_path_and_value(path, value) }
}
