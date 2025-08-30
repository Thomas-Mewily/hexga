#![allow(unused_imports)]
use hexga::{prelude::*,*};

fn main() 
{
    Rng::init(hexga::random::RandomSourceDummy::___());
    for _ in 0..100
    {
        //println!("{}", Rng.random::<bool>());
        println!("{}", Rng.random::<i32>());
    }
}
