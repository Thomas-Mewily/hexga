use super::*;

pub trait AssetSave
{
    type Ok;
    type Error;
    fn save(&mut self) -> Result<Self::Ok, Self::Error>;
}

impl<T> AssetSave for AssetData<T>
    where T:Async + Save
{
    type Ok=();
    type Error=IoError;

    fn save(&mut self) -> Result<Self::Ok, Self::Error> {
        self.save()
    }
}


impl<T> AssetSave for Asset<T>
    where T:Async + Save
{
    type Ok=();
    type Error=IoError;

    fn save(&mut self) -> Result<Self::Ok, Self::Error> {
        (**self).save()
    }
}


impl<T> AssetSave for AssetManager<T>
    where T:Async + Save
{
    type Ok=();
    type Error=Vec<IoError>;

    fn save(&mut self) -> Result<Self::Ok, Self::Error> {
        let mut error = Vec::new();
        for asset in self.iter_mut()
        {
            match asset.save()
            {
                Ok(_) => {},
                Err(err) => error.push(err),
            };
        }
        if error.is_empty() { Ok(()) } else { Err(error) }
    }
}