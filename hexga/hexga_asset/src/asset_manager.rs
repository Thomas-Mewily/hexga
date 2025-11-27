use super::*;



#[allow(non_upper_case_globals)]
pub(crate) static Assets : LazyLock<AssetsManagerUntyped> = LazyLock::new(|| AssetsManagerUntyped::default());

#[derive(Default)]
pub(crate) struct AssetsManagerUntyped
{
    //path_resolver: PathResolver,
    assets: RwLock<HashMap<TypeId, Arc<AnyAsync>>>,
}

/*
pub struct PathResolver
{
    target: PathBuf,
    name: HashMap<PathBuf, PathResolver>,
}
*/


impl AssetsManagerUntyped
{
    pub fn manager<T>(&self) -> AssetManager<T> where T: Async
    {
        let type_id = TypeId::of::<T>();
        {
            let assets_read = self.assets.read().unwrap();
            if let Some(any_manager) = assets_read.get(&type_id)
            {
                let asset_manager = any_manager.downcast_ref::<AssetManager<T>>().unwrap();
                return asset_manager.clone();
            }
        }

        let mut assets_write = self.assets.write().unwrap();
        let manager = AssetManager::<T>{ inner: Arc::new(RwLock::new(AssetManagerInner::new())) };
        assets_write.insert(type_id, Arc::new(manager.clone()));
        manager
    }
}

pub(crate) type AssetManagerArcRwLock<T> = Arc<RwLock<AssetManagerInner<T>>>;
pub struct AssetManager<T> where T: Async
{
    pub(crate) inner: AssetManagerArcRwLock<T>,
}
impl<T> Clone for AssetManager<T> where T: Async
{
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}
// IDEA : make a feature flag for non async loading ?
impl<T> AssetManager<T> where T: Async
{
    pub(crate) fn load<P>(&self, path: P) -> Asset<T> where P: AsRef<Path>, T: Load
    {
        self.load_or_create(&path, ||
            {
                match T::load(&path)
                {
                    Ok(value) => AssetState::Loaded(value),
                    Err(err) => AssetState::Error(err),
                }
            })
    }

    pub(crate) fn load_or_create<P, F, O>(&self, path: P, init: F) -> Asset<T>
        where
        P: AsRef<Path>,
        F: FnOnce() -> O,
        O: Into<AssetInit<T>>,
        T: Load
    {
        let mut need_to_be_loaded = false;
        let asset = self.get_or_generate::<&Path,_,_,_>(AssetPersistance::Persistant(path.as_ref()),|| { need_to_be_loaded = true; AssetInit::with_state(AssetState::Loading) });

        let need_to_be_loaded = need_to_be_loaded || match asset.state().deref()
        {
            AssetState::Error(io_error) => !io_error.kind.is_encoding(),
            _ => false,
        };
        if need_to_be_loaded
        {
            self.update_or_create::<P,_,_>(AssetPersistance::Persistant(path), init());
        }
        asset
    }

    /*
    pub(crate) fn load_or_create<P, F, O>(&self, path: P, init: F) -> Asset<T>
    where
        P: AsRef<Path>,
        F: FnOnce() -> O + 'static,
        O: Into<AssetInit<T>>,
        T: Load
    {
        let mut need_to_be_loaded = false;
        let asset = self.get_or_generate::<&Path,_,_,_>(AssetPersistance::Persistant(path.as_ref()),|| { need_to_be_loaded = true; AssetInit::with_state(AssetState::Loading) });

        if need_to_be_loaded || asset.state().is_error()
        {
            let path = path.as_ref().to_owned();

            Io.load_bytes_async(path.clone(), |result: Result<Vec<u8>, IoError>|
                {
                    let extension = path.extension_or_empty();
                    match result
                    {
                        Ok(bytes) => {
                            let load_result = T::load_from_bytes(&bytes, extension).map_err(|e| IoError::new(path.clone(), e).when_reading());
                            Asset::<T>::update_or_create(path, AssetInit::with_state(load_result.into()));
                        },
                        Err(err) =>
                        {
                            Self::_load_or_create_try_extensions_async( 0, path.clone(), init);
                        },
                    }
                }
            );
        }
        asset
    }

    pub(crate) fn _load_or_create_try_extensions_async<F, O>(extension_index: usize, path: PathBuf, init: F)
    where
        T: Load,
        F: FnOnce() -> O + 'static,
        O: Into<AssetInit<T>>,
    {
        let Some(extension) = T::load_extensions().nth(extension_index) else
        {
            Asset::<T>::update_or_create(&path, init().into());
            return;
        };
        let next_path = path.with_extension(extension);

        Io.load_bytes_async(next_path, move |result| {
            match result
            {
                Ok(bytes) => {
                    let load_result = T::load_from_bytes(&bytes, extension).map_err(|e| IoError::new(path.clone(), e).when_reading());
                    Asset::<T>::update_or_create(path, AssetInit::with_state(load_result.into()));
                },
                Err(_) => {
                    Self::_load_or_create_try_extensions_async(extension_index+1, path, init);
                },
            }
        });
    }
    */
}
impl<T> AssetManager<T> where T: Async
{
    pub(crate) fn update_or_create<P,Persis,I>(&self, persistance: Persis, value: I) -> Asset<T>
        where
        P: AsRef<Path>,
        Persis: Into<AssetPersistance<P>>,
        I: Into<AssetInit<T>>
    {
        let init  = value.into();
        let v = Self::get_or_generate(&self, persistance, || AssetInit { state: AssetState::Loading, lifetime: init.lifetime });
        v.inner.write().unwrap().state = init.state;
        v
    }

