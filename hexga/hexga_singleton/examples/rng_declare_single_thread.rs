use hexga_singleton::prelude::*;

pub struct CurrentUser
{
    pub name : String,
}

singleton_declare_thread_local!(pub User, CurrentUser, GLOBAL_USER);

// Custom logic to init / deinit the singleton
impl SingletonInit for User
{
    fn replace(value: Option<<Self as SingletonRef>::Target>) -> SingletonResult {
        GLOBAL_USER.replace(value);
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
    let _name = &User.name;

    // de init
    User::replace(None).unwrap();
    assert!(User::is_not_init());
}
