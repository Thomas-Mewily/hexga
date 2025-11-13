use super::*;

pub type SingletonSingleThread<T> = Singleton<SingleThread<LazyCell<T>>>;
pub type SingletonMultiThread<T> = Singleton<RwLock<LazyLock<T>>>;



pub struct Singleton<T>
{
    value: UnsafeCell<T>,
}
impl<T> Debug for Singleton<T> where T: Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Singleton({:?})", unsafe { self.value.as_ref_unchecked() })
    }
}
impl<T> Deref for Singleton<T> where T: Deref
{
    type Target=T::Target;
    #[inline(always)]
    #[track_caller]
    fn deref(&self) -> &Self::Target {
        unsafe { self.value.as_ref_unchecked().deref() }
    }
}
impl<T> DerefMut for Singleton<T> where T: DerefMut
{
    #[inline(always)]
    #[track_caller]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.value.as_mut_unchecked().deref_mut() }
    }
}
impl<T> Singleton<T>
{
    pub const fn from_interior_mutability(value: T) -> Self { Self { value: UnsafeCell::new(value) }}

    pub fn try_instance(&self) -> Result<<&T as TryBorrow>::Borrow, <&T as TryBorrow>::Error> where for<'a> &'a T: TryBorrow
    {
        unsafe { self.value.as_ref_unchecked().try_borrow() }
    }
    pub fn instance(&self) -> <&T as TryBorrow>::Borrow where for<'a> &'a T: TryBorrow
    {
        unsafe { self.value.as_ref_unchecked().borrow() }
    }

    pub fn try_instance_mut(&self) -> Result<<&T as TryBorrowMut>::Borrow, <&T as TryBorrowMut>::Error> where for<'a> &'a T: TryBorrowMut
    {
        unsafe { self.value.as_mut_unchecked().try_borrow_mut() }
    }
    pub fn instance_mut(&self) -> <&T as TryBorrowMut>::Borrow where for<'a> &'a T: TryBorrowMut
    {
        unsafe { self.value.as_mut_unchecked().borrow_mut() }
    }
}

unsafe impl<T> Sync for Singleton<T> where T: Sync {}
unsafe impl<T> Send for Singleton<T> where T: Send {}

impl<T> SingletonSingleThread<T>
{
    pub const fn new(f: fn() -> T) -> Self { Self::from_interior_mutability(SingleThread::new(LazyCell::new(f))) }
}
impl<T> SingletonMultiThread<T>
{
    pub const fn new(f: fn() -> T) -> Self { Self::from_interior_mutability(RwLock::new(LazyLock::new(f))) }
}
