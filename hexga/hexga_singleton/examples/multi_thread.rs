use hexga_singleton::{SingletonRead, SingletonWrite, singleton_multi_thread};

pub struct CurrentUser {
    pub name: String,
}

singleton_multi_thread!(
    pub User, CurrentUser, USER,
    || CurrentUser { name: "Foo".to_owned() }
);

fn main() {
    assert_eq!(User::read().name, "Foo");
    User::write().name = "Bar".to_owned();
    assert_eq!(User::read().name, "Bar");
}
