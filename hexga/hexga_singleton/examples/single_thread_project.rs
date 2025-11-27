use hexga_singleton::{singleton_single_thread, singleton_single_thread_project};

pub struct CurrentUser
{
    pub city: City,
    pub name: String,
}

pub struct City
{
    pub name: String,
}

singleton_single_thread!(
    pub User, CurrentUser, USER,
    || CurrentUser { name: "Foo".to_owned(), city: City { name: "Paris".to_owned() } }
);

singleton_single_thread_project!(pub UserCity, City, User, city);

fn main() {
    assert_eq!(User.name, "Foo");
    User.name = "Bar".to_owned();
    assert_eq!(User.name, "Bar");

    assert_eq!(UserCity.name, "Paris");
}
