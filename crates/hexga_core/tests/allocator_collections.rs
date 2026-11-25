#![cfg(feature = "std")]

use hexga_core::prelude::*;
use std::alloc::System;

#[test]
fn vector_traits_accept_custom_allocators() {
    let allocator = System;
    let mut values = <Vec<_, _> as WithCapacity>::with_capacity_and_param(4, &allocator);
    assert_eq!(Push::push(&mut values, 42), 0);
    assert_eq!(TryGet::try_get(&values, 0usize), Ok(&42));
    assert_eq!(Pop::pop(&mut values), Some(42));

    let collected = Vec::<_, System>::try_from_iter([1, 2, 3]).unwrap();
    assert_eq!(collected.as_slice(), [1, 2, 3]);
}

#[test]
fn box_traits_accept_custom_allocators() {
    let boxed = <Box<_, System> as WrapperNew>::new_wrapper(42);
    assert_eq!(SharedCount::strong_count(&boxed), 1);
    assert_eq!(WrapperIntoInner::into_inner(boxed), 42);
}
