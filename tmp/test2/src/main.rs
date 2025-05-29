#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use hexga::prelude::*;
use hexga_io::MarkupExtensionFromRon;
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

    90.degree().save_to_disk("./tmp/test2/asset/angle.ron").unwrap();

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