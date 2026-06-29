use super::*;


pub trait FsProvider
{
    type Fs : FsWrite;
    fn provide_fs() -> Self::Fs;
}
impl FsProvider for Io
{
    type Fs = Io;
    fn provide_fs() -> Self::Fs {
        Io
    }
}

/*
pub trait PersistantValue<T> : Persistant + Into<T>
{
    //fn into_value(&mut self) -> T;
}
*/

pub trait Persistant : GetPath + SetPath + Reload
{
    fn save(&mut self) -> EncodeResult;
}

pub trait PersistantValue<T> : Persistant + Guarded<T> {}
impl<S,T> PersistantValue<T> for S where S:  Persistant + Guarded<T> {}

pub trait FsLoad<T,FS>
    where 
    T: Load,
    FS: FsProvider
{
    type Output : PersistantValue<T>;
    /// Unresolved version of the path
    fn from_path_and_value(path: Option<PathBuf>, value: T) -> Self::Output;

    /// Read and decode the value using the provided extension.
    fn load<P: AsRef<Path>>(path: P) -> EncodeResult<Self::Output>
    {
        let mut path = FS::provide_fs().resolve_path(path)?;
        if path.extension().is_none() && let Some(ex) = T::load_prefered_extension() {
            path.set_extension(ex);
        }
        let value = T::load_from_fs(&mut FS::provide_fs(), &path)?;
        Ok(Self::from_path_and_value(Some(path), value))
    }

    /// Read and decode the value using the provided extension.
    /// If the file don't exist, the value is created, saved, and returned.
    /// It's ok if saving fail.
    fn load_or_create<P: AsRef<Path>, F>(path: P, init: F) -> Self::Output 
        where F: FnOnce() -> T 
    {
        let path = path.as_ref();
        match Self::load(&path)
        {
            Ok(v) => v,
            Err(_) => 
            {
                let value = init();
                
                let mut path = FS::provide_fs().resolve_path(path).unwrap_or_else(|_| path.to_path_buf());
                if path.extension().is_none() && let Some(ex) = T::load_prefered_extension() {
                    path.set_extension(ex);
                }

                let mut fs_value = Self::from_path_and_value(Some(path), value);
                let _ = fs_value.save();
                fs_value
            },
        }
    }

    /// Read the file content and load a decode it using the provided extension.
    /// If the file don't exist, the value is created using [`Default`], saved, and returned.
    /// It's ok if saving fail.
    fn load_or_default<P: AsRef<Path>>(&mut self, path: P) -> Self::Output where P: AsRef<Path>, T: Default
    {
        Self::load_or_create(path, || ___())
    }
}


impl<T> FsLoad<T,Io> for Io where 
    T: Load + Save 
{
    type Output=FileDataOf<T,Io>;
    fn from_path_and_value(path: Option<PathBuf>, value: T) -> Self::Output {
        FileDataOf::<T,Io>::from_path_and_value(path, value)
    }
}