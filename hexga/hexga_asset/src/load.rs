use super::*;

pub trait AssetLoad : Async
{
    fn load<P>(path: P) -> Asset<Self> where P: Into<Path>, Self: Load
    {
        Asset::<Self>::load(path)
    }

    // Todo: renable it
    // fn to_asset_with_path<P>(self, path: P) -> Asset<Self> where P: AsRefPath, Self: Sized
    // {
    //     Asset::<Self>::load_from_value(path.as_ref(), self).with_path(Some(path))
    // }
    //
    // fn to_asset_with_id()
}
impl<T> AssetLoad for T where T: Async {}