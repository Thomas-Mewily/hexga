use hexga_save::prelude::*;

fn main()
{
    let path = "./some_text";
    Io.save(path, &"hello").unwrap();
    let txt : String = Io.load(path).unwrap();

    assert_eq!(txt, "hello");
}