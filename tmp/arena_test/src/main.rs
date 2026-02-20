#![allow(unused)]
#![allow(dead_code)]

use hexga::prelude::*;
use hexga::arena::prelude::*;

fn main()
{
    let mut arena = Arena::new();
    dbg!(&arena);
    {
        let b = arena.allocate_or_panic(42);
        dbg!(b);
    }
    println!("hello");
}
