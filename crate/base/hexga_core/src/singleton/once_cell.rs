use super::*;

/// Single threaded read only singleton, where the value is initialized at compile time.
pub type SingletonOnceCell<T> = SingletonOf<SingleThreadCell<OnceCell<T>>>;
/// Single threaded read only singleton, where the value is initialized from a static fn / lambda at runtime.
pub type SingletonOnceLazyCell<T> = SingletonOf<SingleThreadCell<LazyLock<T>>>;

impl<T> SingletonOnceCell<T>
{
    pub const fn uninit() -> Self { Self::from_guard(SingleThreadCell::new(OnceCell::new())) }

    pub const fn new() -> Self { Self::uninit() }
}

impl<T> Debug for SingletonOnceCell<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult
    {
        match self.try_get()
        {
            Ok(guard) => write!(f, "{:?}", guard),
            Err(e) => write!(
                f,
                "SingletonOnceCell<{}> can't be read: {:?}",
                std::any::type_name::<T>(),
                e
            ),
        }
    }
}

impl<T> Guarded<T> for SingletonOnceCell<T>
{
    type Guard<'a>
        = Ref<'a, T>
    where
        Self: 'a;

    #[track_caller]
    fn get<'a>(&'a self) -> Self::Guard<'a>
    {
        let guard = self.guarded.get();
        if guard.get().is_none()
        {
            panic!(
                "SingletonOnceCell<{}> not initialized",
                std::any::type_name::<T>()
            );
        }
        Ref::map(guard, |cell: &OnceCell<T>| cell.get().unwrap())
    }

    type Error<'a>
        = SingleThreadError
    where
        Self: 'a;

    fn try_get<'a>(&'a self) -> Result<Self::Guard<'a>, Self::Error<'a>>
    {
        let guard = self.guarded.try_get()?;
        match guard.get()
        {
            Some(_) => Ok(Ref::map(guard, |cell: &OnceCell<T>| cell.get().unwrap())),
            None => Err(SingleThreadError::NotInit(NotInitError)),
        }
    }
}

impl<T> SingletonOnceable<T> for SingletonOnceCell<T>
{
    fn init_from_fn<'a, F>(&'a self, init: F) -> Result<Self::Guard<'a>, F>
    where
        F: FnOnce() -> T,
    {
        match self.guarded.try_get()
        {
            Ok(guard) =>
            {
                if guard.get().is_none()
                {
                    guard.set(init()).map_err(|_| unreachable!())?;
                }
                Ok(Ref::map(guard, |cell: &OnceCell<T>| cell.get().unwrap()))
            }
            Err(_) => Err(init),
        }
    }
}
impl_singleton_methods_get!(SingletonOnceCell);

impl<T> SingletonOnceLazyCell<T>
{
    pub const fn new(f: fn() -> T) -> Self
    {
        Self::from_guard(SingleThreadCell::new(LazyLock::new(f)))
    }

    pub const fn default() -> Self
    where
        T: Default,
    {
        Self::from_guard(SingleThreadCell::new(LazyLock::new(___)))
    }
}

impl<T> Debug for SingletonOnceLazyCell<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult
    {
        match self.try_get()
        {
            Ok(guard) => write!(f, "{:?}", guard),
            Err(e) => write!(
                f,
                "SingletonOnceLazyCell<{}> can't be read: {:?}",
                std::any::type_name::<T>(),
                e
            ),
        }
    }
}

impl<T> Guarded<T> for SingletonOnceLazyCell<T>
{
    type Guard<'a>
        = Ref<'a, T>
    where
        Self: 'a;

    #[track_caller]
    fn get<'a>(&'a self) -> Self::Guard<'a> { Ref::map(self.guarded.get(), |v| v.deref()) }

    type Error<'a>
        = SingleThreadError
    where
        Self: 'a;

    fn try_get<'a>(&'a self) -> Result<Self::Guard<'a>, Self::Error<'a>>
    {
        Ok(Ref::map(self.guarded.try_get()?, |v| v.deref()))
    }
}
impl_singleton_methods_get!(SingletonOnceLazyCell);
