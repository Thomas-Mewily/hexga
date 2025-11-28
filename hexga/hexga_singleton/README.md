

## HexGa Singleton

Define Single and Multi threaded Singleton.

Inspired by [RusPiRo Singleton crate](https://github.com/RusPiRo/ruspiro-singleton)

```rust
use hexga_singleton::singleton_single_thread;

pub struct CurrentUser {
    pub name: String,
}

singleton_single_thread!(
    pub User, CurrentUser, USER,
    || CurrentUser { name: "Foo".to_owned() }
);

fn main() {
    assert_eq!(User.name, "Foo");
    User.name = "Bar".to_owned();
    assert_eq!(User.name, "Bar");
}

```