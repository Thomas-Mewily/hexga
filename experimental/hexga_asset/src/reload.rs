use super::*;


/// 🔥 Hot Reloading.
pub trait Reload
{
    type Ok;
    type Error;
    fn try_reload(&mut self) -> Result<Self::Ok, Self::Error>;
    fn reload(&mut self) -> bool { self.try_reload().is_ok() }
}

/*
pub trait PersistentSave
{
    fn save(&mut self) -> IoResult;
}
*/


/*
impl<T> HotReload for Asset<T>
where
    T: Async + Load,
{
    type Ok = Option<AssetState<T>>;
    type Error = IoError;

    fn hot_reload(&mut self) -> Result<Self::Ok, Self::Error> { self.hot_reload() }
}

impl<T> HotReload for AssetManager<T>
where
    T: Async + Load,
{
    type Ok = ();
    type Error = Vec<IoError>;

    fn hot_reload(&mut self) -> Result<Self::Ok, Self::Error>
    {
        let mut error = Vec::new();
        for mut asset in self.iter()
        {
            match asset.hot_reload()
            {
                Ok(_) =>
                {}
                Err(err) => error.push(err),
            };
        }
        if error.is_empty() { Ok(()) } else { Err(error) }
    }
}
*/