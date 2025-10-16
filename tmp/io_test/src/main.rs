#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::fmt::Debug;
use std::fs;
use std::hash::RandomState;
use std::path::Path;
use std::{any::Any, sync::Arc};
use hexga::prelude::*;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct Foo
{
    name:String,
    age:int,
}

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Visitor, ser::SerializeStruct};

#[cfg(feature = "hexga_io")]
use hexga_io::{IoLoad, IoSave, Load, Save};

fn t()
{
        let mut table = Table::new();
    table.insert_with_keys(["42".to_owned(), "quarente deux".to_owned()], 42);
    let _3 = table.insert("3".to_owned(), 3).unwrap();
    table.add_key(_3, "|||".to_owned()).unwrap();

    dbg!(&table);

    println!();
    println!();
    println!();
    println!("ron: ");
    println!("{}", table.to_ron().unwrap());

    println!();
    println!("ron back:");
    let table_back : Table<i32> = Table::from_json(&table.to_json().unwrap()).unwrap();
    println!("{}", table_back.to_ron().unwrap());


    assert_eq!(table.to_ron(), table_back.to_ron());


    println!("Hello, world!");
}

fn test_serialize_deserialize<T>(value: &T) where T: IoLoad + IoSave + PartialEq + Debug
{
    println!("   value: {value:?}");
    let format = value.to_quick_bin().unwrap();
    println!("   ron: {}", value.to_ron().unwrap());
    println!("   bin: {format:?}");
    println!();

    // Not a self describing format
    let from_format = T::from_quick_bin_buf(&format).unwrap();
    println!("=>  ron: {}", from_format.to_ron().unwrap());
    println!("=>  bin: {format:?}");
    println!("ron2value: {from_format:?}");
    assert_eq!(*value, from_format);
    println!();
    println!();
    println!();
    println!();
    println!();
}

fn main()
{
    test_serialize_deserialize(&((0..1).map(|i| i.to_string())).to_genvec());

    test_serialize_deserialize(&42);
    test_serialize_deserialize(&12.34);
    test_serialize_deserialize(&'x');
    test_serialize_deserialize(&"abc".to_owned());
    test_serialize_deserialize(&vec![1,2,3,4]);
    test_serialize_deserialize(&((0..5).map(|i| (i.to_string(), i)).to_hashmap()));
    test_serialize_deserialize(&((0..5).map(|i| i.to_string())).to_hashset());

    let mut g = (0..3).map(|i| i.to_string()).to_genvec();
    g.remove_from_index(0);
    g.remove_from_index(1);
    test_serialize_deserialize(&g);

    test_serialize_deserialize(&g);

    let multihashmap = [(["1".to_owned(), "one".to_owned()], 1), (["2".to_owned(), "deux".to_owned()], 2)].to_multihashmap();
    test_serialize_deserialize(&multihashmap);



    //Todo check grid, image and put it in some test
}
