use serde::ser::Error;

use super::*;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct AssetError
{
    pub path: Path,
    pub kind: IoError,
    pub childs: Vec<AssetError>
}


// TODO: impl it
impl Display for AssetError
{
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl std::error::Error for AssetError
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)>
    {
        None
    }
    fn provide<'a>(&'a self, _: &mut std::error::Request<'a>) {}
}



impl serde::ser::Error for AssetError
{
    fn custom<T>(msg:T) -> Self where T:std::fmt::Display
    {
        Self { kind: IoError::custom(msg), ..Default::default() }
    }
}