#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use hexga::prelude::*;
use hexga_map_on::*;

fn main() 
{
    dbg!(Color::WHITE);
    dbg!(Color::RED);

    dbg!(ColorByte::WHITE);
    dbg!(ColorByte::RED);

    dbg!(ColorByte::WHITE);
    dbg!(ColorMask::RED);

    dbg!(ColorMask::RED | ColorMask::BLUE);
}