use std::{any::TypeId, sync::{Arc, RwLock}};
use hexga_core::sync::ArcWeak;

use super::*;

pub(crate) static ASSET: SingletonOnceLazy<AssetsManagerUntyped> = SingletonOnceLazy::new(|| AssetsManagerUntyped::default());

#[derive(Default)]
pub(crate) struct AssetsManagerUntyped
{
    assets: HashMap<TypeId, Arc<DynAnyAsync>>,
}

/*
pub(crate) type ArcRwLockAssetManager<T> = Arc<RwLock<AssetManager<T>>>;

#[derive(Debug)]
pub(crate) struct AssetManager<T>
where
    T: Async,
{
    pub(crate) values: HashMap<PathBuf, AssetStorage<T>>,
}
*/

/*
#[derive(Debug)]
pub enum AssetStorage<T>
where
    T: Async,
{
    Persistant(Asset<T>),
    ReferenceCounted(AssetWeak<T>),
}
    */

/*
pub struct AssetData<T, IO>
    where 
    IO: FsProvider,
    T: Load + Save
{
    data: FileDataOf<T, IO>
}
*/

pub struct Asset<T,IO>
    where 
    IO: FsProvider,
    T: Load + Save
{
    inner: Arc<RwLock<AssetData<T,IO>>>
}


pub struct AssetWeak<T,IO>
    where 
    IO: FsProvider,
    T: Load + Save
{
    inner: ArcWeak<RwLock<AssetData<T,IO>>>
}