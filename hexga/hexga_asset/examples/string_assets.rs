#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use hexga_asset::prelude::*;

/*
fn main()
{
    let mut hello = Asset::<String>::load_or_create("./tmp/hello", "hi".to_owned());
    let mut hello = Asset::<String>::load_or_create("./tmp/hello", "hello world".to_owned());
    assert_eq!(hello.try_value(), Ok(Arc::new("hello world".to_owned())));

    let mut goodbye = Asset::<String>::load_or_create("./tmp/hello", "goodbye".to_owned());
    assert_eq!(goodbye.try_value(), Ok(Arc::new("goodbye".to_owned())));

    // same path, same resource
    assert_eq!(goodbye.try_value(), hello.try_value());

    goodbye.downgrade()
    goodbye.save().unwrap();

    let mut second_file = Asset::<String>::update_or_create_with_value("./tmp/second_file", "hi".to_owned());


    let assets_string = Asset::<String>::manager();
    for assets in assets_string.iter()
    {
        println!("{:?} : {}", assets.path(), assets.value());
    }
}
    */