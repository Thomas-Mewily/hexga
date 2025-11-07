use super::*;

pub trait AssetLoad : Async
{
    fn load<P>(path: P) -> Asset<Self> where P: AsRefPath, Self: Load
    {
        Asset::<Self>::load(path)
    }

    fn to_asset<P>(self, path: P) -> Asset<Self> where P: AsRefPath, Self: Sized
    {
        Asset::<Self>::load_from_value(path, self)
    }
}
impl<T> AssetLoad for T where T: Async {}