#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use hexga::prelude::*;
use hexga_map_on::*;
use hexga_graphics::*;

fn main() 
{
    dbg!(Color::WHITE);
    dbg!(Color::RED);

    dbg!(ColorByte::WHITE);
    dbg!(ColorByte::RED);

    dbg!(ColorByte::WHITE);
    dbg!(ColorMask::RED);

    dbg!(ColorMask::RED | ColorMask::BLUE);

    let img = ImageBase::from_fn(point2(u8::MAX as _, u8::MAX as _), 
    |p| ColorRGBAOf::rgb(p.x as u8, p.y as u8, zero()));
    //|p| ColorRGBAOf::rgb(p.x as u8, p.y as u8, zero()));
    img.tmp_write_to_png_bytes_inside("./picture32.png");
}