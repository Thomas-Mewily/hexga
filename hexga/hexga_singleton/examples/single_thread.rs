use hexga_singleton::prelude::*;

pub struct CurrentUser {
    pub name: String,
}

static USER : SingletonSingleThread<CurrentUser> = SingletonSingleThread::new(|| CurrentUser { name: "Foo".to_owned() });

fn main() {

    assert_eq!(USER.read().name, "Foo");
    USER.write().name = "Bar".to_owned();
    assert_eq!(USER.read().name, "Bar");
}
