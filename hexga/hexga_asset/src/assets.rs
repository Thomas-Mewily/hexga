use super::*;


singleton_declare_multi_thread!(pub Assets, AssetsManagerUntyped, ASSETS, Default::default());


impl SingletonReplace for Assets
{
    fn replace(value: Option<<Self as SingletonRef>::Target>) -> SingletonResult {
        let cell: &::std::sync::RwLock<Option<AssetsManagerUntyped>> =
            ASSETS.get_or_init(|| ::std::sync::RwLock::new(None));

        let mut guard = cell.write().map_err(|_| ())?;
        if guard.is_some() {
            return Err(()); // The Asset can't be initialized twice
        }
        *guard = value;
        Ok(())
    }
}

#[derive(Default)]
pub struct AssetsManagerUntyped
{
    managers: HashMap<TypeId, Pin<Box<dyn AsyncAny>>>
}
impl AssetsManagerUntyped
{
    /// Creates a new type-specific asset manager if it does not exist using the [`Default`] value of T for loading and error value.
    pub fn update_or_create_manager<T>(&mut self) -> &mut AssetManager<T> where T: Async + Default
    {
        self.update_or_create_manager_with_values::<T>(Some(T::___()), Some(T::___()))
    }

    /// Creates a new type-specific asset manager if it does not exist without loading and error value.
    pub fn update_or_create_manager_with_none<T>(&mut self) -> &mut AssetManager<T> where T: Async
    {
        self.update_or_create_manager_with_values::<T>(None, None)
    }

    /// or updates the existing manager's default and error values.
    ///
    /// - If `loading_value` is `Some`, updates the manager's `loading_value`.
    /// - If `error_value` is `Some`, updates the manager's `error_value`.
    pub fn update_or_create_manager_with_values<T>(&mut self, loading_value: Option<T>, error_value: Option<T>) -> &mut AssetManager<T>
        where T: Async
    {
        let type_id = TypeId::of::<T>();

        if self.managers.get_mut(&type_id).is_none()
        {
            self.managers.insert(type_id, Box::pin(AssetManager::<T>::with_none()));
        }

        let pinned_any: &mut Pin<Box<dyn AsyncAny>> = self.managers.get_mut(&type_id).unwrap();

        // SAFELY get a mutable reference to the inner value using unsafe
        // Because Pin guarantees the value won't move, this is safe
        let manager: &mut AssetManager<T> = unsafe {
            let raw: *mut dyn Any = Pin::as_mut(pinned_any).get_unchecked_mut();
            &mut *(raw as *mut AssetManager<T>)
        };

        if let Some(v) = loading_value
        {
            manager.set_loading_value(v);
        }
        if let Some(v) = error_value
        {
            manager.set_error_value(v);
        }
        manager
    }

    pub fn manager<T>(&mut self) -> &AssetManager<T>
        where T: Async
    {
        self.update_or_create_manager_with_none::<T>()
    }

    pub fn manager_mut<T>(&mut self) -> &mut AssetManager<T>
        where T: Async
    {
        self.update_or_create_manager_with_none::<T>()
    }
}



// pub struct AssetUntyped
// {
//     type_id: std::any::TypeId,
//     id: TableID,
// }