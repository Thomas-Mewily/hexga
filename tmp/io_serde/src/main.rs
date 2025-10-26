#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

mod multi_file;
use std::collections::HashMap;

pub use multi_file::ser2::*;

use hexga::io::fs::FsDisk;

// pub trait MetaStringSerializer
// {
//     fn into_markup(self) -> String;
// }
// impl MetaStringSerializer for serde_json::ser::Serializer<String>
// {
//     fn into_markup(self) -> String
//     {
//         let inner = self.into_inner();
//     }
// }

// pub trait MetaSerializerGenerator
// {

// }




#[derive(Serialize, Deserialize)]
struct Person
{
    age: i32,
    name: String,
}

fn test_serialize<T>(val: &T) where T: Serialize
{
    let mut fs = FsDisk;
    match JsonFileSerializer::serialize_and_save(&mut fs, "./tmp/io_serde/test".into(), val, MultiFileSerializerParam { mf_map: false, mf_struct: true })
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
    _mod: i32,
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

    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("_mod", 42);
    map.insert("three", 3);
    map.insert("? invalid file name", 4);
    map.insert("? invalid 2", 5);
    //test_serialize(&map);

    test_serialize(&Foo{ bar: Bar { x: 42, _mod: 99 } });


    //test_serialize(&alice);
    //test_serialize(&vec![1,2,3]);
    //test_serialize(&512);
    //test_serialize(&line!());

    //println!("{}", alice.to_ron().unwrap());
}

fn main()
{
    test_it();


    println!("hello world");
}
