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

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Visitor, ser::SerializeStruct};

#[cfg(feature = "hexga_io")]
use hexga_io::{IoLoad, Save, Load, Save};


pub struct Fs
{
    name: String,
    kind: FsKind,
}

pub enum FsKind
{
    File(FsFile),
    Folder(FsFolder),
}

pub struct FsFile
{
    content: Vec<u8>,
}

pub struct FsFolder
{
    childs: Vec<Fs>,
}


fn main()
{
    let img = Image::from_fn_coef((4,4), |p| RgbaFloat::rgb(p.x, p.y, 0.).to_u8_range());
    dbg!(&img);

    let path = "./tmp/io_test/myimg";
    img.save_to_disk(path).unwrap();

    let img = Image::load_from_disk(path).unwrap();
    dbg!(&img);


    //42.save_to_disk(path)

    //img.save_to_disk()

    println!("hello world");
}
