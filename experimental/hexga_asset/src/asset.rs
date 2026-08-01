use std::{
    any::TypeId,
    default, mem,
    sync::{Arc, RwLock, RwLockWriteGuard},
};

use super::*;

pub(crate) static ASSET: SingletonLazyRw<AssetsManagerUntyped> = SingletonLazyRw::new(|| AssetsManagerUntyped::default());

#[derive(Default)]
pub(crate) struct AssetsManagerUntyped
{
    /// HashMap<typeof(AssetManager<T,FS>), Box<AssetManager<T,FS>>>
    managers: HashMap<TypeId, Box<DynAnyAsync>>,
}
impl AssetsManagerUntyped
{
    pub(crate) fn manager_mut<'a, T, FS>(&'a mut self) -> &'a mut AssetManagerIn<T, FS>
    where
        FS: FileSystemProvider + Async,
        T: Load + Save + Async,
    {
        let typeid = TypeId::of::<AssetManagerIn<T, FS>>();

        let boxed_any: &'a mut Box<dyn AnyAsync + 'static> = self
            .managers
            .entry(typeid)
            .or_insert_with(|| Box::new(AssetManagerIn::<T, FS>::___()) as Box<DynAnyAsync>);

        boxed_any.deref_mut().as_any_mut().downcast_mut::<AssetManagerIn<T, FS>>().expect("Type mismatch")
    }

    pub(crate) fn try_manager<'a, T, FS>(&'a self) -> Option<&'a AssetManagerIn<T, FS>>
    where
        FS: FileSystemProvider + Async,
        T: Load + Save + Async,
    {
        let typeid = TypeId::of::<AssetManagerIn<T, FS>>();

        self.managers.get(&typeid)?.deref().as_any().downcast_ref::<AssetManagerIn<T, FS>>()
    }
}

pub type AssetManagerData<T> = AssetManagerIn<T, IoData>;
pub type AssetManagerGlobal<T> = AssetManagerIn<T, IoGlobal>;

#[derive(Debug)]
pub struct AssetManagerIn<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async,
{
    pub(crate) values: HashMap<PathBuf, AssetShared<T, FS>>,
    pub(crate) default_storage: AssetStorage,
    // PersistancePreference : Strong or Weak.
}
impl<T, FS> Default for AssetManagerIn<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async,
{
    fn default() -> Self
    {
        Self {
            values: ___(),
            default_storage: ___(),
        }
    }
}

impl<T, FS> AssetManagerIn<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async,
{
    /// Return all the assets that are loaded of this type.
    pub fn assets() -> Vec<AssetIn<T, FS>>
    {
        match ASSET.get().try_manager::<T, FS>()
        {
            Some(manager) => manager.values.iter().filter_map(|(_path, value)| value.upgrade()).collect(),
            None => Vec::new(),
        }
    }

    /// The default storage about how the asset should be stored in memory.
    pub fn default_storage() -> AssetStorage
    {
        match ASSET.get().try_manager::<T, FS>()
        {
            Some(manager) => manager.default_storage,
            None => AssetStorage::default(),
        }
    }

