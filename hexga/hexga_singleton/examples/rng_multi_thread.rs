use hexga_singleton::prelude::*;

pub struct CurrentUser {
    pub name: String,
}

singleton_multi_thread!(pub User, CurrentUser, GLOBAL_USER);

fn main() {
    assert!(User::is_not_init());

    // init
    User::replace(Some(CurrentUser { name: "Foo".to_owned() })).unwrap();
    assert!(User::is_init());

    // Singleton access
    let _name = &User.name;

    // de init
    User::replace(None).unwrap();
    assert!(User::is_not_init());
}
