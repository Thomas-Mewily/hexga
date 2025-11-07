#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use hexga::prelude::*;



fn main()
{
    let path = path::from_str("./tmp/asset_test/hello");
    let file_content = "hello world";
    file_content.save_to_disk(path).unwrap();

    let hello = String::load_from_disk(path).unwrap();
    assert_eq!(hello, file_content);



    let x = Asset::<String>::load(path);
    let assets_string = Assets::<String>::new();

    for assets in assets_string.iter()
    {
        println!("{} : {}", assets.path(), assets.value());
    }


    println!("hello world");
}
