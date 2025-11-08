use super::*;

pub trait HotReload
{
    type Ok;
    type Error;
    fn hot_reload(&mut self) -> Result<Self::Ok, Self::Error>;
}

impl<T> HotReload for AssetData<T>
    where T:Async + Load
{
    type Ok=Option<T>;
    type Error=IoError;

    fn hot_reload(&mut self) -> Result<Self::Ok, Self::Error> {
        self.hot_reload()
    }
}


impl<T> HotReload for Asset<T>
    where T:Async + Load
{
    type Ok=Option<T>;
    type Error=IoError;

    fn hot_reload(&mut self) -> Result<Self::Ok, Self::Error> {
        (**self).hot_reload()
    }
}


impl<T> HotReload for AssetManager<T>
    where T:Async + Load
{
    type Ok=();
    type Error=Vec<IoError>;

    fn hot_reload(&mut self) -> Result<Self::Ok, Self::Error> {
        let mut error = Vec::new();
        for asset in self.iter_mut()
        {
            match asset.hot_reload()
            {
                Ok(_) => {},
                Err(err) => error.push(err),
            };
        }
        if error.is_empty() { Ok(()) } else { Err(error) }
    }
}