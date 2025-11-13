use hexga_singleton::prelude::*;

pub struct CurrentUser {
    pub name: String,
}

static USER : SingletonMultiThread<CurrentUser> = SingletonMultiThread::new(|| CurrentUser { name: "Foo".to_owned() });

fn main() {

    assert_eq!(USER.instance().name, "Foo");
    USER.instance_mut().name = "Bar".to_owned();
    assert_eq!(USER.instance().name, "Bar");
}
