

## HexGa Singleton

Define Single and Multi threaded Singleton.

Inspired by [RusPiRo Singleton crate](https://github.com/RusPiRo/ruspiro-singleton)


```rust
use hexga_singleton::prelude::*;

pub struct CurrentUser
{
    pub name : String,
}

singleton_thread_local!(pub User, CurrentUser, CURRENT_USER);

// Custom logic to init / deinit the singleton
impl SingletonReplace for User
{
    fn replace(value: Option<<Self as SingletonRef>::Target>) -> SingletonResult {
        CURRENT_USER.replace(value);
        Ok(())
    }
}


fn main()
{
    assert!(User::is_not_init());

    // init
    User::replace(Some( CurrentUser { name: "Foo".to_owned() })).unwrap();
    assert!(User::is_init());

    // Singleton access
    let name = &User.name;

    // de init
    User::replace(None).unwrap();
    assert!(User::is_not_init());
}
```