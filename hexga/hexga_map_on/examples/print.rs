use hexga_map_on::prelude::*;

macro_rules! print_type {
    ($type_name:ty) => {
        println!("type {}", ::std::any::type_name::<$type_name>());
    };
}

fn main() 
{
    // manual stuff
    print_type!(f32);
    print_type!(f64);

    println!("---");

    // with a macro
    map_on!((f32, f64), print_type);
    
    println!("---");
    println!("Hello, world!");
}