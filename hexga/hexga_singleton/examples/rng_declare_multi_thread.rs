use hexga_singleton::prelude::*;

pub struct CurrentUser {
    pub name: String,
}

singleton_declare_multi_thread!(pub User, CurrentUser, GLOBAL_USER);

impl SingletonReplace for User {
    fn replace(value: Option<<Self as SingletonRef>::Target>) -> SingletonResult {
        let cell = GLOBAL_USER.get_or_init(|| ::std::sync::RwLock::new(None));
        match cell.write() {
            Ok(mut guard) => { *guard = value; Ok(()) },
            Err(_poisoned) => Err(()),
        }
    }
}

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
