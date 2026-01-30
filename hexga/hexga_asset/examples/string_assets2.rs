#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use hexga_asset::{AssetState, prelude::*};
use std::{ops::Deref, thread, time::Duration};

fn main()
{
    let mut hello = Asset::<String>::update_or_create(&"./tmp/hello", "hello world".to_owned());
    let mut hi = Asset::<String>::update_or_create(&"./tmp/hello", "world".to_owned());
}
