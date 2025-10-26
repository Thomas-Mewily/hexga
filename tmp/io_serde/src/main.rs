#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

mod multi_file;
pub use multi_file::ser::*;

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
    JsonFileSerializer::new_and_serialize(&mut fs, "./tmp/io_serde/test".into(), val).unwrap();
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
    map.insert("three", 3);
    test_serialize(&map);

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
