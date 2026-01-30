use hexga_map_on::*;

trait MinusOne
{
    const MINUS_ONE: Self;
}

map_on!
(
    (
        i8, i16, i32, i64, isize,
        f32, f64
    ),
    ($name:ident) =>
    {
        impl MinusOne for $name
        {
            const MINUS_ONE : Self = -1 as Self;
        }
    }
);

fn main()
{
    println!("This example impl the MinusOne trait for a lot of type with ease");
    dbg!(i16::MINUS_ONE);

    assert_eq!(i32::MINUS_ONE, -1);
    assert_eq!(f32::MINUS_ONE, -1.);
}
