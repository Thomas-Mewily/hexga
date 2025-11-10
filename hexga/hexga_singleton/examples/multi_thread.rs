use hexga_singleton::prelude::*;

pub struct CurrentUser {
    pub name: String,
}

singleton_multi_thread!(pub User, CurrentUser, GLOBAL_USER, CurrentUser { name: "Foo".to_owned() }, init_once);

fn main() {
    // not init here

    // checking the state of the singleton will init it
    assert!(User::is_init());

    // can't init it again
    assert!(User::replace(Some(CurrentUser { name: "Bar".to_owned() })).is_err());
    assert!(User::is_init());

    // Singleton access
    let _name = &User.name;
}
