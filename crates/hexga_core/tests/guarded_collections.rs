#![cfg(feature = "std")]

use hexga_core::prelude::*;
use std::{
    cell::RefCell,
    collections::{BinaryHeap, VecDeque},
    rc::Rc,
    sync::{Arc, Mutex, RwLock},
};

#[test]
fn acquired_read_guards_forward_lookups_and_peeks()
{
    let mutex = Mutex::new(RefCell::new(vec![10, 20]));
    {
        let lock = mutex.lock().unwrap();
        let entries = lock.borrow();
        assert!(Get::get(&entries, 20usize).is_none());
        assert!(TryGet::try_get(&entries, 20usize).is_err());
        assert_eq!(*Get::get(&entries, 1usize).unwrap(), 20);
        assert!(mutex.try_lock().is_err());
    }
    assert!(mutex.try_lock().is_ok());

    let rwlock = RwLock::new(vec![30, 40]);
    {
        let entries = rwlock.read().unwrap();
        let first = First::<usize>::first(&entries).unwrap();
        let last = Last::<usize>::last(&entries).unwrap();
        assert_eq!((*first, *last), (30, 40));
        assert!(rwlock.try_write().is_err());
    }
    assert!(rwlock.try_write().is_ok());
}

#[test]
fn acquired_write_guards_preserve_inner_mutable_guards()
{
    let mut lock = Mutex::new(VecDeque::from([1, 2, 3]));
    {
        let mut entries = lock.lock().unwrap();
        *GetMut::get_mut(&mut entries, 1usize).unwrap() = 20;
        let [first, third] = GetManyMut::try_get_many_mut(&mut entries, [0usize, 2]).unwrap();
        *first = 10;
        *third = 30;
        assert_eq!(GetManyMut::try_get_many_mut(&mut entries, [0usize, 0]), Err(ManyMutError::OverlappingIndices));
    }

    PushFront::push_front(&mut lock, 0);
    Push::push(&mut lock, 40);
    assert_eq!(PopFront::pop_front(&mut lock), Some(0));
    assert_eq!(Pop::pop(&mut lock), Some(40));
    assert_eq!(Length::len(&lock), 3);
    assert_eq!(lock.into_inner().unwrap(), [10, 20, 30]);

    let mut heap = RwLock::new(BinaryHeap::from([1, 2, 3]));
    *LastMut::<()>::last_mut(&mut heap.write().unwrap()).unwrap() = 0;
    assert_eq!(Pop::pop(&mut heap), Some(2));
}

#[test]
fn singleton_and_shared_pointer_forwarding_composes()
{
    let singleton = SingletonMutex::new(vec![1, 2]);
    {
        let mut guard = singleton.get_mut();
        Push::push(&mut guard, 3);
        *GetMut::get_mut(&mut guard, 0usize).unwrap() = 10;
        assert_eq!(*Get::get(&guard, 0usize).unwrap(), 10);
        assert_eq!(*Last::<usize>::last(&guard).unwrap(), 3);
    }

    let mut rc = Rc::new(RefCell::new(vec![10, 20]));
    let shared = Rc::clone(&rc);
    Push::push(&mut rc, 30);
    assert_eq!(*Get::get(&shared.borrow(), 2usize).unwrap(), 30);

    let mut arc = Arc::new(SingletonRw::new(vec![10, 20, 30]));
    let shared = Arc::clone(&arc);
    assert_eq!(Pop::pop(&mut arc), Some(30));
    assert_eq!(Length::len(&shared), 2);
    let entries = SingletonRw::get(&shared);
    assert_eq!(*Get::get(&entries, 1usize).unwrap(), 20);
}

#[test]
fn cloned_arc_can_edit_and_borrow_the_same_collection()
{
    let mut collection = Arc::new(Mutex::new(vec![1, 2]));
    let observer = Arc::clone(&collection);
    Push::push(&mut collection, 3);

    {
        let mut entries = collection.lock().unwrap();
        let [first, last] = GetManyMut::get_many_mut(&mut entries, [0usize, 2]).unwrap();
        *first = 10;
        *last = 30;
        assert!(observer.try_lock().is_err());
    }

    assert_eq!(*observer.try_lock().unwrap(), [10, 2, 30]);
}

#[test]
fn guarded_forwarding_accepts_an_unsized_collection()
{
    let shared: Arc<Mutex<[i32]>> = Arc::new(Mutex::new([1, 2]));
    assert_eq!(Length::len(&shared), 2);
    {
        let mut entries = GuardedMut::get_mut(&*shared);
        *GetMut::get_mut(&mut entries, 1usize).unwrap() = 3;
    }
    let entries = Guarded::get(&*shared);
    assert_eq!(*Get::get(&entries, 1usize).unwrap(), 3);
}
