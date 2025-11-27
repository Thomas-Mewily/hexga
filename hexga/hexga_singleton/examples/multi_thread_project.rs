use hexga_singleton::{SingletonRead, SingletonWrite, singleton_multi_thread, singleton_multi_thread_project};

pub struct CurrentUser
{
    pub city: City,
    pub name: String,
}

pub struct City
{
    pub name: String,
}

singleton_multi_thread!(
    pub User, CurrentUser, USER,
    || CurrentUser { name: "Foo".to_owned(), city: City { name: "Paris".to_owned() } }
);

singleton_multi_thread_project!(pub UserCity, City, User, city);

fn main() {
    assert_eq!(User::read().name, "Foo");
    User::write().name = "Bar".to_owned();
    assert_eq!(User::read().name, "Bar");

    assert_eq!(UserCity::read().name, "Paris");
}