    pub fn set_default_storage(storage: AssetStorage)
    {
        let mut guard = ASSET.get_mut();
        let manager = guard.manager_mut::<T, FS>();

        if manager.default_storage == storage
        {
            return;
        }
        manager.default_storage = storage;

        match storage
        {
            AssetStorage::Persistant =>
            {
                // Also drop all weak asset that are dropped
                manager.values.retain(|_path, shared_asset| match shared_asset.upgrade()
                {
                    Some(asset) =>
                    {
                        *shared_asset = AssetShared::Strong(asset);
                        true
                    }
                    None => false,
                });
            }
            AssetStorage::ReferenceCounted =>
            {
                // Also drop all weak asset that are dropped
                manager.values.retain(|_path, shared_asset| {
                    *shared_asset = AssetShared::Weak(shared_asset.downgrade());
                    shared_asset.upgrade().is_some()
                });
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum AssetStorage
{
    /// If Persistant, the asset will continue to be loaded even if no one is using it.
    Persistant,
    /// If ReferenceCounted, the asset will unloaded when nobody use it. It will be loaded back from FS if someone need it.
    #[default]
    ReferenceCounted,
    // TODO impl it

    // If more asset need to be loaded, older asset will be unloaded first.
    // An existing instance of Asset may contains no data (None T), but when readed it can read/load the data back from the disk again (assuming the deserialization will work)
    // MaxBytes(usize),

    // Hint that the asset should be unloaded if not used for at least the wait time.
    // Wait(Time)
}

#[derive(Debug)]
pub enum AssetShared<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async,
{
    Strong(AssetIn<T, FS>),
    Weak(AssetWeakIn<T, FS>),
}
impl<T, FS> AssetShared<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async,
{
    pub fn is_strong(&self) -> bool { matches!(self, Self::Strong(_)) }
    pub fn is_weak(&self) -> bool { matches!(self, Self::Weak(_)) }
}

impl<T, FS> Clone for AssetShared<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async,
{
    fn clone(&self) -> Self
    {
        match self
        {
            Self::Strong(v) => Self::Strong(v.clone()),
            Self::Weak(v) => Self::Weak(v.clone()),
        }
    }
}

impl<T, FS> SharedCount for AssetShared<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async,
{
    fn strong_count(&self) -> usize
    {
        match self
        {
            AssetShared::Strong(v) => v.strong_count(),
            AssetShared::Weak(v) => v.strong_count(),
        }
    }

    fn weak_count(&self) -> usize
    {
        match self
        {
            AssetShared::Strong(v) => v.weak_count(),
            AssetShared::Weak(v) => v.weak_count(),
        }
    }
}
impl<T, FS> SharedDowngrade for AssetShared<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async,
{
    type Ouput = AssetWeakIn<T, FS>;

    fn downgrade(&self) -> Self::Ouput
    {
        match self
        {
            AssetShared::Strong(v) => v.downgrade(),
            AssetShared::Weak(v) => v.clone(),
        }
    }
}

impl<T, FS> SharedUpgrade for AssetShared<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async,
{
    type Output = AssetIn<T, FS>;

    fn upgrade(&self) -> Option<Self::Output>
    {
        match self
        {
            AssetShared::Strong(v) => Some(v.clone()),
            AssetShared::Weak(v) => v.upgrade(),
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
enum AssetLocation<T, P>
{
    Path(P),
    Value(T),
}

pub type AssetGlobal<T> = AssetIn<T, IoGlobal>;
pub type AssetData<T> = AssetIn<T, IoData>;

pub struct AssetIn<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async,
{
    inner: Arc<RwLock<AssetValue<T, FS>>>,
}

#[cfg(feature = "serde")]
impl<T, FS> Serialize for AssetIn<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let asset = self.get();
        match &asset.path
        {
            Some(path) => AssetLocation::Path(path),
            None => AssetLocation::Value(asset.value()),
        }
        .serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, T, FS> Deserialize<'de> for AssetIn<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async + for<'de2> Deserialize<'de2>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = AssetLocation::<T, PathBuf>::deserialize(deserializer)?;
        match val
        {
            AssetLocation::Path(path) => AssetIn::load(path).map_err(|e| serde::de::Error::custom(e.to_debug())),
            AssetLocation::Value(value) => Ok(AssetIn::from_value(value)),
        }
    }
}

impl<T, FS> Clone for AssetIn<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async,
{
    fn clone(&self) -> Self { Self { inner: self.inner.clone() } }
}
impl<T, FS> SharedCount for AssetIn<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async,
{
    fn strong_count(&self) -> usize { self.inner.strong_count() }

    fn weak_count(&self) -> usize { self.inner.weak_count() }
}
impl<T, FS> SharedDowngrade for AssetIn<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async,
{
    type Ouput = AssetWeakIn<T, FS>;

    fn downgrade(&self) -> Self::Ouput { AssetWeakIn { inner: self.inner.downgrade() } }
}

impl<T, FS> Wrapper for AssetIn<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async,
{
    type Inside = (Option<PathBuf>, T);
}
impl<T, FS> WrapperNew for AssetIn<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async,
{
    fn new_wrapper(value: Self::Inside) -> Self {
        Self::from_path_and_value(value.0, value.1)
    }
}
impl<T, FS> WrapperTryIntoInner for AssetIn<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async,
{
    fn into_inner(self) -> Option<Self::Inside> {
        Some(Arc::into_inner(self.inner)?.into_inner().ok()?.into_path_and_value())
    }
}
impl<T, FS> WrapperTryIntoInnerOrClone for AssetIn<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async + Clone,
{
    fn into_inner_or_clone(self) -> Self::Inside {
        match Arc::try_unwrap(self.inner)
        {
            Ok(guard) => guard.into_inner().expect("poisoned").into_path_and_value(),
            Err(inner) => {
                let asset = AssetIn { inner };
                let v = asset.get();
                (v.path.clone(), v.value().clone())
            },
        }
    }
}

impl<T, FS> Debug for AssetIn<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult
    {
        match self.try_get()
        {
            Ok(v) => write!(f, "{:?}", v.deref()),
            Err(e) => write!(f, "Can't read asset {:?}", e),
        }
    }
}
impl<T, FS> Display for AssetIn<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult
    {
        match self.try_get()
        {
            Ok(v) => write!(f, "{}", v.deref()),
            Err(e) => write!(f, "Can't read asset {:?}", e),
        }
    }
}
impl<T, FS> Guarded<AssetValue<T, FS>> for AssetIn<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async,
{
    type Guard<'a>
        = <RwLock<AssetValue<T, FS>> as Guarded<AssetValue<T, FS>>>::Guard<'a>
    where
        Self: 'a;
    type Error<'a>
        = <RwLock<AssetValue<T, FS>> as Guarded<AssetValue<T, FS>>>::Error<'a>
    where
        Self: 'a;
    fn try_get<'a>(&'a self) -> Result<Self::Guard<'a>, Self::Error<'a>> { self.inner.try_get() }
}
impl<T, FS> GuardedMut<AssetValue<T, FS>> for AssetIn<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async,
{
    type GuardMut<'a>
        = <RwLock<AssetValue<T, FS>> as GuardedMut<AssetValue<T, FS>>>::GuardMut<'a>
    where
        Self: 'a;
    type Error<'a>
        = <RwLock<AssetValue<T, FS>> as GuardedMut<AssetValue<T, FS>>>::Error<'a>
    where
        Self: 'a;
    fn try_get_mut<'a>(&'a self) -> Result<Self::GuardMut<'a>, Self::Error<'a>> { self.inner.try_get_mut() }
}

impl<T, FS> AssetIn<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async,
{
    pub fn value(&self) -> impl Deref<Target = T> { self.get().guard_map(|v| v.value()) }
    pub fn value_mut(&mut self) -> impl DerefMut<Target = T> { self.get_mut().guard_map_mut(|v| v.value_mut()) }
}

impl<T, FS> IsDirty for AssetIn<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async,
{
    fn is_dirty(&self) -> bool { self.get().is_dirty() }
}

impl<T, FS> SetDirty for AssetIn<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async,
{
    fn set_dirty(&mut self, used: bool) -> &mut Self
    {
        self.get_mut().set_dirty(used);
        self
    }
}

impl<T, FS> GetPath for AssetIn<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async,
{
    fn get_path(&self) -> Option<PathBuf> { self.get().path.clone() }
}

impl<T, FS> AssetIn<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async,
{
    pub(crate) fn force_set_path(self, data: &mut RwLockWriteGuard<'_, AssetValue<T, FS>>, path: Option<PathBuf>, manager: &mut AssetManagerIn<T, FS>)
    {
        //todo!("what about if the other path is already used by another asset. Need to merge them");
        if let Some(old_path) = &data.path
        {
            let _old_value = manager.values.remove(old_path);
            debug_assert!(_old_value.is_some());
            data.path = None;
        }

        if let Some(path) = path
        {
            let entry = match manager.default_storage
            {
                AssetStorage::Persistant => AssetShared::Strong(self),
                AssetStorage::ReferenceCounted => AssetShared::Weak(self.downgrade()),
            };

            if let Some(replaced_shared_asset) = manager.values.insert(path.to_owned(), entry)
            {
                if let Some(mut _replaced_asset) = replaced_shared_asset.upgrade()
                {
                    // Can't do anythings about it right now.
                    // I don't want to impose T: Clone.
                    // Maybe I can swap the old path with the new_path of the 2 instance ?
                    //*replaced_asset.get_mut() = self.get
                }
            }
            data.path = Some(path);
        }
    }
}

impl<T, FS> SetPath for AssetIn<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async,
{
    fn set_path<P: AsRef<Path>>(&mut self, path: Option<P>) -> IoResult
    {
        let path = path.as_ref().map(|p| p.as_ref());

        let data = self.get();

        if data.path.as_ref().map(|v| v.as_ref()) != path
        {
            drop(data);

            let mut data: RwLockWriteGuard<'_, AssetValue<T, FS>> = self.get_mut();
            let mut assets = ASSET.get_mut();
            let manager: &mut AssetManagerIn<T, FS> = assets.manager_mut::<T, FS>();

            self.clone().force_set_path(&mut data, path.map(|p| p.to_path_buf()), manager);
            data.mark_dirty();

            drop(data);
            drop(assets);
        }
        Ok(())
    }

