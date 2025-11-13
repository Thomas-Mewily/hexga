use hexga_singleton::prelude::*;

pub struct CurrentUser {
    pub name: String,
}

static USER : SingletonSingleThread<CurrentUser> = SingletonSingleThread::new(|| CurrentUser { name: "Foo".to_owned() });

hexga_singleton::singleton_single_thread_deref_to!(USER : CurrentUser => User);

fn main() {
    assert_eq!(User.name, "Foo");
    User.name = "Bar".to_owned();
    assert_eq!(User.name, "Bar");
}
