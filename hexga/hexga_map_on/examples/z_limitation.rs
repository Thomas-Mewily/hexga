use hexga_map_on::*;

macro_rules! print_type {
    ($type_name:ty) => {
        println!(
            "print type from macro name {}",
            ::std::any::type_name::<$type_name>()
        );
    };
}

fn main()
{
    // work fine
    map_on!((f32, f64), print_type);

    // Don't work :/
    /*
    map_on!((f32, f64),
        ($type_name:ident) =>
        {
            println!("print type from macro lambda {}", ::std::any::type_name::<$type_name>());
        }
    );
    */

    // Check the README.md to get for information about it
}
