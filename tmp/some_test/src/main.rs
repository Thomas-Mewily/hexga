use std::marker::PhantomData;

use hexga::{prelude::*, utils::dirty::SetDirty};

pub struct File<T, IO=Io>
    where 
    IO: IoProvider,
    T: Load + Save
{
    pub path: PathBuf,
    pub value: Dirty<T>,
    phantom: PhantomData<IO>,
}
impl<T,IO> Deref for File<T,IO> 
    where 
    IO: IoProvider,
    T: Load + Save
{
    type Target=Dirty<T>;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl<T,IO> DerefMut for File<T,IO> 
    where 
    IO: IoProvider,
    T: Load + Save
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T,IO> File<T,IO> 
    where 
    IO: IoProvider,
    T: Load + Save
{
    pub fn load<P: AsRef<Path>>(path: P) -> IoResult<File<T,IO>>
    {
        let path = path.as_ref();
        let value = IO::default().load(path)?;
        Ok(Self{ path: path.to_owned(), value: Dirty::new(value), phantom: PhantomData })
    }
    pub fn load_or_create<P: AsRef<Path>, F>(path: P, init: F) -> File<T,IO> where T: Load + Save + Sized, F: FnOnce() -> T 
    {
        let path = path.as_ref();
        let value = IO::default().load_or_create(path, init);
        Self { path: path.to_owned(), value: Dirty::new(value), phantom: PhantomData }
    }
    pub fn load_unresolved<P: AsRef<Path>>(path: P) -> IoResult<File<T,IO>>
    {
        let path = path.as_ref();
        let value = IO::default().load_unresolved(path)?;
        Ok(Self{ path: path.to_owned(), value: Dirty::new(value), phantom: PhantomData })
    }
    pub fn load_or_create_unresolved<P: AsRef<Path>, F>(path: P, init: F) -> File<T,IO> where T: Load + Save + Sized, F: FnOnce() -> T 
    {
        let path = path.as_ref();
        let value = IO::default().load_or_create_unresolved(path, init);
        Self { path: path.to_owned(), value: Dirty::new(value), phantom: PhantomData }
    }

    pub fn save(&mut self) -> IoResult 
    { 
        if self.is_not_dirty() { return Ok(()); }
        self.undirty();
        IO::default().save(&self.path, self.value()) 
    }

    pub fn save_unresolved(&mut self) -> IoResult 
    { 
        if self.is_not_dirty() { return Ok(()); }
        self.undirty();
        IO::default().save_unresolved(&self.path, self.value()) 
    }

    pub fn value(&self) -> &T { self.deref() }
    pub fn value_mut(&mut self) -> &mut T { self.deref_mut() }

    //pub fn drop_without_save
}
impl<T,IO> Drop for File<T,IO> 
    where 
    IO: IoProvider,
    T: Load + Save
{
    fn drop(&mut self) {
        // Todo: if saving fail, try to save the file somewhere else / in a recovery folder ?
        let _ = self.save();
    }
}

fn main()
{

}