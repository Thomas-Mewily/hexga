use super::*;

pub type FileData<T> = FileDataIn<T>;

#[derive(Clone)]
pub struct FileDataIn<T, FS = Io>
where
    FS: FileSystemProvider,
    T: Load + Save,
{
    /// If no path, it's just a value
    path: Option<PathBuf>,
    // Always Some. Is only used for Self::into_value()
    value: Option<Dirty<T>>,
    phantom: PhantomData<FS>,
}

// Todo: better impl
/*
impl<T, FS> Load for FileDataIn<T, FS>
where
    FS: FsProvider,
    T: Load + Save,
{
}

impl<T, FS> Save for FileDataIn<T, FS>
where
    FS: FsProvider,
    T: Load + Save,
{
}

impl<T, FS> FsProvider for FileDataIn<T, FS>
where
    FS: FsProvider,
    T: Load + Save,
{
    type Fs = FS::Fs;
    fn provide_fs() -> Self::Fs { FS::provide_fs() }
}
*/

#[cfg(feature = "serde")]
impl<T, FS> Serialize for FileDataIn<T, FS>
where
    FS: FileSystemProvider,
    T: Load + Save + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("FileDataIn", 2)?;
        state.serialize_field("path", &self.path)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

#[cfg(feature = "serde")]
impl<'de, T, FS> Deserialize<'de> for FileDataIn<T, FS>
where
    FS: FileSystemProvider,
    T: Load + Save + for<'de2> Deserialize<'de2>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct FileDataIn<T>
        {
            path: Option<PathBuf>,
            value: Option<Dirty<T>>,
        }

        let helper = FileDataIn::deserialize(deserializer)?;
        Ok(super::FileDataIn {
            path: helper.path,
            value: helper.value,
            phantom: PhantomData,
        })
    }
}

impl<T, FS> From<T> for FileDataIn<T, FS>
where
    FS: FileSystemProvider,
    T: Load + Save,
{
    fn from(value: T) -> Self { Self::from_path_and_value(None, value) }
}

impl<T, FS> Guarded<T> for FileDataIn<T, FS>
where
    FS: FileSystemProvider,
    T: Load + Save,
{
    type Guard<'a>
        = &'a T
    where
        Self: 'a;
    type Error<'a>
        = Never
    where
        Self: 'a;

    fn try_get<'a>(&'a self) -> Result<Self::Guard<'a>, Self::Error<'a>> { Ok(self.value()) }
}
/*
impl<T,FS> GuardedMut<T> for FileDataOf<T,FS> where
    FS: FsProvider,
    T: Load + Save
{
    type GuardMut<'a> = &'a mut T where Self: 'a ;
    type Error<'a> = Never where Self: 'a;

    fn try_get_mut<'a>(&'a self) -> Result<Self::GuardMut<'a>, Self::Error<'a>> {
        Ok(self.value_mut())
    }
}
*/

impl<T, FS> Saveable for FileDataIn<T, FS>
where
    FS: FileSystemProvider,
    T: Load + Save,
{
    fn save(&mut self) -> FileResult
    {
        if self.is_not_dirty()
        {
            return Ok(());
        }
        self.save_forced()
    }

    fn save_forced(&mut self) -> FileResult
    {
        let Some(path) = &self.path
        else
        {
            return Ok(());
        };
        self.value().save_to_fs_at(&mut FS::provide_fs(), path)?;
        /*
        if let Some(resolved) = self.value().save_to_fs_resolved(&mut FS::provide_fs(), path)?
        {
            let _ = self.set_path(Some(resolved));
        }*/
        self.undirty();
        Ok(())
    }
}

impl<T, FS> Reload for FileDataIn<T, FS>
where
    FS: FileSystemProvider,
    T: Load + Save,
{
    type Ok = ();
    type Error = FileError;

    fn try_reload(&mut self) -> Result<Self::Ok, Self::Error>
    {
        let Some(path) = &self.path
        else
        {
            return Ok(());
        };

        match T::load_from_fs(&mut FS::provide_fs(), path)
        {
            Ok(v) =>
            {
                *self.value_mut() = v;
                Ok(())
            }
            Err(e) =>
            {
                if e.is_io() && path.extension().is_some()
                {
                    // Maybe the extension was changed
                    let resolved = FS::provide_fs()
                        .resolve_path(path.with_extension(""))
                        .map_err(|e| FileError::new(e).with_path(Some(path.to_path_buf())))?;
                    if *path != resolved
                    {
                        match T::load_from_fs(&mut FS::provide_fs(), &resolved)
                        {
                            Ok(v) =>
                            {
                                *self.value_mut() = v;
                                let _ = self.set_path(Some(resolved));
                                Ok(())
                            }
                            Err(e) =>
                            {
                                // Still change the path
                                let _ = self.set_path(Some(resolved));
                                Err(e)
                            }
                        }
                    }
                    else
                    {
                        Err(e)
                    }
                }
                else
                {
                    Err(e)
                }
            }
        }
    }
}


impl<T, FS> FileSystemLoadSave<T, FS> for FileDataIn<T, FS>
where
    FS: FileSystemProvider,
    T: Load + Save,
{
    type Output = Self;
    fn from_path_and_value(path: Option<PathBuf>, value: T) -> Self::Output
    {
        Self {
            path,
            value: Some(Dirty::new(value)),
            phantom: PhantomData,
        }
    }
}

