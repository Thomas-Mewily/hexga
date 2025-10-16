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
    println!("=>  bin: {:?}", from_format.to_quick_bin());
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
    test_serialize_deserialize(&point2(10, 20)); // Todo serialize using tuple
    test_serialize_deserialize(&45.degree());
    test_serialize_deserialize(&45.s());

    /*
    */
    //Todo check grid, image and put it in some test to check it
}
