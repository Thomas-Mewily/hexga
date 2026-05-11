use super::*;

/// Single threaded singleton, where the value is initialized at compile time.
pub type SingletonCell<T> = SingletonOf<SingleThreadCell<Identity<T>>>;
/// Single threaded singleton, where the value is initialized from a static fn / lambda at runtime.
pub type SingletonLazyCell<T> = SingletonOf<SingleThreadCell<LazyCell<T>>>;
/// Single threaded singleton, where the value should be manually initialized at runtime.
pub type SingletonOptionCell<T> = SingletonOf<SingleThreadCell<Option<T>>>;



impl<T> SingletonCell<T> {
    pub const fn new(value: T) -> Self 
    {
        Self::from_guard(SingleThreadCell::new(Identity::new(value)))
    }
}

impl<T> Debug for SingletonCell<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.try_get() {
            Ok(v) => write!(f, "{:?}", v.deref()),
            Err(e) => write!(f, "SingletonCell<{}> can't be read: {:?}", std::any::type_name::<T>(), e),
        }
    }
}

impl<T> Guarded<T> for SingletonCell<T> {
    type Guard<'a> = Ref<'a, T> where Self: 'a;
    
    #[track_caller]
    fn get<'a>(&'a self) -> Self::Guard<'a> {
        Ref::map(self.guarded.get(), |v| v.as_ref())
    }
}
impl<T> TryGuarded<T> for SingletonCell<T> {
    type Error<'a> = SingleThreadError where Self: 'a;
    
    fn try_get<'a>(&'a self) -> Result<Self::Guard<'a>, Self::Error<'a>> {
        Ok(Ref::map(self.guarded.try_get()?, |v| v.as_ref()))
    }
}

impl<T> GuardedMut<T> for SingletonCell<T> {
    type GuardMut<'a> = RefMut<'a, T> where Self: 'a;
    
    #[track_caller]
    fn get_mut<'a>(&'a self) -> Self::GuardMut<'a> {
        RefMut::map(self.guarded.get_mut(), |v| v.as_mut())
    }
}

impl<T> TryGuardedMut<T> for SingletonCell<T> {
    type Error<'a> = SingleThreadMutError where Self: 'a;
    
    fn try_get_mut<'a>(&'a self) -> Result<Self::GuardMut<'a>, Self::Error<'a>> {
        Ok(RefMut::map(self.guarded.try_get_mut()?, |v| v.as_mut()))
    }
}
impl_singleton_methods!(SingletonCell);



impl<T> SingletonLazyCell<T> {
    pub const fn new(f: fn() -> T) -> Self {
        Self::from_guard(SingleThreadCell::new(LazyCell::new(f)))
    }
}

impl<T> Debug for SingletonLazyCell<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.try_get() {
            Ok(v) => write!(f, "{:?}", v.deref()),
            Err(e) => write!(f, "SingletonLazyCell<{}> can't be read: {:?}", std::any::type_name::<T>(), e),
        }
    }
}

impl<T> Guarded<T> for SingletonLazyCell<T> {
    type Guard<'a> = Ref<'a, T> where Self: 'a;
    
    #[track_caller]
    fn get<'a>(&'a self) -> Self::Guard<'a> {
        Ref::map(self.guarded.get(), |v| v.deref())
    }
}

impl<T> TryGuarded<T> for SingletonLazyCell<T> {
    type Error<'a> = SingleThreadError where Self: 'a;
    
    fn try_get<'a>(&'a self) -> Result<Self::Guard<'a>, Self::Error<'a>> {
        Ok(Ref::map(self.guarded.try_get()?, |v| v.deref()))
    }
}
impl<T> GuardedMut<T> for SingletonLazyCell<T> {
    type GuardMut<'a> = RefMut<'a, T> where Self: 'a;
    
    #[track_caller]
    fn get_mut<'a>(&'a self) -> Self::GuardMut<'a> {
        RefMut::map(self.guarded.get_mut(), |v| v.deref_mut())
    }
}
impl<T> TryGuardedMut<T> for SingletonLazyCell<T> {
    type Error<'a> = SingleThreadMutError where Self: 'a;
    
