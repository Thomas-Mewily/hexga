//! Run with `cargo run -p hexga_core --example collection_guards`.

use hexga_core::{identity::Identity, prelude::*};
use std::{
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex},
};

fn swap_and_read<C>(collection: &mut C) -> Result<[u8; 2], ManyMutError>
where
    C: GetManyMut<usize, Output = u8>,
{
    {
        let [mut first, mut second] = collection.try_get_many_mut([0, 1])?;
        std::mem::swap(&mut *first, &mut *second);
    } // Both mutable entry guards drop before we read the values back.

    Ok([*collection.get_or_panic(0), *collection.get_or_panic(1)])
}

fn main() -> Result<(), ManyMutError>
{
    let mut ordinary = vec![10, 20];
    println!("Vec: {:?}", swap_and_read(&mut ordinary)?);
    assert_eq!(ordinary, [20, 10]);

    let mut shared = Arc::new(Mutex::new(vec![10, 20]));
    let observer = Arc::clone(&shared);
    Push::push(&mut shared, 30); // A single operation acquires its own lock.
    {
        let mut entries = shared.lock().unwrap();
        // The full swap and readback run under this one lock.
        println!("Arc<Mutex<Vec<_>>>: {:?}", swap_and_read(&mut entries)?);
    }
    assert_eq!(*observer.lock().unwrap(), [20, 10, 30]);

    let mut encoded = EncodedBytes([!10, !20]);
    println!("Write-back guards: {:?}", swap_and_read(&mut encoded)?);
    assert_eq!(encoded.0, [!20, !10]);

    Ok(())
}

// Each stored byte is inverted. The guard exposes its decoded value and
// writes the encoded byte back when dropped.
struct EncodedBytes([u8; 2]);

struct DecodedByte<'a>
{
    stored: &'a mut u8,
    value: u8,
}

impl<'a> DecodedByte<'a>
{
    fn new(stored: &'a mut u8) -> Self
    {
        let value = !*stored;
        Self { stored, value }
    }
}

impl Deref for DecodedByte<'_>
{
    type Target = u8;

    fn deref(&self) -> &u8 { &self.value }
}

impl DerefMut for DecodedByte<'_>
{
    fn deref_mut(&mut self) -> &mut u8 { &mut self.value }
}

impl Drop for DecodedByte<'_>
{
    fn drop(&mut self) { *self.stored = !self.value; }
}

impl CollectionIndex<usize> for EncodedBytes
{
    type Output = u8;
    type Ref<'a> = Identity<u8>;
}

impl CollectionIndexMut<usize> for EncodedBytes
{
    type RefMut<'a> = DecodedByte<'a>;
}

impl Get<usize> for EncodedBytes
{
    fn get<'a>(&'a self, index: usize) -> Option<Self::Ref<'a>>
    where
        u8: 'a,
    {
        self.0.get(index).map(|stored| Identity(!*stored))
    }
}

impl GetMut<usize> for EncodedBytes
{
    fn get_mut<'a>(&'a mut self, index: usize) -> Option<Self::RefMut<'a>>
    where
        u8: 'a,
    {
        self.0.get_mut(index).map(DecodedByte::new)
    }
}

impl GetManyMut<usize> for EncodedBytes
{
    fn try_get_many_mut<'a, const N: usize>(&'a mut self, indices: [usize; N]) -> Result<[Self::RefMut<'a>; N], ManyMutError>
    where
        u8: 'a,
    {
        self.0.get_disjoint_mut(indices).map(|entries| entries.map(DecodedByte::new))
    }
}
