use super::*;

#[derive(Clone)]
pub struct AssetData<T, IO=Io> // A:AutoSave>
    where 
    IO: FsProvider,
    T: Load + Save
{
    file: FileDataOf<T,IO>
}

impl<T,IO> std::hash::Hash for AssetData<T,IO> where 
    IO: FsProvider,
    T: Load + Save 
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.file.hash(state);
    }
}

impl<T,IO> Ord for AssetData<T,IO> where 
    IO: FsProvider,
    T: Load + Save 
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.file.cmp(&other.file)
    }
}
impl<T,IO> PartialOrd for AssetData<T,IO> 
where 
    IO: FsProvider,
    T: Load + Save 
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.file.partial_cmp(&other.file)
    }
}

impl<T,IO> Eq for AssetData<T,IO> where 
    IO: FsProvider,
    T: Load + Save 
{}
impl<T,IO> PartialEq for AssetData<T,IO> 
where 
    IO: FsProvider,
    T: Load + Save 
{
    fn eq(&self, other: &Self) -> bool {
        self.file == other.file
    }
}

impl<T,IO> Debug for AssetData<T,IO> 
    where 
    IO: FsProvider,
    T: Load + Save + Debug
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(&self.file, f)
    }
}

impl<T,IO> Display for AssetData<T,IO> 
    where 
    IO: FsProvider,
    T: Load + Save + Display
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.file, f)
    }
}

/*
impl<T,IO> Deref for AssetData<T,IO> 
    where 
    IO: IoProvider,
    T: Load + Save
{
    type Target=Dirty<T>;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl<T,IO> DerefMut for AssetData<T,IO> 
    where 
    IO: IoProvider,
    T: Load + Save
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
*/


/*
impl<T,IO> AssetData<T,IO> 
    where 
    IO: IoProvider,
    T: Load + Save
{
    pub fn load<P: AsRef<Path>>(path: P) -> IoResult<AssetData<T,IO>>
    {
        let mut io = IO::default();
        let path = io.resolve_path(path)?;
        let value = io.load_unresolved(&path)?;
        Ok(Self{ path, value: Dirty::new(value), phantom: PhantomData })
    }
    pub fn load_or_create<P: AsRef<Path>, F>(path: P, init: F) -> AssetData<T,IO> where T: Load + Save + Sized, F: FnOnce() -> T 
    {
        let mut io = IO::default();
        let p = path.as_ref();
        let path = io.resolve_path(p).unwrap_or_else(|_| p.to_owned());
        let value = io.load_or_create(&path, init);
        Self { path: path, value: Dirty::new(value), phantom: PhantomData }
    }
    pub fn load_unresolved<P: AsRef<Path>>(path: P) -> IoResult<AssetData<T,IO>>
    {
        let path = path.as_ref();
        let value = IO::default().load_unresolved(path)?;
        Ok(Self{ path: path.to_owned(), value: Dirty::new(value), phantom: PhantomData })
    }
    pub fn load_or_create_unresolved<P: AsRef<Path>, F>(path: P, init: F) -> AssetData<T,IO> where T: Load + Save + Sized, F: FnOnce() -> T 
    {
        let path = path.as_ref();
        let value = IO::default().load_or_create_unresolved(path, init);
        Self { path: path.to_owned(), value: Dirty::new(value), phantom: PhantomData }
    }

    pub fn save(&mut self) -> IoResult
    { 
        if self.is_not_dirty() { return Ok(()); }
        self.undirty();
        self.path = IO::default().save(&self.path, self.value())?;
        Ok(())
    }

    pub fn save_unresolved(&mut self) -> IoResult 
    { 
        if self.is_not_dirty() { return Ok(()); }
        self.undirty();
        IO::default().save_unresolved(&self.path, self.value()) 
    }

    pub fn value(&self) -> &T { self.deref() }
    pub fn value_mut(&mut self) -> &mut T { self.deref_mut() }

    pub fn path(&self) -> &Path { &self.path }

    /// Change the path without moving the old file.
    pub fn set_path_unresolved<P: AsRef<Path>>(&mut self, path: P)
    {
        self.path = path.as_ref().to_path_buf();
        self.mark_dirty();
    }

    /// Change the path without moving the old file.
    pub fn set_path<P: AsRef<Path>>(&mut self, path: P)
    {
        let path = path.as_ref();
        self.path = IO::default().resolve_path(path).unwrap_or_else(|_| 
            match self.path.extension()
            {
                Some(ex) => path.with_extension(ex),
                None => path.to_owned(),
            }
        );
        self.mark_dirty();
    }

    /// Change the path and move the old file.
    /// Do not change the file path if the renaming failed.
    pub fn rename<P: AsRef<Path>>(&mut self, to: P) -> IoResult
    {
        let mut io = IO::default();
        self.path = io.rename(&self.path, &to)?;
        Ok(())
    }

    /// Change the path and move the old file.
    /// Do not change the file path if the renaming failed.
    pub fn rename_unresolved<P: AsRef<Path>>(&mut self, to: P) -> IoResult
    {
        let mut io = IO::default();
        let to = to.as_ref().to_owned();
        io.rename_unresolved(&self.path, &to)?;
        self.path = to;
        Ok(())
    }

    /*
    pub fn into_value_without_saving(self) -> T {
        let Self { value, path, phantom } = self;
        let result = value.into_value();
        result
    }*/
}

impl<T,IO> Reload for AssetData<T,IO> 
    where 
    IO: IoProvider,
    T: Load + Save
{
    type Ok = ();
    type Error = IoError;

    fn try_reload(&mut self) -> Result<Self::Ok, Self::Error> {
        self.path = IO::default().resolve_path(&self.path).unwrap_or_else(|_| std::mem::take(&mut self.path));
        match IO::default().load(&self.path)
        {
            Ok(v) => { self.value = Dirty::new(v); Ok(()) },
            Err(e) => Err(e),
        }
    }
}

impl<T,IO> Drop for AssetData<T,IO> 
    where 
    IO: IoProvider,
    T: Load + Save
{
    fn drop(&mut self) {
        // Todo: if saving fail, try to save the file somewhere else / in a recovery folder ?
        let _ = self.save();
    }
}
*/