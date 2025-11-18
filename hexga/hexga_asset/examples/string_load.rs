use hexga_asset::{AssetState, prelude::*};

fn main()
{
    let hello = Asset::<String>::load_or_create("./tmp/hello", || "hello".to_owned());

    if let Some(content) = hello.get()
    {
        assert_eq!(*content, "hello".to_owned());
        assert_eq!(*hello.state(), AssetState::Loaded("hello".to_owned()));
    }

    let hi = Asset::<String>::load_or_create("./tmp/hello", || "hi".to_owned());
    // Still "hello", because the asset already exist.
    assert!(hello.ptr_eq(&hi));

    if let Some(content) = hi.get()
    {
        assert_eq!(*content, "hello".to_owned());
        assert_eq!(*hi.state(), AssetState::Loaded("hello".to_owned()));
    }
}