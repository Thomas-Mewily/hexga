#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use hexga_asset::prelude::*;

fn main()
{
    let mut hello = Asset::<String>::load_from_value("./tmp/hello", "hello world".to_owned());
    assert_eq!(hello.value(), "hello world");

    let mut goodbye = Asset::<String>::load_from_value("./tmp/hello", "goodbye".to_owned());
    assert_eq!(goodbye.value(), "goodbye");

    // same path, same resource
    assert_eq!(goodbye.value(), hello.value());

    goodbye.save().unwrap();

    let mut second_file = Asset::<String>::load_from_value("./tmp/second_file", "hi".to_owned());


    let assets_string = Assets::<String>::new();
    for assets in assets_string.iter()
    {
        println!("{:?} : {}", assets.path(), assets.value());
    }
}
