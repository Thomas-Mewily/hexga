

## HexGa Singleton

Define Single and Multi threaded Singleton.

Inspired by [RusPiRo Singleton crate](https://github.com/RusPiRo/ruspiro-singleton)

```rust
use hexga_singleton::prelude::*;

pub struct CurrentUser {
    pub name: String,
}

static USER : SingletonSingleThread<CurrentUser> = SingletonSingleThread::new(|| CurrentUser { name: "Foo".to_owned() });

hexga_singleton::singleton_single_thread_access!(pub User, CurrentUser, USER);

fn main() {
    assert_eq!(User.name, "Foo");
    User.name = "Bar".to_owned();
    assert_eq!(User.name, "Bar");
}
```