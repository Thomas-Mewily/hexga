use std::{any::TypeId, sync::{Arc, RwLock}};
use hexga_core::sync::{ArcWeak};

use super::*;

pub(crate) static ASSET: SingletonLazyRw<AssetsManagerUntyped> = SingletonLazyRw::new(|| AssetsManagerUntyped::default());

#[derive(Default)]
pub(crate) struct AssetsManagerUntyped
{
    /// HashMap<typeof(AssetManager<T,FS>), Box<AssetManager<T,FS>>>
    assets: HashMap<TypeId, Box<DynAnyAsync>>,
}
impl AssetsManagerUntyped
{
    pub(crate) fn get_manager<'a,T, FS>(&'a mut self) -> &'a mut AssetManager<T, FS>
    where
        FS: FsProvider + Async,
        T: Load + Save + Async,
    {
        let typeid = TypeId::of::<AssetManager<T, FS>>();

        let boxed_any: &'a mut Box<dyn AnyAsync + 'static> = self.assets
            .entry(typeid)
            .or_insert_with(|| {
                Box::new(AssetManager::<T, FS>::___()) as Box<DynAnyAsync>
            });

        boxed_any.as_any_mut()
            .downcast_mut::<AssetManager<T, FS>>()
            .expect("Type mismatch")
    }
}

pub(crate) type ArcRwLockAssetManager<T,FS> = Arc<RwLock<AssetManager<T,FS>>>;

#[derive(Debug)]
pub(crate) struct AssetManager<T,FS>
where
    FS: FsProvider + Async,
    T: Load + Save + Async,
{
    pub(crate) values: HashMap<PathBuf, AssetStorage<T, FS>>,
}
impl<T,FS> Default for AssetManager<T,FS>
    where
        FS: FsProvider + Async,
        T: Load + Save + Async,
{
    fn default() -> Self {
        Self { values: Default::default() }
    }
}

#[derive(Debug)]
pub enum AssetStorage<T,FS>
where
    FS: FsProvider + Async,
    T: Load + Save + Async,
{
    Strong(AssetIn<T,FS>),
    Weak(AssetWeakIn<T,FS>),
}
impl<T,FS> AssetStorage<T,FS>
    where 
    FS: FsProvider + Async,
    T: Load + Save + Async
{
    pub fn is_strong(&self) -> bool { matches!(self, Self::Strong(_)) }
    pub fn is_weak(&self) -> bool { matches!(self, Self::Weak(_)) }
}

impl<T,FS> SharedCount for AssetStorage<T,FS>
    where 
    FS: FsProvider + Async,
    T: Load + Save + Async
{
    fn strong_count(&self) -> usize {
        match self
        {
            AssetStorage::Strong(v) => v.strong_count(),
            AssetStorage::Weak(v) => v.strong_count(),
        }
    }

    fn weak_count(&self) -> usize {
        match self
        {
            AssetStorage::Strong(v) => v.weak_count(),
            AssetStorage::Weak(v) => v.weak_count(),
        }
    }
}
impl<T,FS> SharedDowngrade for AssetStorage<T,FS>
    where 
    FS: FsProvider + Async,
    T: Load + Save + Async
{
    type Ouput = AssetWeakIn<T,FS>;

    fn downgrade(&self) -> Self::Ouput {
        match self
        {
            AssetStorage::Strong(v) => v.downgrade(),
            AssetStorage::Weak(v) => v.clone(),
        }
    }
}

impl<T,FS> SharedUpgrade for AssetStorage<T,FS>
    where 
    FS: FsProvider + Async,
    T: Load + Save + Async
{
    type Output = AssetIn<T,FS>;

    fn upgrade(&self) -> Option<Self::Output> {
        match self
        {
            AssetStorage::Strong(v) => Some(v.clone()),
            AssetStorage::Weak(v) => v.upgrade(),
        }
    }
}
/*
pub struct AssetIn<T, IO>
    where 
    IO: FsProvider,
    T: Load + Save + Async
{
    data: FileDataOf<T, IO>
}
*/

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum AssetLocation<T,P>
{
    Path(P),
    Value(T),
}


pub struct AssetIn<T,FS>
    where 
    FS: FsProvider + Async,
    T: Load + Save + Async,
{
    inner: Arc<RwLock<AssetDataIn<T,FS>>>
}


#[cfg(feature = "serde")]
impl<T, FS> Serialize for AssetIn<T, FS>
where
    FS: FsProvider + Async,
    T: Load + Save + Async + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let asset = self.get();
        match asset.get_path()
        {
            Some(path) => AssetLocation::Path(path),
            None => AssetLocation::Value(asset.value()),
        }.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, T, FS> Deserialize<'de> for AssetIn<T, FS>
where
    FS: FsProvider + Async,
    T: Load + Save + Async + for <'de2> Deserialize<'de2>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = AssetLocation::<T,PathBuf>::deserialize(deserializer)?;
        match val 
        {
            AssetLocation::Path(path) => AssetIn::load(path).map_err(|e| serde::de::Error::custom(e.to_debug())),
            AssetLocation::Value(value) => Ok(AssetIn::from_value(value)),
        }
    }
}

