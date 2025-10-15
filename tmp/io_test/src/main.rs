#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::fmt::Debug;
use std::fs;
use std::path::Path;
use std::{any::Any, sync::Arc};
use hexga::prelude::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Visitor, ser::SerializeStruct};

#[cfg(feature = "hexga_io")]
use hexga_io::{IoLoad, IoSave, Load, Save};



fn main()
{
    let mut table = Table::new();
    table.insert_with_keys(["42".to_owned(), "quarente deux".to_owned()], 42);
    let _3 = table.insert("3".to_owned(), 3).unwrap();
    table.add_key(_3, "|||".to_owned()).unwrap();

    dbg!(table);


    println!("Hello, world!");
}
