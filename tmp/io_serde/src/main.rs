#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

mod multi_file;
use std::collections::HashMap;
use hexga::prelude::*;

pub use multi_file::ser2::*;

use hexga::io::fs::FsDisk;






#[derive(Serialize, Deserialize)]
struct Person
{
    age: i32,
    name: String,
}

fn test_serialize<T>(val: &T) where T: Serialize
{
    let mut fs = FsDisk;
    match JsonFileSerializer::serialize_and_save(&mut fs, "./tmp/io_serde/test2".into(), val, MultiFileSerializerParam { mf_map: false, mf_struct: true, ..Default::default() })
    {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    }
}

#[derive(Serialize, Deserialize)]
struct Foo
{
    bar: Bar,
}

#[derive(Serialize, Deserialize)]
struct Bar
{
    x: i32,
    __mod: i32,
    image: Image,
}

fn test_it()
{
    //let c : serde_json::ser::Compound<'static,String,serde_json::ser::CompactFormatter> = serde_json::ser::Compound::
    //test_serialize(true);
    //test_serialize("ok");

    let alice = Person {
        name: "Alice".into(),
        age: 30,
    };

    // let mut map = HashMap::new();
    // map.insert("one", 1);
    // map.insert("two", 2);
    // map.insert("_mod", 42);
    // map.insert("__mod", 42);
    // map.insert("three", 3);
    // map.insert("? invalid file name", 4);
    // map.insert("? invalid 2", 5);
    // //map.save_to_disk("./tmp/io_serde/mymap").unwrap();
    // //test_serialize(&map);
    // map.save_to_disk("./tmp/io_serde/test2.json").unwrap()
    //test_serialize(&Foo{ bar: Bar { x: 42, __mod: 99, image: Image::from_fn((1,1), |p| ColorU8::RED) } });

    let mut map = HashMap::new();
    map.insert("one", "un");
    map.insert("two", "deux");
    map.insert("__mod", "__mod2");
    map.save_to_disk("./tmp/io_serde/test2.json").unwrap()


    //let mapback = HashMap::load

    //test_serialize(&alice);
    //test_serialize(&vec![1,2,3]);
    //test_serialize(&512);
    //test_serialize(&line!());

    //println!("{}", alice.to_ron().unwrap());
}

fn main()
{
    test_it();

    // let img = Image::load_from_disk("./tmp/io_serde/smiley").unwrap();
    // img.save_to_disk("./tmp/io_serde/smiley3").unwrap();
    // dbg!(&img);
    // println!("{}", img.to_url("png").unwrap());


    println!("hello world");
}
