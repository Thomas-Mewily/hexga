use std::{fmt::Debug, ops::{Deref, DerefMut}};

pub trait SingletonRead
{
    type Error:  Debug;
    type Target;
    type ReadGuard : Deref<Target=Self::Target>;

    fn try_read() -> Result<Self::ReadGuard,Self::Error>;
    #[inline(always)]
    #[track_caller]
    fn read() -> Self::ReadGuard { Self::try_read().expect("not init") }

    fn is_init() -> bool { Self::try_read().is_ok() }
    fn is_not_init() -> bool { !Self::is_init() }
}
pub trait SingletonWrite: SingletonRead
{
    type Error:  Debug;
    type WriteGuard : DerefMut<Target=Self::Target>;
    fn try_write() -> Result<Self::WriteGuard, <Self as SingletonWrite>::Error>;
    #[inline(always)]
    #[track_caller]
    fn write() -> Self::WriteGuard { Self::try_write().expect("not init") }
}


#[macro_export]
macro_rules! singleton_single_thread {
    ($(#[$attr:meta])* $vis:vis $wrapper:ident, $inner:ty, $static_name:ident, $init:expr) => {
        static $static_name: $crate::SingletonSingleThread<$inner> = $crate::SingletonSingleThread::new($init);

        $(#[$attr])*
        $vis struct $wrapper;

        impl $crate::SingletonRead for $wrapper {
            type Error = <$crate::cell::SingleThread<$inner> as $crate::guard::TryReadGuard>::Error<'static>;
            type Target = $inner;
            type ReadGuard = <$crate::cell::SingleThread<$inner> as $crate::guard::ReadGuard>::ReadGuard<'static>;

            fn try_read() -> Result<Self::ReadGuard, Self::Error>
            {
                <$crate::SingletonSingleThread<$inner> as $crate::guard::TryReadGuard>::try_read(&$static_name)
                    .map(|v| $crate::guard::ReferenceReadGuard::new(&**v.value))
            }
        }

        impl $crate::SingletonWrite for $wrapper {
            type Error = <$crate::cell::SingleThread<$inner> as $crate::guard::TryWriteGuard>::Error<'static>;
            type WriteGuard = <$crate::cell::SingleThread<$inner> as $crate::guard::WriteGuard>::WriteGuard<'static>;

            fn try_write() -> Result<Self::WriteGuard, <Self as $crate::SingletonWrite>::Error>
            {
                <$crate::SingletonSingleThread<$inner> as $crate::guard::TryWriteGuard>::try_write(&$static_name)
                    .map(|v| $crate::guard::ReferenceWriteGuard::new(&mut **v.value))
            }
        }

        impl std::ops::Deref for $wrapper {
            type Target = $inner;
            fn deref(&self) -> &Self::Target {
                <$wrapper as $crate::SingletonRead>::read().value
            }
        }

        impl std::ops::DerefMut for $wrapper {
            fn deref_mut(&mut self) -> &mut Self::Target {
                <$wrapper as $crate::SingletonWrite>::write().value
            }
        }
    };
}

#[macro_export]
macro_rules! singleton_single_thread_access {
    ($(#[$attr:meta])* $vis:vis $wrapper:ident, $inner:ty, $try_read:block, $try_write:block) => {

        $(#[$attr])*
        $vis struct $wrapper;

        impl $crate::SingletonRead for $wrapper {
            type Error = <$crate::cell::SingleThread<$inner> as $crate::guard::TryReadGuard>::Error<'static>;
            type Target = $inner;
            type ReadGuard = $crate::guard::ReferenceReadGuard<'static, $inner>;

            fn try_read() -> Result<Self::ReadGuard, Self::Error> {
                let reference = $try_read?;
                Ok($crate::guard::ReferenceReadGuard::new(reference))
            }
        }

        impl $crate::SingletonWrite for $wrapper {
            type Error = <$crate::cell::SingleThread<$inner> as $crate::guard::TryWriteGuard>::Error<'static>;
            type WriteGuard = <$crate::cell::SingleThread<$inner> as $crate::guard::WriteGuard>::WriteGuard<'static>;

            fn try_write() -> Result<Self::WriteGuard, <Self as $crate::SingletonWrite>::Error> {
                let reference_mut = $try_write?;
                Ok($crate::guard::ReferenceWriteGuard::new(reference_mut))
            }
        }

        impl std::ops::Deref for $wrapper {
            type Target = $inner;
            fn deref(&self) -> &Self::Target {
                <$wrapper as $crate::SingletonRead>::read().value
            }
        }

        impl std::ops::DerefMut for $wrapper {
            fn deref_mut(&mut self) -> &mut Self::Target {
                <$wrapper as $crate::SingletonWrite>::write().value
            }
        }
    };
}


#[macro_export]
macro_rules! singleton_multi_thread {
    ($(#[$attr:meta])* $vis:vis $wrapper:ident, $inner:ty, $static_name:ident, $init:expr) => {
        static $static_name: $crate::SingletonMultiThread<$inner> =
            $crate::SingletonMultiThread::new($init);

$(#[$attr])*
        $vis struct $wrapper;

        impl $crate::SingletonRead for $wrapper {
            type Error =
                <$crate::SingletonMultiThread<$inner> as $crate::guard::TryReadGuard>::Error<'static>;

            type Target = $inner;

            type ReadGuard =
                $crate::features::MappedRwLockReadGuard<'static, $inner>;

            fn try_read() -> Result<Self::ReadGuard, Self::Error> {
                let g =
                    <$crate::SingletonMultiThread<$inner> as $crate::guard::TryReadGuard>
                        ::try_read(&$static_name)?;

                Ok($crate::features::rw_read_guard_map(g, |l| ::std::sync::LazyLock::force(l)))
            }
        }

        impl $crate::SingletonWrite for $wrapper {
            type Error =
                <$crate::SingletonMultiThread<$inner> as $crate::guard::TryWriteGuard>::Error<'static>;

            type WriteGuard =
                $crate::features::MappedRwLockWriteGuard<'static, $inner>;

            fn try_write()
                -> Result<Self::WriteGuard, <Self as $crate::SingletonWrite>::Error>
            {
                let g =
                    <$crate::SingletonMultiThread<$inner> as $crate::guard::TryWriteGuard>
                        ::try_write(&$static_name)?;

                Ok($crate::features::rw_write_guard_map(
                    g,
                    |l| $crate::features::lazy_lock_force_mut(l),
                ))
            }
        }
    };
}

/*
TODO: how to not expose the mapped guard
#[macro_export]
macro_rules! singleton_multi_thread_access {
    ($(#[$attr:meta])* $vis:vis $wrapper:ident, $inner:ty, $try_read:block, $try_write:block) => {

        $(#[$attr])*
        $vis struct $wrapper;

        impl $crate::SingletonRead for $wrapper {
            type Error =
                <$crate::SingletonMultiThread<$inner> as $crate::guard::TryReadGuard>::Error<'static>;

            type Target = $inner;

            type ReadGuard =
                $crate::features::MappedRwLockReadGuard<'static, $inner>;

            fn try_read() -> Result<Self::ReadGuard, Self::Error> {
                $try_read
            }
        }

        impl $crate::SingletonWrite for $wrapper {
            type Error =
                <$crate::SingletonMultiThread<$inner> as $crate::guard::TryWriteGuard>::Error<'static>;

            type WriteGuard =
                $crate::features::MappedRwLockWriteGuard<'static, $inner>;

            fn try_write()
                -> Result<Self::WriteGuard, <Self as $crate::SingletonWrite>::Error>
            {
                $try_write
            }
        }
    };
}
*/


// Used insde the macro, since some feature are unstable
#[doc(hidden)]
pub mod features
{
    pub type MappedRwLockReadGuard<'a,T> = ::std::sync::MappedRwLockReadGuard<'a, T>;
    pub type MappedRwLockWriteGuard<'a,T> = ::std::sync::MappedRwLockWriteGuard<'a, T>;

    #[must_use]
    #[inline(always)]
    pub fn lazy_lock_force_mut<T,F>(this: &mut std::sync::LazyLock<T, F>) -> &mut T
        where F: FnOnce()->T
    {
        std::sync::LazyLock::force_mut(this)
    }

    #[must_use]
    #[inline(always)]
    pub fn rw_read_guard_map<'rwlock,T, U, F>(orig: std::sync::RwLockReadGuard<'rwlock,T>, f: F) -> std::sync::MappedRwLockReadGuard<'rwlock, U>
    where
        F: FnOnce(&T) -> &U,
        U: ?Sized
    {
        std::sync::RwLockReadGuard::map(orig, f)
    }

    #[must_use]
    #[inline(always)]
    pub fn rw_write_guard_map<'rwlock,T, U, F>(orig: std::sync::RwLockWriteGuard<'rwlock,T>, f: F) -> std::sync::MappedRwLockWriteGuard<'rwlock, U>
    where
        F: FnOnce(&mut T) -> &mut U,
        U: ?Sized
    {
        std::sync::RwLockWriteGuard::map(orig, f)
    }
}