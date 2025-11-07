use hexga_singleton::prelude::*;

pub struct CurrentUser {
    pub name: String,
}

// The last part is the default singleton expression used to automatically init it
singleton_thread_local!(pub User, CurrentUser, GLOBAL_USER, CurrentUser { name: "Foo".to_owned() });

fn main() {

    // Not init here

    // Any attemps to read it (mutably or not with `try_as_ref()` or `try_as_mut()`) will init it,
    // including `User::is_init()` because it just redirect to

    // Singleton access
    let name = &User.name;
    assert_eq!(name, "Foo");

    // de init
    User::replace(None).unwrap();
    // Uninit here

    // But initialized again
    let name = &User.name;
    assert_eq!(name, "Foo");
}
