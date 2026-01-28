#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]


use hexga::{generational::gen_id::GenIDOf, prelude::*};
use hexga_math::bijection::*;

fn dbg_mix<S,F>(src: S, dest:S, coef:F) where S:Mix<F> + Copy + Debug, F:Float
{
    dbg!(&src);
    dbg!(&dest);
    dbg!(src.mix(dest, coef));
}

fn x()
{
    let v = vec2(0., 1.);


    let size = point2(20, 40);
    let mut g = size.to_grid(|v| -2);
    let mut g2 = size.to_grid(|v| v.sum_axis());
    let m = g.max(g2);
    println!("{}", m);


    dbg!(v);
    dbg!(m);
    dbg!(Mat4::from_fn(|v| v.sum_axis() as float * 5.));

    let x = 127u8.to_float_range();

    for coef in (0.0..=1.).sample(10)
    {
        dbg!(coef);
        dbg!(false.mix(true, coef));
    }
    println!();
    println!();


    let x = 10.to_f32();
    let x = [1i32,2,3].to_f32();

    let a = max([1,5], [2,3]);
    dbg!(a);
    println!("hello world");
    dbg_mix(0., 1., 1.);
    dbg_mix(0., 1., 0.);
    dbg_mix(0., 1., 0.5);
}


fn img()
{
    //let i = Image::from_fn_coef((4, 4), |v| hsl(v.x, v.y, 1.).to_rgba_of::<float>());
    let i = ImageOf::<ColorU16>::from_fn_coef((1024, 10), |v| rgb(v.x, v.y, 0.0).to_u16_range());
    let original_pixels = i.pixels().to_owned();

    i.save("./test").unwrap();
    let i = ImageOf::<ColorU16>::load("./test.png").unwrap();
    let open_pixels = i.pixels().to_owned();

    assert_eq!(original_pixels, open_pixels);

    dbg!(i);
}

fn img_test()
{
    let i = Image::from_fn_coef((1024, 1024),
        |v|
        if (v - 0.5.splat2()).length() <= 0.5
        {
            ColorU8::WHITE
        }
        else
        {
            ColorU8::BLACK
        }
    ).save("./image_test.png").unwrap();
}

use std::{num::Saturating, ops::*};



#[math_vec]
pub struct DamageOf2<T>
{
    physic: T,
    magic: T,
}



#[math_vec]
pub struct DamageArray<T, const N:usize>
{
    boum:[T;N],
}

#[derive(Serialize, Deserialize, Debug)]
struct Foo
{
    bar:i32,
    age:i32,
}

#[math_vec]
#[non_exhaustive]
pub struct DamageOf<T>
{
    physic: T,
    magic: T,
    melee: T,
}
type Damage = DamageOf<i32>;


#[derive(Serialize, Deserialize)]
pub enum DamageVersion
{
    Version(Damage),
    Version2(Damage),
    //
}

struct Linear<T>
{
    origin: T,
    coef: T,
}


fn x2()
{
    // A + Bx

    dbg!(Vector3::new(true, false, true) & Vector3::new(true, true, false));
    let x = Vector3::new(true, false, true);
    let f = &x.x;

    println!("{}", DamageOf::<u8>::ONE.to_ron().unwrap());
    println!("{}", DamageOf::<u8>::from_ron("(melee:18,physic:42)").unwrap());
    println!("{}", <DamageOf::<u8> as WithDefault<u8,ConstantOne<u8>>>::WithDefault::from_ron("(physic:1,melee:1)").unwrap().into_value());

    //println!("{}", DamageVersion::Version2(DamageOf::MAX).to_ron().unwrap());


    //dbg!()
    //println!("{}", <Damage::<u8> as WithDefault<u8,ConstantMax<u8>>>::WithDefault::from_ron("(physic:1,melee:1)").unwrap().into_value());



    //println!("{}", DamageOf::<u8>::from_ron("(1, 1, 1)").unwrap());

    //println!("{}", DamageOf::<u8>::from_ron("(physic:1,magic:1,melee:1)").unwrap());

    /*
    let x = Damage::<i32>::ONE * Damage::<i32>::MINUS_ONE;


    let j = Damage::<Saturating<u8>>::splat(Saturating(140)) + Damage::<Saturating<u8>>::splat(Saturating(170));
    println!("{}", j);

    println!("{}", Damage::<u8>::ONE.to_ron().unwrap());
    println!("{}", Damage::<u8>::from_ron("(physic:1,magic:1,melee:1)").unwrap());
    println!("{}", Damage::<u8>::from_ron("(physic:1,melee:1)").unwrap());
    println!("{}", <Damage::<u8> as WithDefault<u8,ConstantMax<u8>>>::WithDefault::from_ron("(physic:1,melee:1)").unwrap().into_value());
    */
    //println!("{}", DamageWithDefault::<u8, ConstantMax<u8>>::from_ron("(physic:1,melee:1)").unwrap().into_value());
}

#[bit_index]
#[repr(u8)]
pub enum Color
{
    Red,
    Green,
    Blue = 5,
    Yellow, // = 6
    RedAndBlue = Color::Red | Self::Blue,
    Purple, // 7
    GreenAndYellowAndPurple = ((Color::Yellow | Self::Purple)) | Self::Green,
}

fn color()
{
    let color = Color::Red | Color::Green;
    dbg!(&color);

    for c in color
    {
        dbg!(c);
    }

    println!("hello world");
}


pub type Grid2<C, Idx, const N: usize> = Bijection<C,BijectionPointToUsize<Idx,N>>;

fn bi()
{
    let v = vec![1,2,3,4,5,6];
    let s: &[i32] = &*v;


    dbg!(&s.get(0));
    dbg!(&Bijection::from_values_and_bijection(s, BijectionRev::from(s)).get(0usize));
    dbg!(s.bijection_rev());

    let g = Grid2::from_values_and_bijection(s, BijectionPointToUsize::new(point2(2, 3)));
    dbg!(&g.get(point2(0, 1)));
    dbg!(s.as_view().bijection_rev());

    // grid type ?
}


fn main()
{
    let slice: &[char] = &['a','b','c','d','e','f'];

    assert_eq!(slice.bijection_identity().get(0usize), Some(&'a'));
    assert_eq!(slice.bijection_rev().get(0usize), Some(&'f'));

    let it = slice.bijection_identity().into_iter().collect::<Vec<_>>();
    println!("{:?}", it);

    let it = slice.bijection_rev().into_iter().collect::<Vec<_>>();
    println!("{:?}", it);

    let grid_3x2 = slice.with_bijection(BijectionPointToUsize::new(point2(3, 2)));
    assert_eq!(grid_3x2.get(point2(0,1)), Some(&'d'));
    let it = grid_3x2.into_iter().collect::<Vec<_>>();
    println!("{:?}", it);
}