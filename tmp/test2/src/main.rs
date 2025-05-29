#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use hexga::prelude::*;
use hexga_map_on::*;
use hexga_graphics::*;

use serde::{Serialize, Deserialize};

//#[derive(Serialize, Deserialize, Save, Load)]
#[io]
struct Person
{
    age : i32,
    name : String,
}

fn main()
{
    for markup in Io::MARKUP_EXTENSIONS
    {
        let r = [1,2,3,4].save_to_disk(&format!("./array.{markup}"));
        println!("{:?}", r);
    }

    Mat2P::IDENTITY.save_to_disk("./tmp/test2/asset/matrix.ron").unwrap();
    Vec3::ONE.save_to_disk("./tmp/test2/asset/vec3.ron").unwrap();

    let p = Person{ age: 42, name: "Idk what name to choose".to_owned() };

    assert_eq!(u8::cast_range_from(1f32), 255u8);
    assert_eq!(u8::cast_range_from(0f32), 0u8);
    assert_eq!(u8::cast_range_from(0.5f32), 127u8);

    // Also support casting the same size, but different precision
    assert_eq!(u8::cast_range_from(0i8), 0u8);
    assert_eq!(u8::cast_range_from(127i8), 254u8);

    assert_eq!(i8::cast_range_from(0u8), 0i8);
    assert_eq!(i8::cast_range_from(255u8), 127i8);

    assert_eq!(u8::cast_range_from(128u8), 128u8);



    assert_eq!(<i8 as CastRangeInto<u8>>::cast_range_into(2i8), 4u8);

    // Also work with composite like [std::array], [Vector], [ColorRGBA]...
    assert_eq!(<[u8;3] as CastRangeIntoComposite<u16>>::cast_range_into_composite([0u8, 127u8, 255u8]), [0u16, 32639u16, 65535u16]);
    assert_eq!(<[u8;3] as CastRangeIntoComposite<u16>>::cast_range_into_composite([0u8, 127u8, 255u8]), [0u16, u16::MAX / 2 - u8::RANGE_MAX as u16 / 2 - 1, u16::MAX]);
    assert_eq!(<ColorRGBAOf::<u8> as CastRangeIntoComposite<u16>>::cast_range_into_composite(ColorRGBAOf::<u8>::RED),
                ColorRGBAOf::<u16>::RED
            );

    let float32 = 2.5f32;
    let float64 = 2.5f64;
    let float32_to_64 = f64::cast_from(float32);
    assert_eq!(float32_to_64, float32_to_64);

    // The most generic way to do it
    let float32_to_64 = <f32 as CastIntoComposite<f64>>::cast_into_composite(float32);
    assert_eq!(float32_to_64, float32_to_64);

    // Example applied  to a vector
    let vec_f32 = Vector2::<f32>::new(0.5, 0.5);
    let vec_f64 = Vector2::<f64>::new(0.5, 0.5);
    let vec_f32_to_f64 = <Vector2::<f32> as CastIntoComposite<f64>>::cast_into_composite(vec_f32);
    assert_eq!(vec_f32_to_f64, vec_f64);



    assert_eq!(<u8 as CastRangeInto<u16>>::cast_range_into(u8::MAX), u16::MAX);


    assert_eq!(<u8 as CastInto<i32>>::cast_into(255u8), 255i32);
    // Also work with composite
    assert_eq!(<[u8;2] as CastIntoComposite<i32>>::cast_into_composite([42u8, 99u8]), [42i32,99i32]);

    vec_f32.save_to_disk("./tmp/test2/asset/person.ron").unwrap();

    //u8::MAX.cast_range_into()
    //let x = u16::cast_range_from(u8::MAX)
    //dbg!(u)

    /*
    dbg!(Color::WHITE);
    dbg!(Color::RED);

    dbg!(ColorByte::WHITE);
    dbg!(ColorByte::RED);

    dbg!(ColorByte::WHITE);
    dbg!(ColorMask::RED);

    dbg!(ColorMask::RED | ColorMask::BLUE);


    let img = ImageBase::from_fn(point2(u8::MAX as _, u8::MAX as _),
    |p| ColorRGBAOf::rgb(p.x as u8, p.y as u8, zero()));
    img.tmp_write_to_png_bytes_inside("./tmp/picture32.png");


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
    */
}