use super::*;



#[derive(Default)]
pub struct AssetManager<T>
    where T: Async
{
    pub(crate) assets : Table<AssetData<T>>,
    pub(crate) loading_value: Option<T>,
    pub(crate) error_value: Option<T>,
}
impl<T> AssetManager<T>
    where T: Async
{
    pub fn with_loading_values() -> Self where T: Default { Self::with_values(___(), ___())}
    pub fn with_none() -> Self { Self::with_values(None, None) }
    pub fn with_values(loading_value: Option<T>, error_value: Option<T>) -> Self
    {
        Self { assets: ___(), loading_value, error_value }
    }
}

impl<'a,T> IntoIterator for &'a AssetManager<T>
    where T: Async
{
    type Item=&'a AssetData<T>;
    type IntoIter=Iter<'a,T>;

    fn into_iter(self) -> Self::IntoIter {
         Iter{ iter: self.assets.iter() }
    }
}
#[derive(Debug)]
pub struct Iter<'a,T>
    where T: Async
{
    iter: table::Iter<'a,AssetData<T>>
}
impl<'a,T> Clone for Iter<'a,T> where T: Async
{
    fn clone(&self) -> Self {
        Self { iter: self.iter.clone() }
    }
}

impl<'a,T> Iterator for Iter<'a,T>
    where T: Async
{
    type Item=&'a AssetData<T>;
    fn next(&mut self) -> Option<Self::Item>
    {
        let (_entry,value) = self.iter.next()?;
        Some(value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
impl<'a,T> FusedIterator for Iter<'a,T> where T: Async, table::Iter<'a,AssetData<T>>: FusedIterator  {}
impl<'a,T> ExactSizeIterator for Iter<'a,T> where T: Async, table::Iter<'a,AssetData<T>>: ExactSizeIterator { fn len(&self) -> usize { self.iter.len() } }


impl<T> AssetManager<T>
    where T: Async
{
    pub(crate) fn get_or_init_with_state<P,F>(&mut self, path: P, create_state: F) -> Asset<T>
        where P: AsRefPath, F: FnOnce(&path) -> AssetState<T>
    {
        let path = path.as_ref();
        if let Some(asset) = self.assets.get_from_key(path.as_str())
        {
            return Asset::from_asset_data(asset);
        }

        let data = AssetData
        {
            state: create_state(path),
            manager_ptr: unsafe { NonNull::new_unchecked(self as *mut Self) },
            id: TableID::NULL,
            count: 0,
        };
        let id = self.assets.insert(path.to_owned().into(), data).unwrap();
        self.assets[id].id = id;
        Asset::from_asset_data(&self.assets[id])
    }

    /// Sets the asset at `path` to an error state, creating it if needed.
    ///
    /// Returns a handle to the resulting asset.
    pub fn update_or_create_with_error<P>(&mut self, path: P, error: IoError) -> Asset<T>
        where P: AsRefPath
    {
        let path = path.as_ref();
        match self.assets.get_mut_from_key(path.as_str())
        {
            Some(asset) =>
            {
                asset.state = AssetState::Error(error);
                Asset::from_asset_data(asset)
            },
            None => self.get_or_init_with_state(path, |path| AssetState::Error(error)),
        }
    }


    /// Sets the asset at `path` to the given value, and create it if needed.
    ///
    /// Returns a handle to the resulting asset.
    pub fn update_or_create_with_value<P>(&mut self, path: P, value: T) -> Asset<T>
        where P: AsRefPath
    {
        let path = path.as_ref();
        match self.assets.get_mut_from_key(path.as_str())
        {
            Some(asset) =>
            {
                asset.replace_value(value);
                Asset::from_asset_data(asset)
            },
            None => self.get_or_init_with_state(path, |path| AssetState::Loaded(value)),
        }
    }

    pub fn get_or_load<P>(&mut self, path: P) -> Asset<T>
        where P: AsRefPath, T: Load
    {
        let mut was_loaded_from_file = false;
        let mut asset = self.get_or_init_with_state(path,
            |path|
            {
                was_loaded_from_file = true;
                match T::load_from_disk(path)
                {
                    Ok(asset) => AssetState::Loaded(asset),
                    Err(err) => AssetState::Error(err),
                }
            }
        );
        if !was_loaded_from_file // avoid trying to load the asset twice
            && asset.is_error()
        {
            let _ = asset.hot_reload();
        }
        asset
    }

    pub fn iter(&self) -> Iter<'_,T> { (&self).into_iter() }


    pub fn loading_value(&self) -> Option<&T> { self.loading_value.as_ref() }
    pub fn error_value(&self) -> Option<&T> { self.error_value.as_ref() }

    pub fn set_loading_and_error_value(&mut self, loading_value: T, error_value: T) -> &mut Self { self.set_loading_value(loading_value).set_error_value(error_value) }
    pub fn set_loading_value(&mut self, value: T) -> &mut Self { self.loading_value = Some(value); self }
    pub fn set_error_value(&mut self, value: T) -> &mut Self { self.error_value = Some(value); self }
}