    fn rename_path<P: AsRef<Path>>(&mut self, to: P) -> IoResult
    {
        let dest = to.as_ref();

        let mut data = self.get_mut();
        let path = match &data.path
        {
            Some(path) => path,
            None =>
            {
                let mut assets = ASSET.get_mut();
                let manager: &mut AssetManagerIn<T, FS> = assets.manager_mut::<T, FS>();
                data.mark_dirty();
                self.clone().force_set_path(&mut data, Some(dest.to_path_buf()), manager);
                return Ok(());
            }
        };

        let mut assets = ASSET.get_mut();
        let manager: &mut AssetManagerIn<T, FS> = assets.manager_mut::<T, FS>();
        match FS::file_system().rename(path, dest)
        {
            Ok(path) =>
            {
                data.mark_dirty();
                self.clone().force_set_path(&mut data, Some(path), manager);
                Ok(())
            }
            Err(e) =>
            {
                data.mark_dirty();
                self.clone().force_set_path(&mut data, Some(dest.to_path_buf()), manager);
                Err(e)
            }
        }
    }
}

impl<T, FS> Saveable for AssetIn<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async,
{
    fn save(&mut self) -> FileResult { self.get_mut().save() }

    fn save_forced(&mut self) -> FileResult { self.get_mut().save_forced() }
}

impl<T, FS> Reload for AssetIn<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async,
{
    type Ok = ();
    type Error = FileError;

