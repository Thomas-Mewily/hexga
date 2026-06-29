use hexga::prelude::*;

fn main()
{
    let mut f = FileData::load_or_create("hello", || "hi".to_owned());
    dbg!(f.get_path());
    f.push_str(" goodbye!");
}
