use std::any::TypeId;

use super::*;

pub type AssetData<T> = AssetDataIn<T>;

#[derive(Clone)]
pub struct AssetDataIn<T, FS = Io>
where
    FS: FsProvider,
    T: Load + Save,
{
    file: FileDataIn<T,FS>
}

// Todo: better impl
impl<T, FS> Load for AssetDataIn<T, FS>
where
    FS: FsProvider,
    T: Load + Save {}

impl<T, FS> Save for AssetDataIn<T, FS>
where
    FS: FsProvider,
    T: Load + Save {}

#[cfg(feature = "serde")]
impl<T, FS> Serialize for AssetDataIn<T, FS>
where
    FS: FsProvider,
    T: Load + Save + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.file.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, T, FS> Deserialize<'de> for AssetDataIn<T, FS>
where
    FS: FsProvider,
    T: Load + Save + for <'de2> Deserialize<'de2>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let file = FileDataIn::<T,FS>::deserialize(deserializer)?;
        Ok(AssetDataIn { file })
    }
}

/*
impl<T, FS> From<T> for AssetDataIn<T, FS>
where
    FS: FsProvider,
    T: Load + Save,
{
    fn from(value: T) -> Self { Self::from_path_and_value(None, value) }
}
*/

impl<T, FS> Guarded<T> for AssetDataIn<T, FS>
where
    FS: FsProvider,
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

    fn try_get<'a>(&'a self) -> Result<Self::Guard<'a>, Self::Error<'a>> { self.file.try_get() }
}
/*
impl<T,FS> GuardedMut<T> for AssetOf<T,FS> where
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

impl<T, FS> Persistant for AssetDataIn<T, FS>
where
    FS: FsProvider,
    T: Load + Save,
{
    fn save(&mut self) -> FileResult
    {
        self.file.save()
    }
}

impl<T, FS> Reload for AssetDataIn<T, FS>
where
    FS: FsProvider,
    T: Load + Save,
{
    type Ok = ();
    type Error = FileError;

    fn try_reload(&mut self) -> Result<Self::Ok, Self::Error>
    {
        let Some(path) = self.get_path()
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
                    let resolved = FS::provide_fs().resolve_path(path.with_extension("")).map_err(|e| FileError::new(e).with_path(Some(path.to_path_buf())))?;
                    if path != resolved
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

impl<T, FS> FsProvider for AssetDataIn<T, FS>
where
    FS: FsProvider,
    T: Load + Save,
{
    type Fs = FS::Fs;
    fn provide_fs() -> Self::Fs { FS::provide_fs() }
}



impl<T, FS> Hash for AssetDataIn<T, FS>
where
    FS: FsProvider,
    T: Load + Save + Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H)
    {
        self.get_path().hash(state);
        self.value().hash(state);
    }
}

impl<T, FS> Ord for AssetDataIn<T, FS>
where
    FS: FsProvider,
    T: Load + Save + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering { (self.get_path(), self.value()).cmp(&(other.get_path(), other.value())) }
}
impl<T, FS> PartialOrd for AssetDataIn<T, FS>
where
    FS: FsProvider,
    T: Load + Save + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { (self.get_path(), self.value()).partial_cmp(&(other.get_path(), other.value())) }
}

impl<T, FS> Eq for AssetDataIn<T, FS>
where
    FS: FsProvider,
    T: Load + Save + Eq,
{
}
impl<T, FS> PartialEq for AssetDataIn<T, FS>
where
    FS: FsProvider,
    T: Load + Save + PartialEq,
{
    fn eq(&self, other: &Self) -> bool { self.get_path() == other.get_path() && self.value() == other.value() }
}

impl<T, FS> Debug for AssetDataIn<T, FS>
where
    FS: FsProvider,
    T: Load + Save + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { f.debug_struct("Asset").field("path", &self.get_path()).field("value", self.value()).finish() }
}

impl<T, FS> Display for AssetDataIn<T, FS>
where
    FS: FsProvider,
    T: Load + Save + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult
    {
        write!(f, "{}", self.value())?;
        if let Some(path) = self.get_path()
        {
            write!(f, " at {}", path.display())?;
        }
        Ok(())
    }
}

impl<T, FS> Deref for AssetDataIn<T, FS>
where
    FS: FsProvider,
    T: Load + Save,
{
    type Target = T;
    fn deref(&self) -> &Self::Target { self.file.deref() }
}
impl<T, FS> DerefMut for AssetDataIn<T, FS>
where
    FS: FsProvider,
    T: Load + Save,
{
    fn deref_mut(&mut self) -> &mut Self::Target { self.file.deref_mut() }
}

impl<T, IO> AssetDataIn<T, IO>
where
    IO: FsProvider,
    T: Load + Save,
{
    pub fn value(&self) -> &T { self.deref() }
    pub fn value_mut(&mut self) -> &mut T { self.deref_mut() }

    // Extract the value without saving it.
    // pub fn into_value(mut self) -> T { std::mem::take(&mut self.value).unwrap().into_value() }
}

impl<T, FS> IsDirty for AssetDataIn<T, FS>
where
    FS: FsProvider,
    T: Load + Save,
{
    fn is_dirty(&self) -> bool { self.file.is_dirty() }
}
impl<T, FS> SetDirty for AssetDataIn<T, FS>
where
    FS: FsProvider,
    T: Load + Save,
{
    fn set_dirty(&mut self, used: bool) -> &mut Self
    {
        self.file.set_dirty(used);
        self
    }
}
impl<T, FS> GetPath for AssetDataIn<T, FS>
where
    FS: FsProvider,
    T: Load + Save,
{
    fn get_path(&self) -> Option<&Path>
    {
        self.file.get_path()
    }
}
impl<T, FS> SetPath for AssetDataIn<T, FS>
where
    FS: FsProvider,
    T: Load + Save,
{
    fn set_path<P: AsRef<Path>>(&mut self, path: Option<P>) -> IoResult
    {
        todo!();
        self.file.set_path(path);
    }

    fn rename_path<P: AsRef<Path>>(&mut self, to: P) -> IoResult
    {
        let path = self.get_path();
        let path = match &path
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

impl<T, FS> Drop for AssetDataIn<T, FS>
where
    FS: FsProvider,
    T: Load + Save,
{
    fn drop(&mut self)
    {
        // Todo: if saving fail, try to save the file somewhere else / in a recovery folder ?
        let _ = self.save();
    }
}