    fn try_reload(&mut self) -> Result<Self::Ok, Self::Error>
    {
        let mut data = self.get_mut();
        let Some(path) = &mut data.path
        else
        {
            return Ok(());
        };

        match T::load_from_fs_resolved(&mut FS::file_system(), &path)
        {
            Ok((value, path)) =>
            {
                *data.value_mut() = value;
                if let Some(resolved) = path
                {
                    let mut assets = ASSET.get_mut();
                    let manager: &mut AssetManagerIn<T, FS> = assets.manager_mut::<T, FS>();
                    self.clone().force_set_path(&mut data, Some(resolved), manager);
                }
                Ok(())
            }
            Err(e) =>
            {
                if e.is_io() && path.extension().is_some()
                {
                    // Maybe the extension was changed
                    path.set_extension("");
                    match T::load_from_fs_resolved(&mut FS::file_system(), path)
                    {
                        Ok((value, path)) =>
                        {
                            *data.value_mut() = value;
                            if let Some(resolved) = path
                            {
                                let mut assets = ASSET.get_mut();
                                let manager: &mut AssetManagerIn<T, FS> = assets.manager_mut::<T, FS>();
                                self.clone().force_set_path(&mut data, Some(resolved), manager);
                            }
                            Ok(())
                        }
                        Err(_) => Err(e),
                    }
                }
                else
                {
                    Err(e)
                }
            }
        }
    }
}

impl<T, FS> FileSystemLoadSave<T, FS> for AssetIn<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async,
{
    type Output = Self;

