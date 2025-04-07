use hexga_map_on::*;

trait Zero
{
    const ZERO : Self;
}

macro_rules! impl_zero {
    ($type_name:ty) => {
        impl Zero for $type_name
        {
            const ZERO : Self = 0 as Self;
        }
    };
}

map_on!
(
    (
        i8, i16, i32, i64, isize,
        u8, u16, u32, u64, usize,
        f32, f64
    ), 
    impl_zero
);

fn main() 
{
    println!("This example impl the Zero trait for a lot of type with ease");
    dbg!(i16::ZERO);

    assert_eq!(i32::ZERO  , 0);
    assert_eq!(usize::ZERO, 0);
    assert_eq!(f32::ZERO  , 0.);

    assert_ne!(usize::ZERO, 1);
}