use hexga_singleton::{singleton_single_thread, singleton_single_thread_access};

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

singleton_single_thread_access!(pub UserCity, City,
    {User::try_read().map(|v| &v.value.city) },
    {User::try_write().map(|v| &mut v.value.city)}
);

fn main() {
    assert_eq!(User.name, "Foo");
    User.name = "Bar".to_owned();
    assert_eq!(User.name, "Bar");

    assert_eq!(UserCity.name, "Paris");
}
