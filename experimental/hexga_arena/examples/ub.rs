use hexga_arena::prelude::*;

fn main()
{
    let mut arena = Arena::new();
    dbg!(&arena);

    let a = arena.alloc_or_panic(10);
    let b = arena.alloc_or_panic(20);
    dbg!(&a);
    dbg!(&b);

    drop(arena);
    dbg!(&a); // UB, because the arena is dropped
}
