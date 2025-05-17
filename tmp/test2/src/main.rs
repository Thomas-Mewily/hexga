#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use hexga::prelude::*;
use hexga_map_on::*;

fn main() 
{
    let mut it = (0.0..1.0).step(0.3);

    dbg!(it.next());
    dbg!(it.next());
    dbg!(it.next_back());
    dbg!(it.next_back());
    dbg!(it.next_back());
    dbg!(it.next_back());
}
