use hexga_singleton::prelude::*;

pub struct CurrentUser {
    pub name: String,
}

static USER : SingletonMultiThread<CurrentUser> = SingletonMultiThread::new(|| CurrentUser { name: "Foo".to_owned() });

fn main() {

    assert_eq!(USER.read().name, "Foo");
    USER.write().name = "Bar".to_owned();
    assert_eq!(USER.write().name, "Bar");
}