impl<T, FS> Hash for FileDataIn<T, FS>
where
    FS: FileSystemProvider,
    T: Load + Save + Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H)
    {
        self.path.hash(state);
        self.value().hash(state);
    }
}

impl<T, FS> Ord for FileDataIn<T, FS>
where
    FS: FileSystemProvider,
    T: Load + Save + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering { (&self.path, self.value()).cmp(&(&other.path, other.value())) }
}
impl<T, FS> PartialOrd for FileDataIn<T, FS>
where
    FS: FileSystemProvider,
    T: Load + Save + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { (&self.path, self.value()).partial_cmp(&(&other.path, other.value())) }
}

impl<T, FS> Eq for FileDataIn<T, FS>
where
    FS: FileSystemProvider,
    T: Load + Save + Eq,
{
}
impl<T, FS> PartialEq for FileDataIn<T, FS>
where
    FS: FileSystemProvider,
    T: Load + Save + PartialEq,
{
    fn eq(&self, other: &Self) -> bool { self.path == other.path && self.value() == other.value() }
}

impl<T, FS> Debug for FileDataIn<T, FS>
where
    FS: FileSystemProvider,
    T: Load + Save + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { f.debug_struct("File").field("path", &self.path).field("value", self.value()).finish() }
}
impl<T, FS> Display for FileDataIn<T, FS>
where
    FS: FileSystemProvider,
    T: Load + Save + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult
    {
        write!(f, "File {}", self.value())?;
        if let Some(path) = &self.path
        {
            write!(f, " at {}", path.display())?;
        }
        Ok(())
    }
}

impl<T, FS> Deref for FileDataIn<T, FS>
where
    FS: FileSystemProvider,
    T: Load + Save,
{
    type Target = T;
    fn deref(&self) -> &Self::Target { self.value.as_ref().unwrap().deref() }
}
impl<T, FS> DerefMut for FileDataIn<T, FS>
where
    FS: FileSystemProvider,
    T: Load + Save,
{
    fn deref_mut(&mut self) -> &mut Self::Target { self.value.as_mut().unwrap().deref_mut() }
}

impl<T, FS> FileDataIn<T, FS>
where
    FS: FileSystemProvider,
    T: Load + Save,
{
    pub fn value(&self) -> &T { self.deref() }
    pub fn value_mut(&mut self) -> &mut T { self.deref_mut() }

    /// Extract the value without saving it.
    pub fn into_value(mut self) -> T { std::mem::take(&mut self.value).unwrap().into_value() }
}

impl<T, FS> IsDirty for FileDataIn<T, FS>
where
    FS: FileSystemProvider,
    T: Load + Save,
{
    fn is_dirty(&self) -> bool { self.value.as_ref().unwrap().is_dirty() }
}
impl<T, FS> SetDirty for FileDataIn<T, FS>
where
    FS: FileSystemProvider,
    T: Load + Save,
{
    fn set_dirty(&mut self, used: bool) -> &mut Self
    {
        self.value.as_mut().unwrap().set_dirty(used);
        self
    }
}
impl<T, FS> GetPath for FileDataIn<T, FS>
where
    FS: FileSystemProvider,
    T: Load + Save,
{
    fn get_path(&self) -> Option<PathBuf>
    {
        match &self.path
        {
            Some(p) => Some(p.clone()),
            None => None,
        }
    }
}
impl<T, FS> SetPath for FileDataIn<T, FS>
where
    FS: FileSystemProvider,
    T: Load + Save,
{
    fn set_path<P: AsRef<Path>>(&mut self, path: Option<P>) -> IoResult
    {
        let path = path.as_ref().map(|p| p.as_ref());
        self.path = match path
        {
            Some(p) => match FS::provide_fs().resolve_path(p)
            {
                Ok(resolved) => Some(resolved),
                Err(_) => Some(p.to_path_buf()),
            },
            None => None,
        };
        self.mark_dirty();
        Ok(())
    }

    fn rename_path<P: AsRef<Path>>(&mut self, to: P) -> IoResult
    {
        let path = match &self.path
        {
            Some(path) => path,
            None =>
            {
                return self.set_path(Some(to));
            }
        };
        let dest = to.as_ref();
        match FS::provide_fs().rename(path, dest)
        {
            Ok(path) =>
            {
                let _ = self.set_path(Some(path));
                Ok(())
            }
            Err(e) =>
            {
                let _ = self.set_path(Some(dest.to_path_buf()));
                Err(e)
            }
        }
    }
}

impl<T, FS> Drop for FileDataIn<T, FS>
where
    FS: FileSystemProvider,
    T: Load + Save,
{
    fn drop(&mut self)
    {
        // Todo: if saving fail, try to save the file somewhere else / in a recovery folder ?
        let _ = self.save();
    }
}

pub trait ToFileData: Load + Save
{
    fn to_file_data<FS: FileSystemProvider>(self, path: Option<PathBuf>) -> FileDataIn<Self, FS> { FileDataIn::from_path_and_value(path, self) }
}
impl<T> ToFileData for T where T: Load + Save {}
