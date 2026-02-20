#![allow(unused)]
#![allow(dead_code)]

use hexga::prelude::*;
use hexga::arena::prelude::*;

fn main()
{
    let mut arena = Arena::new();
    dbg!(&arena);
    {
        let a = arena.alloc_or_panic(10);
        let b = arena.alloc_or_panic(42);
        dbg!(&a);
        dbg!(&b);
        drop(arena);
        dbg!(&a);
    }
    println!("hello");
}