impl<T,FS> Clone for AssetIn<T,FS>
    where 
    FS: FsProvider + Async,
    T: Load + Save + Async
{
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}
impl<T,FS> SharedCount for AssetIn<T,FS>
    where 
    FS: FsProvider + Async,
    T: Load + Save + Async
{
    fn strong_count(&self) -> usize {
        self.inner.strong_count()
    }

    fn weak_count(&self) -> usize {
        self.inner.weak_count()
    }
}
impl<T,FS> SharedDowngrade for AssetIn<T,FS>
    where 
    FS: FsProvider + Async,
    T: Load + Save + Async
{
    type Ouput = AssetWeakIn<T,FS>;

    fn downgrade(&self) -> Self::Ouput {
        AssetWeakIn{ inner: self.inner.downgrade() }
    }
}
impl<T,FS> Debug for AssetIn<T,FS>
    where 
    FS: FsProvider + Async,
    T: Load + Save + Async + Debug
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.try_get()
        {
            Ok(v) => write!(f, "{:?}", v.deref()),
            Err(e) => write!(f, "Can't read asset {:?}", e),
        }
    }
}
impl<T,FS> Guarded<AssetDataIn<T,FS>> for AssetIn<T,FS>
    where 
    FS: FsProvider + Async,
    T: Load + Save + Async
{
    type Guard<'a> = <RwLock<AssetDataIn<T,FS>> as Guarded<AssetDataIn<T,FS>>>::Guard<'a> where Self: 'a;
    type Error<'a> = <RwLock<AssetDataIn<T,FS>> as Guarded<AssetDataIn<T,FS>>>::Error<'a> where Self: 'a;
    fn try_get<'a>(&'a self) -> Result<Self::Guard<'a>, Self::Error<'a>> { self.inner.try_get() }
}
impl<T,FS> GuardedMut<AssetDataIn<T,FS>> for AssetIn<T,FS>
    where 
    FS: FsProvider + Async,
    T: Load + Save + Async
{
    type GuardMut<'a> = <RwLock<AssetDataIn<T,FS>> as GuardedMut<AssetDataIn<T,FS>>>::GuardMut<'a> where Self: 'a;
    type Error<'a> = <RwLock<AssetDataIn<T,FS>> as GuardedMut<AssetDataIn<T,FS>>>::Error<'a> where Self: 'a;
    fn try_get_mut<'a>(&'a self) -> Result<Self::GuardMut<'a>, Self::Error<'a>> { self.inner.try_get_mut() }
}

impl<T, FS> GetPath for AssetIn<T, FS>
where
    FS: FsProvider + Async,
    T: Load + Save + Async,
{
    fn get_path(&self) -> Option<PathBuf>
    {
        self.get().get_path()
    }
}
impl<T, FS> SetPath for AssetIn<T, FS>
where
    FS: FsProvider + Async,
    T: Load + Save + Async,
{
    fn set_path<P: AsRef<Path>>(&mut self, path: Option<P>) -> IoResult {
        todo!();
        self.get_mut().set_path(path)
    }

    fn rename_path<P: AsRef<Path>>(&mut self, to: P) -> IoResult {
        todo!();
        self.get_mut().rename_path(to)
    }
}

impl<T, FS> Persistant for AssetIn<T, FS>
where
    FS: FsProvider + Async,
    T: Load + Save + Async,
{
    fn save(&mut self) -> FileResult
    {
        self.get_mut().save()
    }
}

impl<T, FS> Reload for AssetIn<T, FS>
where
    FS: FsProvider + Async,
    T: Load + Save + Async,
{
    type Ok = ();
    type Error = FileError;

    fn try_reload(&mut self) -> Result<Self::Ok, Self::Error>
    {
        let (result, path_changed) = self.get_mut().try_reload_and_indicate_if_path_changed();
        if path_changed
        {
            self.set_path(path)
        }
        result
    }
}


impl<T, FS> FsLoad<T, FS> for AssetIn<T, FS>
where
    FS: FsProvider + Async,
    T: Load + Save + Async,
{
    type Output = Self;

    fn from_path_and_value(path: Option<PathBuf>, value: T) -> Self::Output
    {
        todo!()
        /*
        if let Some(path) = path
        {
            match ASSET.get_mut().get_manager::<T,FS>().values.get(&path)
            {
                Some(asset) => match asset
                {
                    AssetStorage::Strong(asset) => todo!(),
                    AssetStorage::Weak(asset) => todo!(),
                },
                None => todo!(),
            }
        }
        Self {
            file: FileDataIn::from_path_and_value(path, value)
        }
        */
    }
}




pub struct AssetWeakIn<T,FS>
    where 
    FS: FsProvider + Async,
    T: Load + Save + Async
{
    inner: ArcWeak<RwLock<AssetDataIn<T,FS>>>
}
impl<T,FS> Clone for AssetWeakIn<T,FS>
    where 
    FS: FsProvider + Async,
    T: Load + Save + Async
{
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}
impl<T,FS> SharedCount for AssetWeakIn<T,FS>
    where 
    FS: FsProvider + Async,
    T: Load + Save + Async
{
    fn strong_count(&self) -> usize {
        self.inner.strong_count()
    }

    fn weak_count(&self) -> usize {
        self.inner.weak_count()
    }
}
impl<T,FS> SharedUpgrade for AssetWeakIn<T,FS>
    where 
    FS: FsProvider + Async,
    T: Load + Save + Async
{
    type Output = AssetIn<T,FS>;
    
    fn upgrade(&self) -> Option<Self::Output> {
        self.inner.upgrade().map(|inner| AssetIn { inner })   
    }
}

impl<T,FS> Debug for AssetWeakIn<T,FS>
    where 
    FS: FsProvider + Async,
    T: Load + Save + Async + Debug
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.upgrade()
        {
            Some(asset) => write!(f, "{:?}", asset),
            None => write!(f, "Asset was dropped"),
        }
    }
}
