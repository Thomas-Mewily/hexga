use super::*;
use crate::{
    cell::SingleThreadCell,
    identity::Identity,
    rc::Rc,
    singleton::{
        SingletonCell, SingletonLazyCell, SingletonLazyMutex, SingletonLazyRw, SingletonMutex, SingletonOnce, SingletonOnceCell, SingletonOnceLazy,
        SingletonOnceLazyCell, SingletonOptionCell, SingletonOptionMutex, SingletonOptionRw, SingletonRw,
    },
    sync::Arc,
    utils::DoubleDeref,
};
use std::{
    cell::{LazyCell, OnceCell, Ref, RefCell, RefMut},
    sync::{LazyLock, Mutex, MutexGuard, OnceLock, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

macro_rules! forward_mut_guard {
    ($guard:ident) => {
        forward_read!(['g, C: ?Sized] $guard<'g, C> => C, Deref::deref);
        forward_write!(['g, C: ?Sized] $guard<'g, C> => C, (guard => DerefMut::deref_mut(guard)));
    };
}

forward_mut_guard!(MutexGuard);
forward_mut_guard!(RwLockWriteGuard);
forward_mut_guard!(RefMut);
forward_read!(['g, C: ?Sized] RwLockReadGuard<'g, C> => C, Deref::deref);
forward_read!(['g, C: ?Sized] Ref<'g, C> => C, Deref::deref);

macro_rules! forward_double_guard {
    (read $guard:ident, $inner:ident) => {
        forward_read!(['g, C] DoubleDeref<$guard<'g, $inner<C>>> => C, Deref::deref);
    };
    (write $guard:ident, $inner:ident) => {
        forward_double_guard!(read $guard, $inner);

        forward_write!(['g, C] DoubleDeref<$guard<'g, $inner<C>>> => C, (guard => DerefMut::deref_mut(guard)));
    };
}

forward_double_guard!(write MutexGuard, Identity);
forward_double_guard!(write RwLockWriteGuard, Identity);
forward_double_guard!(read RwLockReadGuard, Identity);
forward_double_guard!(write MutexGuard, LazyLock);
forward_double_guard!(read RwLockReadGuard, LazyLock);
forward_double_guard!(write RwLockWriteGuard, LazyLock);

#[cfg(feature = "feature_mapped_lock_guards")]
mod mapped
{
    use super::*;
    use std::sync::{MappedMutexGuard, MappedRwLockReadGuard, MappedRwLockWriteGuard};

    forward_mut_guard!(MappedMutexGuard);
    forward_mut_guard!(MappedRwLockWriteGuard);
    forward_read!(['g, C: ?Sized] MappedRwLockReadGuard<'g, C> => C, Deref::deref);
}

macro_rules! forward_guarded_collection {
    ($container:ident, $read:path) => {
        forward_inspect!([C] $container<C> => C, $read);
    };
}

forward_inspect!([C: ?Sized] Mutex<C> => C, Guarded::get);
forward_inspect!([C: ?Sized] RwLock<C> => C, Guarded::get);
forward_inspect!([C: ?Sized] RefCell<C> => C, Guarded::get);
forward_guarded_collection!(SingleThreadCell, Guarded::get);

forward_guarded_collection!(SingletonMutex, GuardedMut::get_mut);
forward_guarded_collection!(SingletonLazyMutex, GuardedMut::get_mut);
forward_guarded_collection!(SingletonOptionMutex, GuardedMut::get_mut);
forward_guarded_collection!(SingletonRw, Guarded::get);
forward_guarded_collection!(SingletonLazyRw, Guarded::get);
forward_guarded_collection!(SingletonOptionRw, Guarded::get);
forward_guarded_collection!(SingletonCell, Guarded::get);
forward_guarded_collection!(SingletonLazyCell, Guarded::get);
forward_guarded_collection!(SingletonOptionCell, Guarded::get);
forward_guarded_collection!(SingletonOnceCell, Guarded::get);
forward_guarded_collection!(SingletonOnceLazyCell, Guarded::get);
forward_read!([C] SingletonOnce<C> => C, Guarded::get);
forward_read!([C] SingletonOnceLazy<C> => C, Guarded::get);

fn read_once_cell<C>(cell: &OnceCell<C>) -> &C { OnceCell::get(cell).expect("collection cell is not initialized") }

fn read_once_lock<C>(lock: &OnceLock<C>) -> &C { OnceLock::get(lock).expect("collection lock is not initialized") }

forward_read!([C] OnceCell<C> => C, read_once_cell);
forward_read!([C] OnceLock<C> => C, read_once_lock);
forward_read!([C, F: FnOnce() -> C] LazyCell<C, F> => C, LazyCell::force);
forward_read!([C, F: FnOnce() -> C] LazyLock<C, F> => C, LazyLock::force);

forward_write!([C, F: FnOnce() -> C] LazyCell<C, F> => C, (cell => LazyCell::force_mut(cell)));
forward_write!([C, F: FnOnce() -> C] LazyLock<C, F> => C, (lock => LazyLock::force_mut(lock)));

macro_rules! forward_exclusive_access {
    ([$($parameters:tt)*] $container:ident, $source:ident => $inner:expr) => {
        forward_edit!([$($parameters)*] $container<C> => C, ($source => $inner));
    };
}

forward_exclusive_access!([C: ?Sized] Mutex, lock => Mutex::get_mut(lock).expect("collection mutex is poisoned"));
forward_exclusive_access!([C: ?Sized] RwLock, lock => RwLock::get_mut(lock).expect("collection rwlock is poisoned"));
forward_exclusive_access!([C: ?Sized] RefCell, cell => RefCell::get_mut(cell));
forward_exclusive_access!([C] SingleThreadCell, cell => cell.get_exclusive());
forward_write!([C] OnceCell<C> => C, (cell => OnceCell::get_mut(cell).expect("collection cell is not initialized")));
forward_write!([C] OnceLock<C> => C, (lock => OnceLock::get_mut(lock).expect("collection lock is not initialized")));

forward_exclusive_access!([C] SingletonMutex, singleton => &mut Mutex::get_mut(&mut singleton.guarded).expect("collection mutex is poisoned").0);
forward_exclusive_access!([C] SingletonLazyMutex, singleton => LazyLock::force_mut(Mutex::get_mut(&mut singleton.guarded).expect("collection mutex is poisoned")));
forward_exclusive_access!([C] SingletonOptionMutex, singleton => Mutex::get_mut(&mut singleton.guarded).expect("collection mutex is poisoned").as_mut().expect("singleton is not initialized"));
forward_exclusive_access!([C] SingletonRw, singleton => &mut RwLock::get_mut(&mut singleton.guarded).expect("collection rwlock is poisoned").0);
forward_exclusive_access!([C] SingletonLazyRw, singleton => LazyLock::force_mut(RwLock::get_mut(&mut singleton.guarded).expect("collection rwlock is poisoned")));
forward_exclusive_access!([C] SingletonOptionRw, singleton => RwLock::get_mut(&mut singleton.guarded).expect("collection rwlock is poisoned").as_mut().expect("singleton is not initialized"));
forward_exclusive_access!([C] SingletonCell, singleton => &mut singleton.guarded.get_exclusive().0);
forward_exclusive_access!([C] SingletonLazyCell, singleton => LazyCell::force_mut(singleton.guarded.get_exclusive()));
forward_exclusive_access!([C] SingletonOptionCell, singleton => singleton.guarded.get_exclusive().as_mut().expect("singleton is not initialized"));

fn read_shared<C: ?Sized, G: ?Sized + Guarded<C>>(shared: &impl Deref<Target = G>) -> G::Guard<'_> { Guarded::get(&**shared) }

fn write_shared<C: ?Sized, G: ?Sized + GuardedMut<C>>(shared: &impl Deref<Target = G>) -> G::GuardMut<'_> { GuardedMut::get_mut(&**shared) }

macro_rules! forward_shared_guard {
    ([$($parameters:tt)*] $pointer:ident, $container:ident, $read:path) => {
        forward_inspect!([$($parameters)*] $pointer<$container<C>> => C, $read);
        forward_edit!([$($parameters)*] $pointer<$container<C>> => C, (shared => write_shared(shared)));
    };
}

macro_rules! forward_shared_collections {
    ($pointer:ident) => {
        forward_shared_guard!([C: ?Sized] $pointer, Mutex, read_shared);
        forward_shared_guard!([C: ?Sized] $pointer, RwLock, read_shared);
        forward_shared_guard!([C: ?Sized] $pointer, RefCell, read_shared);

        forward_shared_guard!([C] $pointer, SingleThreadCell, read_shared);
        forward_shared_guard!([C] $pointer, SingletonMutex, write_shared);
        forward_shared_guard!([C] $pointer, SingletonLazyMutex, write_shared);
        forward_shared_guard!([C] $pointer, SingletonOptionMutex, write_shared);
        forward_shared_guard!([C] $pointer, SingletonRw, read_shared);
        forward_shared_guard!([C] $pointer, SingletonLazyRw, read_shared);
        forward_shared_guard!([C] $pointer, SingletonOptionRw, read_shared);
        forward_shared_guard!([C] $pointer, SingletonCell, read_shared);
        forward_shared_guard!([C] $pointer, SingletonLazyCell, read_shared);
        forward_shared_guard!([C] $pointer, SingletonOptionCell, read_shared);

        forward_inspect!([C] $pointer<SingletonOnceCell<C>> => C, read_shared);
        forward_inspect!([C] $pointer<SingletonOnceLazyCell<C>> => C, read_shared);
        forward_read!([C] $pointer<SingletonOnce<C>> => C, read_shared);
        forward_read!([C] $pointer<SingletonOnceLazy<C>> => C, read_shared);
    };
}

forward_shared_collections!(Rc);
forward_shared_collections!(Arc);
