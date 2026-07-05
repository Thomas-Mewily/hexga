use hexga::prelude::*;

fn main()
{
    /*
    let mut f = FileData::load_or_create("_tmp/hello", || "hi".to_owned());
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
    }
    */

    for i in (0.0..=1.0f32).sample(9.9).rev()
    {
        println!("{}", i);
    }

    let img = Image::from_fn((16,16), |(x,y)| 
    {
        ColorU8::rgb((255 / 16 * x) as _, (255 / 16 * y) as _, 0)
    });

    Io::save(&img, "_tmp/image").unwrap();
    //Io::load_or_create(path, init)
img.save_to_fs(&mut Io, "_tmp/image2").unwrap();
    //img.save_to_fs(extension)

    
}
