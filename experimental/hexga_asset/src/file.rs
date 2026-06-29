use super::*;

pub type FileData<T> = FileDataOf<T>;

#[derive(Clone)]
pub struct FileDataOf<T, FS=Io> // A:AutoSave>
    where 
    FS: FsProvider,
    T: Load + Save
{
    /// If no path, it's just a value
    path: Option<PathBuf>,
    // Always Some. Is only used for Self::into_value()
    value: Option<Dirty<T>>,
    phantom: PhantomData<FS>,
}


pub trait FsLoad<T,FS>
    where FS: FsProvider
{
    type Output;
    fn from_path_and_value(path: Option<PathBuf>, value: T) -> Self::Output;

    fn load_unresolved<P: AsRef<Path>>(path: P) -> IoResult<Self::Output>
    {
        let path = path.as_ref();
        let value = FS::provide_fs().read
        //Ok(Self)
    }

    fn load<P: AsRef<Path>>(path: P) -> IoResult<Self::Output>
    {
        let mut io = FS::provide_fs();
        let path = io.resolve_path(path)?;
        let value = io.load_unresolved(&path)?;
        Ok(Self{ path, value: Dirty::new(value), phantom: PhantomData })
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

impl<T,FS> FsLoad<T,FS> for FileDataOf<T,FS> where 
    FS: FsProvider,
    T: Load + Save 
{
    type Output=Self;
    fn from_path_and_value(path: Option<PathBuf>, value: T) -> Self::Output {
        Self{ path, value: Some(Dirty::new(value)), phantom: PhantomData }
    }
}

impl<T,FS> FsProvider for FileDataOf<T,FS> where 
    FS: FsProvider,
    T: Load + Save 
{
    type Fs = FS::Fs;
    fn provide_fs() -> Self::Fs {
        FS::provide_fs()
    }
}

impl<T,FS> Hash for FileDataOf<T,FS> where 
    FS: FsProvider,
    T: Load + Save + Hash
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_path().hash(state);
        self.value().hash(state);
    }
}

impl<T,FS> Ord for FileDataOf<T,FS> where 
    FS: FsProvider,
    T: Load + Save + Ord
{
    fn cmp(&self, other: &Self) -> Ordering {
        (self.get_path(), self.value()).cmp(&(other.get_path(), other.value()))
    }
}
impl<T,FS> PartialOrd for FileDataOf<T,FS> 
where 
    FS: FsProvider,
    T: Load + Save + PartialOrd
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.get_path(), self.value()).partial_cmp(&(other.get_path(), other.value()))
    }
}

impl<T,FS> Eq for FileDataOf<T,FS> where 
    FS: FsProvider,
    T: Load + Save + Eq
{}
impl<T,FS> PartialEq for FileDataOf<T,FS> 
where 
    FS: FsProvider,
    T: Load + Save + PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.get_path() == other.get_path() && self.value() == other.value()
    }
}

impl<T,FS> Debug for FileDataOf<T,FS> 
    where 
    FS: FsProvider,
    T: Load + Save + Debug
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("File").field("path", &self.get_path()).field("value", self.value()).finish()
    }
}

impl<T,FS> Display for FileDataOf<T,FS> 
    where 
    FS: FsProvider,
    T: Load + Save + Display
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.value())?;
        if let Some(path) = self.get_path()
        {
            write!(f, " at {}", path.display())?;
        }
        Ok(())
    }
}

impl<T,FS> Deref for FileDataOf<T,FS> 
    where 
    FS: FsProvider,
    T: Load + Save
{
    type Target=T;
    fn deref(&self) -> &Self::Target {
        self.value.as_ref().unwrap().deref()
    }
}
impl<T,FS> DerefMut for FileDataOf<T,FS> 
    where 
    FS: FsProvider,
    T: Load + Save
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value.as_mut().unwrap().deref_mut()
    }
}

impl<T,IO> FileDataOf<T,IO> 
    where 
    IO: FsProvider,
    T: Load + Save
{
    /*
    pub fn load<P: AsRef<Path>>(path: P) -> IoResult<FileDataOf<T,IO>>
    {
        let mut io = IO::default();
        let path = io.resolve_path(path)?;
        let value = io.load_unresolved(&path)?;
        Ok(Self{ path, value: Dirty::new(value), phantom: PhantomData })
    }
    pub fn load_or_create<P: AsRef<Path>, F>(path: P, init: F) -> FileDataOf<T,IO> where T: Load + Save + Sized, F: FnOnce() -> T 
    {
        let mut io = IO::default();
        let p = path.as_ref();
        let path = io.resolve_path(p).unwrap_or_else(|_| p.to_owned());
        let value = io.load_or_create(&path, init);
        Self { path: path, value: Dirty::new(value), phantom: PhantomData }
    }
    pub fn load_unresolved<P: AsRef<Path>>(path: P) -> IoResult<FileDataOf<T,IO>>
    {
        let path = path.as_ref();
        let value = IO::default().load_unresolved(path)?;
        Ok(Self{ path: path.to_owned(), value: Dirty::new(value), phantom: PhantomData })
    }
    pub fn load_or_create_unresolved<P: AsRef<Path>, F>(path: P, init: F) -> FileDataOf<T,IO> where T: Load + Save + Sized, F: FnOnce() -> T 
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
    */
    
    pub fn value(&self) -> &T { self.deref() }
    pub fn value_mut(&mut self) -> &mut T { self.deref_mut() }

    /*
    pub fn path(&self) -> Option<&Path> { self.path.map(|p| p.as_path()) }

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
    */

    /*
    pub fn into_value_without_saving(self) -> T {
        let Self { value, path, phantom } = self;
        let result = value.into_value();
        result
    }*/

    pub fn into_value_without_saving(mut self) -> T 
    {
        std::mem::take(&mut self.value).unwrap().into_value()
    }
}

impl<T,FS> IsDirty for FileDataOf<T,FS> 
    where 
    FS: FsProvider,
    T: Load + Save
{
    fn is_dirty(&self) -> bool {
        self.value.as_ref().unwrap().is_dirty()
    }
}
impl<T,FS> SetDirty for FileDataOf<T,FS> 
    where 
    FS: FsProvider,
    T: Load + Save
{
    fn set_dirty(&mut self, used: bool) -> &mut Self {
        self.value.as_mut().unwrap().set_dirty(used);
        self
    }
}
impl<T,FS> GetPath for FileDataOf<T,FS> 
    where 
    FS: FsProvider,
    T: Load + Save
{
    fn get_path(&self) -> Option<&Path> {
        match &self.path
        {
            Some(p) => Some(p.deref()),
            None => None,
        }
    }
}


/*
impl<T,IO> Reload for FileDataOf<T,IO> 
    where 
    IO: Fs,
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
*/

impl<T,FS> Drop for FileDataOf<T,FS> 
    where 
    FS: FsProvider,
    T: Load + Save
{
    fn drop(&mut self) {
        // Todo: if saving fail, try to save the file somewhere else / in a recovery folder ?
        //let _ = self.save();
        todo!()
    }
}