use hexga_map_on::prelude::*;

trait One
{
    const ONE : Self;
}

macro_rules! impl_one {
    ($type_name:ty) => {
        impl One for $type_name
        {
            const ONE : Self = 1 as Self;
        }
    };
}

map_on_number!(impl_one);

/*
/*  This macro will be expanded into somethings like : */
map_on!
(
    (
        i8, i16, i32, i64, isize,
        u8, u16, u32, u64, usize,
        f32, f64
    ), 
    impl_one
);

*/

fn main() 
{
    println!("This example impl the One trait for a lot of type with ease");
    dbg!(i16::ONE);

    assert_eq!(i32::ONE  , 1);
    assert_eq!(usize::ONE, 1);
    assert_eq!(f32::ONE  , 1.);

    assert_ne!(usize::ONE, 42);
}