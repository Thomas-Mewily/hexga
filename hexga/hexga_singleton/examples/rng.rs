use hexga_singleton::prelude::*;

pub struct CurrentUser
{
    pub name : String,
}

singleton_thread_local!(pub User, CurrentUser, CURRENT_USER);

// Custom logic to init / deinit the singleton
impl SingletonInit for User
{
    fn replace(value: Option<<Self as SingletonRef>::Target>) {
        CURRENT_USER.replace(value);
    }
}


fn main()
{
    assert!(User::is_not_init());

    // init
    User::replace(Some( CurrentUser { name: "Foo".to_owned() }));
    assert!(User::is_init());

    // Singleton access
    let _the_name = &User.name;

    // de init
    User::replace(None);
    assert!(User::is_not_init());
}
