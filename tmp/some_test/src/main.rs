use std::marker::PhantomData;

use hexga::prelude::*;

pub struct File<T, F=Io>
    where 
    F: Fs,
    T: Load + Save
{
    pub path: PathBuf,
    pub value: T,
    phantom: PhantomData<F>,
}
impl<T,F> File<T,F> 
    where 
    F: Fs,
    T: Load + Save
{
    pub fn load<P: AsRef<Path>>(path: P) -> IoResult<File<T,F>>
    {
        let path = path.as_ref();
        let value = F::default().load(path)?;
        Ok(Self{ path: path.to_owned(), value, phantom: PhantomData })
    }
    pub fn load_or_create
}

fn main()
{

}