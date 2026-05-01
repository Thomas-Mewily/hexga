use hexga_asset::{AssetState, prelude::*};

fn main()
{
    let hello = Asset::<String>::update_or_create(&"./tmp/hello", "hello world".to_owned());

    assert_eq!(*hello.state(), AssetState::Loaded("hello world".to_owned()));
    assert_eq!(*hello.get().unwrap(), "hello world".to_owned());

    let hi = Asset::<String>::update_or_create(&"./tmp/hello", "hi".to_owned());
    assert!(hello.ptr_eq(&hi));
    assert_eq!(*hi.get().unwrap(), *hello.get().unwrap());
}