    pub(crate) fn get_or_generate<P,Persis,F,O>(&self, persistance: Persis, init: F) -> Asset<T>
        where
        P: AsRef<Path>,
        Persis: Into<AssetPersistance<P>>,
        F: FnOnce() -> O,
        O: Into<AssetInit<T>>
    {
        let persistance = persistance.into();
        let path = match &persistance
        {
            AssetPersistance::Generated =>
            {
                let AssetInit { state, lifetime: _ } = init().into();
                return Asset::_new(state, AssetPersistance::Generated);
            }
            AssetPersistance::Persistant(path) => path.as_ref(),
        };

        let r = self.inner.read().unwrap();

        if let Some(storage) = r.values.get(path)
        {
            match storage
            {
                AssetStorage::Persistant(asset) => return asset.clone(),
                AssetStorage::ReferenceCounted(asset_weak) => if let Some(asset) = asset_weak.upgrade()
                {
                    return asset.clone();
                }
            }
        }

        // Not loaded or weak and invalid
        drop(r);

        let mut w = self.inner.write().unwrap();
        let AssetInit { state: value, lifetime } = init().into();
        let asset = Asset::_new(value, AssetPersistance::Persistant(path.to_owned()));

        let storage = match lifetime
        {
            AssetLifetime::ReferenceCounted => asset.clone().into(),
            AssetLifetime::Persistant => Asset::downgrade(&asset).into(),
        };
        w.values.insert(path.to_owned(), storage);
        asset
    }


    pub fn all(&self) -> Vec<Asset<T>>
    {
        self.inner.read().unwrap().values.values().filter_map(|v| v.upgrade()).collect()
    }
    pub fn iter(&self) -> AssetIter<T> { AssetIter { inner: self.all().into_iter() } }

    pub fn error_value(&self) -> AssetWeak<T> { self.inner.read().unwrap().error.clone() }
    pub fn set_error_value(&self, value: AssetWeak<T>) -> &Self { self.inner.write().unwrap().error = value; self }

    pub fn loading_value(&self) -> AssetWeak<T> { self.inner.read().unwrap().loading.clone() }
    pub fn set_loading_value(&self, value: AssetWeak<T>) -> &Self { self.inner.write().unwrap().loading = value; self }

}

#[derive(Debug)]
pub(crate) struct AssetManagerInner<T> where T: Async
{
    pub(crate) values: HashMap<PathBuf, AssetStorage<T>>,
    pub(crate) loading: AssetWeak<T>,
    pub(crate) error: AssetWeak<T>,
}
impl<T> AssetManagerInner<T> where T: Async
{
    pub(crate) fn new() -> Self
    {
        Self { values: ___(), loading: ___(), error: ___() }
    }
}



#[derive(Debug)]
pub enum AssetStorage<T> where T: Async
{
    Persistant(Asset<T>),
    ReferenceCounted(AssetWeak<T>),
}
impl<T> Clone for AssetStorage<T> where T: Async
{
    fn clone(&self) -> Self {
        match self {
            Self::Persistant(arg0) => Self::Persistant(arg0.clone()),
            Self::ReferenceCounted(arg0) => Self::ReferenceCounted(arg0.clone()),
        }
    }
}
impl<T> AssetStorage<T> where T: Async
{
    pub fn lifetime(&self) -> AssetLifetime
    {
        match self
        {
            AssetStorage::Persistant(_asset) => AssetLifetime::Persistant,
            AssetStorage::ReferenceCounted(_asset_weak) => AssetLifetime::ReferenceCounted,
        }
    }

    pub fn upgrade(&self) -> Option<Asset<T>>
    {
        match self
        {
            AssetStorage::Persistant(asset) => Some(asset.clone()),
            AssetStorage::ReferenceCounted(asset_weak) => asset_weak.upgrade(),
        }
    }

    pub fn downgrade(&self) -> AssetWeak<T>
    {
        match self
        {
            AssetStorage::Persistant(asset) => asset.downgrade(),
            AssetStorage::ReferenceCounted(asset_weak) => asset_weak.clone(),
        }
    }
}
impl<T> From<Asset<T>> for AssetStorage<T> where T: Async
{
    fn from(value: Asset<T>) -> Self {
        Self::Persistant(value)
    }
}
impl<T> From<AssetWeak<T>> for AssetStorage<T> where T: Async
{
    fn from(value: AssetWeak<T>) -> Self {
        Self::ReferenceCounted(value)
    }
}


impl<'a,T> IntoIterator for &'a AssetManager<T> where T: Async
{
    type Item=Asset<T>;
    type IntoIter=AssetIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
pub struct AssetIter<T> where T: Async
{
    inner: std::vec::IntoIter<Asset<T>>,
}
impl<T: Async> Iterator for AssetIter<T>
{
    type Item = Asset<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}