use std::{borrow::Cow, collections::hash_map::Values};

use hexga_encoding::FileErrorKind;

use super::*;



/*
pub trait PersistantValue<T> : Persistant + Into<T>
{
    //fn into_value(&mut self) -> T;
}
*/

trait_marker!(Persistant: Saveable + GetPath + SetPath + Reload);

pub trait Saveable
{
    /// Attempts to save the value, but may skip the operation if no changes have been made.
    fn save(&mut self) -> FileResult { self.save_forced() }

    /// Forcefully saves the value, regardless of whether it has been modified.
    fn save_forced(&mut self) -> FileResult;
}

/*
pub trait PersistantValue<T>: Persistant + Guarded<T> {}
impl<S, T> PersistantValue<T> for S where S: Persistant + Guarded<T> {}
*/

pub trait FileSystemSave<T>: FileSystemProvider
where
    T: Save + ?Sized,
{
    /// Encode the value using the provided extension and write it to a file.
    fn save<P: AsRef<Path>>(value: &T, path: P) -> FileResult { value.save_to_fs(&mut Self::file_system(), path) }
}
impl<S, T> FileSystemSave<T> for S
where
    S: FileSystemProvider,
    T: Save + ?Sized,
{
}

pub trait FileSystemLoadSave<T, FS>
where
    T: Load + Save,
    FS: FileSystemProvider,
{
    type Output: Persistant; //PersistantValue<T>;
    fn from_path_and_value(path: Option<PathBuf>, value: T) -> Self::Output;
    /// This fn can return an error only if the init fn return an error. Otherwise it's a logic bug in the impl.
    /// Resolving the path should be done here.
    #[doc(hidden)]
    fn from_path_and_fn<F>(mut path: Option<PathBuf>, init: F) -> FileResult<Self::Output>
    where
        F: FnOnce(Option<&mut PathBuf>) -> FileResult<T>,
    {
        path.as_mut().map(|p| *p = FS::file_system().resolve_path_for::<T, _>(&p));
        let value = init(path.as_mut())?;
        Ok(Self::from_path_and_value(path, value))
    }

    fn from_value(value: T) -> Self::Output { Self::from_path_and_value(None, value) }

    /// Read and decode the value using the provided extension.
    fn load<P: AsRef<Path>>(path: P) -> FileResult<Self::Output>
    {
        Self::from_path_and_fn(Some(path.as_ref().to_path_buf()), |p| match p
        {
            Some(path) => T::load_from_fs_at(&mut FS::file_system(), &path),
            None => Err(FileError::new(FileErrorKind::Io(IoError::new(IoErrorKind::InvalidData, "Missing path")))),
        })
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

                let path = FS::file_system().resolve_path_for::<T, _>(path);

                let mut need_save = false;
                let mut fs_value = Self::from_path_and_fn(Some(path), |_| {
                    need_save = true;
                    Ok(init())
                })?;
                if need_save
                {
                    let _ = fs_value.save_forced();
                }
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
                let path = FS::file_system().resolve_path_for::<T, _>(path);

                let mut need_save = false;
                let mut fs_value = Self::from_path_and_fn(Some(path), |_| {
                    need_save = true;
                    Ok(init())
                })
                .expect("Bad impl");
                if need_save
                {
                    let _ = fs_value.save_forced();
                }
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

impl<T> FileSystemLoadSave<T, IoGlobal> for IoGlobal
where
    T: Load + Save,
{
    type Output = FileIn<T, IoGlobal>;
    fn from_path_and_value(path: Option<PathBuf>, value: T) -> Self::Output { FileIn::<T, IoGlobal>::from_path_and_value(path, value) }
}
