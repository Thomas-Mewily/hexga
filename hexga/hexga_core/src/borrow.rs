use super::*;

pub trait TryBorrow : Sized
{
    type Borrow;
    type Error: Debug;
    #[inline(always)]
    #[track_caller]
    fn borrow(self) -> Self::Borrow { self.try_borrow().expect("can't borrow") }
    fn try_borrow(self) -> Result<Self::Borrow, Self::Error>;
}
pub trait TryBorrowMut : Sized
{
    type Borrow;
    type Error: Debug;
    #[inline(always)]
    #[track_caller]
    fn borrow_mut(self) -> Self::Borrow { self.try_borrow_mut().expect("can't borrow mut") }
    fn try_borrow_mut(self) -> Result<Self::Borrow, Self::Error>;
}

/*
impl<'a, 'b, T> TryBorrow for &'a &'b T
{
    type Borrow=&'a &'b T;
    type Error=();

    fn try_borrow(self) -> Result<Self::Borrow, Self::Error> {
        Ok(self)
    }
    fn borrow(self) -> Self::Borrow {
        self
    }
}
*/




impl<'a,T> TryBorrow for &'a std::cell::RefCell<T>
{
    type Borrow=std::cell::Ref<'a,T>;
    type Error = std::cell::BorrowError;

    #[inline(always)]
    #[track_caller]
    fn borrow(self) -> Self::Borrow {
        self.borrow()
    }
    fn try_borrow(self) -> Result<Self::Borrow, Self::Error> {
        self.try_borrow()
    }
}
impl<'a,T> TryBorrowMut for &'a std::cell::RefCell<T>
{
    type Borrow = std::cell::RefMut<'a,T>;
    type Error = std::cell::BorrowMutError;

    #[inline(always)]
    #[track_caller]
    fn borrow_mut(self) -> Self::Borrow {
        self.borrow_mut()
    }
    fn try_borrow_mut(self) -> Result<Self::Borrow, Self::Error> {
        self.try_borrow_mut()
    }
}
impl<'a,T> TryBorrow for &'a std::sync::Mutex<T>
{
    type Borrow = std::sync::MutexGuard<'a,T>;
    type Error = std::sync::PoisonError<std::sync::MutexGuard<'a,T>>;

    fn try_borrow(self) -> Result<Self::Borrow, Self::Error> {
        self.lock()
    }
}
impl<'a,T> TryBorrowMut for &'a std::sync::Mutex<T>
{
    type Borrow=std::sync::MutexGuard<'a,T>;
    type Error = std::sync::PoisonError<std::sync::MutexGuard<'a,T>>;

    fn try_borrow_mut(self) -> Result<Self::Borrow, Self::Error> {
        self.lock()
    }
}
impl<'a,T> TryBorrow for &'a std::sync::RwLock<T>
{
    type Borrow = std::sync::RwLockReadGuard<'a,T>;
    type Error = std::sync::PoisonError<std::sync::RwLockReadGuard<'a,T>>;

    fn try_borrow(self) -> Result<Self::Borrow, Self::Error> {
        self.read()
    }
}
impl<'a,T> TryBorrowMut for &'a std::sync::RwLock<T>
{
    type Borrow=std::sync::RwLockWriteGuard<'a,T>;
    type Error = std::sync::PoisonError<std::sync::RwLockWriteGuard<'a,T>>;

    fn try_borrow_mut(self) -> Result<Self::Borrow, Self::Error> {
        self.write()
    }
}

/*
// unstable library
impl<'a,T> TryBorrow for &'a std::sync::nonpoison::Mutex<T>
{
    type Borrow = std::sync::nonpoison::MutexGuard<'a,T>;
    type Error = std::sync::nonpoison::PoisonError<std::sync::nonpoison::MutexGuard<'a,T>>;

    fn try_borrow(self) -> Result<Self::Borrow, Self::Error> {
        self.lock()
    }
}
impl<'a,T> TryBorrowMut for &'a std::sync::nonpoison::Mutex<T>
{
    type Borrow=std::sync::nonpoison::MutexGuard<'a,T>;
    type Error = std::sync::nonpoison::PoisonError<std::sync::nonpoison::MutexGuard<'a,T>>;

    fn try_borrow_mut(self) -> Result<Self::Borrow, Self::Error> {
        self.lock()
    }
}
impl<'a,T> TryBorrow for &'a std::sync::nonpoison::RwLock<T>
{
    type Borrow = std::sync::nonpoison::RwLockReadGuard<'a,T>;
    type Error = std::sync::nonpoison::PoisonError<std::sync::nonpoison::RwLockReadGuard<'a,T>>;

    fn try_borrow(self) -> Result<Self::Borrow, Self::Error> {
        self.read()
    }
}
impl<'a,T> TryBorrowMut for &'a std::sync::nonpoison::RwLock<T>
{
    type Borrow=std::sync::nonpoison::RwLockWriteGuard<'a,T>;
    type Error = std::sync::nonpoison::PoisonError<std::sync::nonpoison::RwLockWriteGuard<'a,T>>;

    fn try_borrow_mut(self) -> Result<Self::Borrow, Self::Error> {
        self.write()
    }
}

+ same for for SyncUnsafeCell<T>
*/