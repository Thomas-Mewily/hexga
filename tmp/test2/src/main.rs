#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use hexga::prelude::*;
use hexga_io::{fs::LoadToDisk, MarkupExtensionFromRon};
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

    Mat2P::IDENTITY.save_to_disk("./tmp/test2/asset/matrix.ron").unwrap();
    Vec3::ONE.save_to_disk("./tmp/test2/asset/vec3.ron").unwrap();

    //Grid2::from_fn((3,4), |p| p.sum_axis()) .save_to_disk("./tmp/test2/asset/grid.ron").unwrap();


    let i = Image::from_fn((16, 16), |(x,y)|
    {
        if (x + y) / 8 % 2 == 0
        {
            ColorByte::BLACK
        }else
        {
            ColorByte::WHITE
        }
    }).save_to_disk("./tmp/test2/asset/image_byte.ron").unwrap();




    /*
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