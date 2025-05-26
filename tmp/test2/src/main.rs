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

    /* 
    let img = ImageBase::from_fn(point2(u8::MAX as _, u8::MAX as _), 
    |p| ColorRGBAOf::rgb(p.x as u8, p.y as u8, zero()));
    img.tmp_write_to_png_bytes_inside("./tmp/picture32.png");
    */

    for nb_sample in 2..4
    {
        let mut c = Vec::new();

        for r in u8::sample_inclusive(nb_sample)
        {
            for g in u8::sample_inclusive(nb_sample)
            {
                for b in u8::sample_inclusive(nb_sample)
                {
                    c.push(rgb_byte(r, g, b));  
                }
            }
        }

        let img = ImageBase::from_vec(point2(c.len() as _, 1), c).unwrap();
        img.tmp_write_to_png_bytes_inside(&format!("./tmp/constant_color_{nb_sample}.png"));
    }

    let mut c = Vec::new();

    for x in u8::sample_inclusive(2)
    {
        for y in u8::sample_inclusive(2)
        {
            c.push(rgb_byte(x, y, u8::RANGE_HALF));
            c.push(rgb_byte(x, u8::RANGE_HALF, y));
            c.push(rgb_byte(u8::RANGE_HALF, x, y));
        }
    }

    for co in c.iter().skip(3)
    {
        
        println!(
            "    /// {}
    const COLOR_NAME : Self;", co.to_rgba_byte_hex_string());
    }

    let img = ImageBase::from_vec(point2(c.len() as _, 1), c).unwrap();
    img.tmp_write_to_png_bytes_inside(&format!("./tmp/constant_color_half.png"));

    
    //|p| ColorRGBAOf::rgb(p.x as u8, p.y as u8, zero()));
}