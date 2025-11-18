use super::*;

pub trait PathExtension
{
    fn extension_or_empty(&self) -> &extension;
}
impl PathExtension for Path
{
    fn extension_or_empty(&self) -> &extension {
        match self.extension()
        {
            Some(ex) => match ex.to_str()
            {
                Some(e) => e,
                None => "",
            },
            None => "",
        }
    }
}