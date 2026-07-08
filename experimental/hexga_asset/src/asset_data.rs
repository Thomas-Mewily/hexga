use std::any::TypeId;

use super::*;

pub type AssetData<T> = AssetDataIn<T>;

// Not a FileData, because every operation that can modify the Path also need to update hashmap<Path, Asset> inside the global AssetManager.
#[derive(Clone)]
pub struct AssetDataIn<T, FS = Io>
where
    FS: FsProvider + Async,
    T: Load + Save + Async,
{
    /// If no path, it's just a value
    pub(crate) path: Option<PathBuf>,
    
    // Probably a bad idea since it can't be serialized
    //pub(crate) storage : Option<AssetStorage>,

    // Always Some. Is only used for Self::into_value()
    pub(crate) value: Option<Dirty<T>>,
    pub(crate) phantom: PhantomData<FS>,
}

impl<T, FS> Hash for AssetDataIn<T, FS>
where
    FS: FsProvider + Async,
    T: Load + Save + Async + Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H)
    {
        self.path.hash(state);
        self.value().hash(state);
    }
}

impl<T, FS> Ord for AssetDataIn<T, FS>
where
    FS: FsProvider + Async,
    T: Load + Save + Async + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering { (&self.path, self.value()).cmp(&(&other.path, other.value())) }
}
impl<T, FS> PartialOrd for AssetDataIn<T, FS>
where
    FS: FsProvider + Async,
    T: Load + Save + Async + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { (&self.path, self.value()).partial_cmp(&(&other.path, other.value())) }
}

impl<T, FS> Eq for AssetDataIn<T, FS>
where
    FS: FsProvider + Async,
    T: Load + Save + Async + Eq,
{
}
impl<T, FS> PartialEq for AssetDataIn<T, FS>
where
    FS: FsProvider + Async,
    T: Load + Save + Async + PartialEq,
{
    fn eq(&self, other: &Self) -> bool { self.path == other.path && self.value() == other.value() }
}

impl<T, FS> Debug for AssetDataIn<T, FS>
where
    FS: FsProvider + Async,
    T: Load + Save + Async + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { f.debug_struct("Asset").field("path", &self.path).field("value", self.value()).finish() }
}

impl<T, FS> Display for AssetDataIn<T, FS>
where
    FS: FsProvider + Async,
    T: Load + Save + Async + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult
    {
        write!(f, "Asset {}", self.value())?;
        if let Some(path) = &self.path
        {
            write!(f, " at {}", path.display())?;
        }
        Ok(())
    }
}

impl<T, FS> Deref for AssetDataIn<T, FS>
where
    FS: FsProvider + Async,
    T: Load + Save + Async,
{
    type Target = T;
    fn deref(&self) -> &Self::Target { self.value.as_ref().unwrap().deref() }
}
impl<T, FS> DerefMut for AssetDataIn<T, FS>
where
    FS: FsProvider + Async,
    T: Load + Save + Async,
{
    fn deref_mut(&mut self) -> &mut Self::Target { self.value.as_mut().unwrap().deref_mut() }
}

impl<T, FS> AssetDataIn<T, FS>
where
    FS: FsProvider + Async,
    T: Load + Save + Async,
{
    pub fn value(&self) -> &T { self.deref() }
    pub fn value_mut(&mut self) -> &mut T { self.deref_mut() }

    // Extract the value without saving it.
    // pub fn into_value(mut self) -> T { std::mem::take(&mut self.value).unwrap().into_value() }
}

impl<T, FS> IsDirty for AssetDataIn<T, FS>
where
    FS: FsProvider + Async,
    T: Load + Save + Async,
{
    fn is_dirty(&self) -> bool { self.value.as_ref().unwrap().is_dirty() }
}
impl<T, FS> SetDirty for AssetDataIn<T, FS>
where
    FS: FsProvider + Async,
    T: Load + Save + Async,
{
    fn set_dirty(&mut self, used: bool) -> &mut Self
    {
        self.value.as_mut().unwrap().set_dirty(used);
        self
    }
}

impl<T, FS> Drop for AssetDataIn<T, FS>
where
    FS: FsProvider + Async,
    T: Load + Save + Async,
{
    fn drop(&mut self)
    {
        // Todo: if saving fail, try to save the file somewhere else / in a recovery folder ?
        let _ = self.save();
    }
}

impl<T, FS> Saveable for AssetDataIn<T, FS>
where
    FS: FsProvider + Async,
    T: Load + Save + Async,
{
    fn save(&mut self) -> FileResult
    {
        if self.is_not_dirty()
        {
            return Ok(());
        }
        self.save_forced()
    }

    fn save_forced(&mut self) -> FileResult {
        let Some(path) = &self.path
        else
        {
            return Ok(());
        };
        self.value().save_to_fs(&mut FS::provide_fs(), path)?;
        self.undirty();
        Ok(())
    }
}