#[allow(unused)]
use hexga::{default, prelude::*};

fn _x()
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

    /*
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
    */

    //let m = FileData::load_or_create("_tmp/hello", || "hello".to_owned());
    let hello = AssetGlobal::load_or_create("_tmp/hello.txt", || "hello".to_owned());
    println!();
    let hi = AssetGlobal::load_or_create("_tmp/hello", || "hi".to_owned());
    println!();

    let hi2 = AssetGlobal::from_path_and_value(Some("_tmp/hello.txt".into()), "hi 2".to_owned());
    println!();

    println!("{}", hello);
    println!("{}", hi);
    println!("{}", hi2);

    //Asset::load

    dbg!(AssetManagerIn::<String, IoGlobal>::assets());

    println!();
    drop(hello);
    println!();
    drop(hi);
    println!();
    drop(hi2);

    dbg!(AssetManagerIn::<String, IoGlobal>::assets());

    //img.save_to_fs(extension)
}


fn _cast_into<X,Y>(value: X) -> Y where X: CastInto<Y> { value.cast_into() }



fn main()
{

    //let x = [1u8, 2u8] as [f32;2];
    //let x : [f32;2] = [1u8, 2u8].cast_into();
    
    
    /*
    let _img = Image::from_fn((16,16), |(x,y)|
    {
        ColorU8::rgb((255 / 16 * x) as _, (255 / 16 * y) as _, 0)
    });

    //IoGlobal::save(&img, "_tmp/image3").unwrap();

    //IoData::save(&"hello", "cool").unwrap();

    //let f = FileData::from("hello".to_owned());
    //println!("{:?}", f.get_path());
    */
    //_x();


    
    //let x = AssetData::<String>::from_path_and_value(None, "Toto".to_owned());
    //let x = AssetGlobal::<String>::from_path_and_value(None, "Toto".to_owned());

    //IoData::read_bytes(&mut self, "todo")
    //"grass"
    //"block/dirt"
    /*
    let mut f = FileData::load_or_create("toto", || "hello !".to_owned());
    let mut f = FileData::load_or_create("toto", || "hello !".to_owned());
    let mut f = FileData::load_or_create("toto", || "hello !".to_owned());
    let mut f = FileData::load_or_create("toto", || "hello !".to_owned());
    let mut f = FileData::load_or_create("toto", || "hello !".to_owned());
    f.push_str(" hi !");

    let mut f = AssetData::load_or_create("toto", || "hello !".to_owned());
    let mut f = AssetData::load_or_create("toto", || "hello !".to_owned());
    let mut f = AssetData::load_or_create("toto", || "hello !".to_owned());
    let mut f = AssetData::load_or_create("toto", || "hello !".to_owned());
    let mut f = AssetData::load_or_create("toto", || "hello !".to_owned());
        */
    //IoGlobal.read_bytes("/toto")
    
    //dbg!(f.get_path())

    
}