    fn from_path_and_fn<F>(path: Option<PathBuf>, init: F) -> FileResult<Self::Output>
    where
        F: FnOnce(Option<&mut PathBuf>) -> FileResult<T>,
    {
        let Some(path) = path
        else
        {
            return Ok(Self {
                inner: Arc::new(RwLock::new(AssetValue {
                    path: None,
                    value: Some(Dirty::new(init(None)?)),
                    phantom: PhantomData,
                })),
            });
        };

        let mut path = FS::file_system().resolve_path_for::<T, _>(path);
        let mut assets = ASSET.get_mut();

        let manager: &mut AssetManagerIn<T, FS> = assets.manager_mut::<T, FS>();

        match manager.values.get(&path)
        {
            Some(asset) => match asset
            {
                AssetShared::Strong(asset) => return Ok(asset.clone()),
                AssetShared::Weak(asset) =>
                {
                    if let Some(asset) = asset.upgrade()
                    {
                        return Ok(asset);
                    }
                }
            },
            None =>
            {}
        }

        // Drop the assets lock, because init can take some time. That way multiple asset can be loaded in parallel.
        drop(assets);
        let value = init(Some(&mut path))?;

        let mut assets = ASSET.get_mut();
        let manager: &mut AssetManagerIn<T, FS> = assets.manager_mut::<T, FS>();

        match manager.values.get_mut(&path)
        {
            Some(shared) => match shared.upgrade()
            {
                Some(asset) =>
                {
                    // Asset was loaded again by someone else during init. Replace it
                    *asset.get_mut().value_mut() = value;
                    return Ok(asset);
                }
                None =>
                {}
            },
            None =>
            {}
        }

        let asset = Self {
            inner: Arc::new(RwLock::new(AssetValue {
                path: Some(path.clone()),
                value: Some(Dirty::new(value)),
                phantom: PhantomData,
            })),
        };

        let entry = match manager.default_storage
        {
            AssetStorage::Persistant => AssetShared::Strong(asset.clone()),
            AssetStorage::ReferenceCounted => AssetShared::Weak(asset.downgrade()),
        };
        manager.values.insert(path, entry);
        return Ok(asset);
    }

    fn from_path_and_value(path: Option<PathBuf>, value: T) -> Self::Output
    {
        let mut value = Some(value);
        let asset = Self::from_path_and_fn(path, |_| Ok(mem::take(&mut value).unwrap())).expect("logic bug in from_path_and_fn");
        if let Some(v) = value
        {
            *asset.get_mut().value_mut() = v;
        }
        asset
    }
}

pub struct AssetWeakIn<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async,
{
    inner: ArcWeak<RwLock<AssetValue<T, FS>>>,
}

impl<T, FS> AssetWeakIn<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async,
{
    /// Constructs a new `AssetWeakIn<T, FS>`, without allocating any memory, technically in the provided
    /// allocator.
    /// Calling [`SharedUpgrade::upgrade`] on the return value always gives [`None`].
    pub const fn new() -> Self { Self { inner: ArcWeak::new() } }

    /// Return a weak pointer of the asset from the path.
    ///
    /// If the asset is not loaded, return `Self::new()` (always return [`None`] when upgrading),
    /// even if an asset with the same path is loaded later.
    pub fn from_path<P: AsRef<Path>>(path: &P) -> Self
    {
        let path = path.as_ref();

        match ASSET.get().try_manager()
        {
            Some(manager) => match manager.values.get(path)
            {
                Some(shared) => shared.downgrade(),
                None => Self::new(),
            },
            None => Self::new(),
        }
    }

    /*
    pub fn from_path<P: AsRef<Path>>(path: &P) -> Self
    {
        let path = path.as_ref();

        let mut guard = ASSET.get_mut();
        let manager = guard.manager_mut();

        match ASSET.get_mut().manager_mut().values.get(path)
        {
            Some(shared) => shared.downgrade(),
            None =>
            {
                manager.values.insert(path.to_owned(), AssetShared::Weak(()))
            },
        }
    }
    */
}
impl<T, FS> Default for AssetWeakIn<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async,
{
    fn default() -> Self { Self::new() }
}
impl<T, FS> Clone for AssetWeakIn<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async,
{
    fn clone(&self) -> Self { Self { inner: self.inner.clone() } }
}
impl<T, FS> SharedCount for AssetWeakIn<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async,
{
    fn strong_count(&self) -> usize { self.inner.strong_count() }

    fn weak_count(&self) -> usize { self.inner.weak_count() }
}
impl<T, FS> SharedUpgrade for AssetWeakIn<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async,
{
    type Output = AssetIn<T, FS>;

    fn upgrade(&self) -> Option<Self::Output> { self.inner.upgrade().map(|inner| AssetIn { inner }) }
}

impl<T, FS> Debug for AssetWeakIn<T, FS>
where
    FS: FileSystemProvider + Async,
    T: Load + Save + Async + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult
    {
        match self.upgrade()
        {
            Some(asset) => write!(f, "{:?}", asset),
            None => write!(f, "Asset was dropped"),
        }
    }
}