    fn try_get_mut<'a>(&'a self) -> Result<Self::GuardMut<'a>, Self::Error<'a>> {
        Ok(RefMut::map(self.guarded.try_get_mut()?, |v| v.deref_mut()))
    }
}
impl_singleton_methods!(SingletonLazyCell);



impl<T> SingletonOptionCell<T> 
{
    /// Creates an uninitialized singleton (None).
    pub const fn uninit() -> Self {
        Self::new(None)
    }

    /// Creates a pre-initialized singleton with the given value.
    pub const fn from_value(value: T) -> Self {
        Self::new(Some(value))
    }

    /// Creates a singleton with the given option state.
    pub const fn new(value : Option<T>) -> Self {
        Self::from_guard(SingleThreadCell::new(value))
    }
}

impl<T> SingletonOptionable<T> for SingletonOptionCell<T> 
{
    fn init_from_fn<F>(&self, init: F) -> Result<RefMut<'_,T>, (SingleThreadMutError, F)> 
        where F: FnOnce() -> T
    {
        match self.guarded.try_get_mut() {
            Ok(mut guard) => {
                if guard.is_none() 
                {
                    *guard = Some(init());
                }
                Ok(guard.guard_map_mut(|v| v.as_mut().unwrap()))
            }
            Err(e) => {
                Err((e, init))
            }
        }
    }

    fn swap(&self, other: &mut Option<T>) -> Result<(), SingleThreadMutError> {
        match self.guarded.try_get_mut() {
            Ok(mut guard) => {
                std::mem::swap(&mut *guard, other);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

impl<T> Debug for SingletonOptionCell<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.try_get() {
            Ok(v) => write!(f, "Some({:?})", v.deref()),
            Err(e) => write!(f, "SingletonOptionCell<{}> can't be read: {:?}", std::any::type_name::<T>(), e),
        }
    }
}

impl<T> Guarded<T> for SingletonOptionCell<T> {
    type Guard<'a> = Ref<'a, T> where Self: 'a;
    
    #[track_caller]
    fn get<'a>(&'a self) -> Self::Guard<'a> 
    {
        self.guarded.get().guard_map(|g| g.as_ref().unwrap_or_else(|| 
            match self.try_get()
            {
                Ok(_) => panic!("SingletonOptionCell<{}> can't be read", std::any::type_name::<T>()),
                Err(e) => panic!("SingletonOptionCell<{}> can't be read: {:?}", std::any::type_name::<T>(), e),
            }
        ))
    }
}

impl<T> TryGuarded<T> for SingletonOptionCell<T> {
    type Error<'a> = SingleThreadError where Self: 'a;
    
    fn try_get<'a>(&'a self) -> Result<Self::Guard<'a>, Self::Error<'a>> {
        let guard: Ref<'_, Option<T>> = self.guarded.try_get()?;
        match Ref::filter_map(guard, |opt: &Option<T>| opt.as_ref()) {
            Ok(guard) => Ok(guard),
            Err(_) => Err(SingleThreadError::NotInit(NotInitError)),
        }
    }
}

impl<T> GuardedMut<T> for SingletonOptionCell<T> {
    type GuardMut<'a> = RefMut<'a, T> where Self: 'a;
    
    #[track_caller]
    fn get_mut<'a>(&'a self) -> Self::GuardMut<'a> {
        RefMut::filter_map(self.guarded.get_mut(), |opt| opt.as_mut())
            .unwrap_or_else(|_| 
                match self.try_get()
                {
                    Ok(_) => panic!("SingletonOptionCell<{}> can't be written", std::any::type_name::<T>()),
                    Err(e) => panic!("SingletonOptionCell<{}> can't be written: {:?}", std::any::type_name::<T>(), e),
                }
            )
    }
}

impl<T> TryGuardedMut<T> for SingletonOptionCell<T> {
    type Error<'a> = SingleThreadMutError where Self: 'a;
    
    fn try_get_mut<'a>(&'a self) -> Result<Self::GuardMut<'a>, Self::Error<'a>> {
        let guard: RefMut<'_, Option<T>> = self.guarded.try_get_mut()?;
        match RefMut::filter_map(guard, |opt: &mut Option<T>| opt.as_mut()) {
            Ok(guard) => Ok(guard),
            Err(_) => Err(SingleThreadMutError::NotInit(NotInitError)),
        }
    }
}
impl_singleton_methods!(SingletonOptionCell);
