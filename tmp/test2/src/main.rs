#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use hexga::prelude::*;
use hexga_io::{fs::LoadToDisk, MarkupExtensionFromRon};
use hexga_map_on::*;
use hexga_graphics::prelude::*;

use serde::{Serialize, Deserialize};

//#[derive(Serialize, Deserialize, Save, Load)]
#[io]
struct Person
{
    age : i32,
    name : String,
}

struct VArray<const N : usize>(pub [i32;N]);

impl<const N : usize> ::serde::Serialize for VArray<N> //where [T;N] : ::serde::Serialize
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}


fn main()
{
    /*
    for markup in Io::MARKUP_EXTENSIONS
    {
        let r = [1,2,3,4].save_to_disk(&format!("./array.{markup}"));
        println!("{:?}", r);
    }
    */

    //Mat2P::IDENTITY.save_to_disk("./tmp/test2/asset/matrix.ron").unwrap();
    //Vec3::ONE.save_to_disk("./tmp/test2/asset/vec3.ron").unwrap();

    //Grid2::from_fn((3,4), |p| p.sum_axis()) .save_to_disk("./tmp/test2/asset/grid.ron").unwrap();


    //90.degree().save_to_disk("./tmp/test2/asset/mon_angle.ron").unwrap();
    /*
    let i = Image::from_fn((16, 16), |(x,y)|
    {
        if (x + y) / 8 % 2 == 0
        {
            ColorByte::BLACK
        }else
        {
            ColorByte::WHITE
        }
    }).save_to_disk("./tmp/test2/asset/image_byte.jpg").unwrap();
    */

    dbg!(ColorU8::RED);
    dbg!(ColorRgbaF32::RED);

    let c = ColorU8::RED;
    let c2 = c.to_float();

    let a = 90.degree();
    let ma = [1,2,3].degree();

    //let x : f32 = 32u8.cast_into();

    let i = Image::from_fn((16, 16), |(x,y)|
    {
        ColorU8::rgb(x as u8 * 16, y as u8 * 16, 0)
    }).save_to_disk("./tmp/test2/asset/image_byte.png").unwrap();

    let x = 64;
    let i = Image::from_fn((x, x), |(x,y)|
    {
        if (x + y * x) / 2 % 3 == 0
        {
            ColorU8::BLACK
        }else
        {
            ColorU8::WHITE
        }
    }).save_to_disk("./tmp/test2/asset/test_img").unwrap();


    let mut gros_cercle= Image::from_fn_ndc(32.splat2(), |v|
        if v.length() <= 0.6 { ColorU8::WHITE } else { ColorU8::BLACK }
    );

    let petit_cercle= Image::from_fn_ndc(16.splat2(), |v|
        if v.length() <= 0.6 { ColorU8::RED } else { ColorU8::BLACK }
    );

    //let c = petit_cercle.to_color_rgba_f64();

    dbg!(&petit_cercle.update_into(&mut gros_cercle.view_mut().crop_intersect(petit_cercle.rect())));


    //petit_cercle.to_float();
    //gros_cercle.view_mut().swap


    gros_cercle.save_to_disk("./tmp/test2/asset/mon_cercle").unwrap();

    /*D
    dbg!(Angle::from_ron("45"));
    dbg!(Angle::from_ron("(rad: 2.0)"));
    dbg!(Angle::from_ron("(deg: 2.0)"));
    dbg!(Angle::from_ron("(turn: 0.5)"));


    dbg!(Angle::from_ron("(turn: 0.5, deg: 2.0, rad:0.1)"));
    dbg!(Angle::from_ron("()"));

    dbg!(Time::from_ron("42.5"));
    dbg!(Time::from_ron("(min: 2)"));
    dbg!(Time::from_ron("(d: 1.)"));
    dbg!(Time::from_ron("(d: 1., ms:1000)"));
    dbg!(Time::from_ron("(d: 1., ms:1000, min:42)"));
    dbg!(Time::from_ron("()"));
    dbg!(Time::from_ron(""));
    */
}