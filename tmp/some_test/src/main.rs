use hexga::prelude::*;

fn main()
{
    /*
    let mut f = FileData::load_or_create("hello", || "hi".to_owned());
    dbg!(f.get_path());
    f.push_str(" goodbye!");

    for i in (0.0..=1.0f32).sample(10f32)
    {
        println!("{}", i);
    }

    for i in (Angle::ZERO..Angle::FULL).sample(10)
    {
        println!("{}", i);
    }
    */

    /*
    for i in (10.degree()..=20.degree()).step(1.degree())
    {
        println!("{}", i);
    }*/

    for i in (0.0..=1.0f32).sample(10).rev()
    {
        println!("{}", i);
    }
}
