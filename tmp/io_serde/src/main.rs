#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::collections::HashMap;
use hexga::prelude::*;


fn main()
{
    let mut map = HashMap::new();
    map.insert("one", "un");
    map.insert("two", "deux");
    map.insert("okidoki", "yo");
    map.insert("?!", "!");
    map.insert("?!2", "!");
    map.insert("__mod", "!");
    map.save_to_disk("./tmp/io_serde/test2").unwrap();

    // test_it();

    // let img = Image::load_from_disk("./tmp/io_serde/smiley").unwrap();
    // img.save_to_disk("./tmp/io_serde/smiley3").unwrap();
    // dbg!(&img);
    // println!("{}", img.to_url("png").unwrap());

    let img = Image::from_fn((256,256), |(x,y)| ColorU8::rgb(x as u8, y as u8, 255));
    img.save_to_disk("./tmp/io_serde/smiley5").unwrap();
    let img_loaded = Image::load_from_disk("./tmp/io_serde/smiley5").unwrap();
    assert_eq!(img, img_loaded);

    println!("hello world");
}
