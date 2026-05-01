use hexga_map_on::*;

trait Zero
{
    const ZERO: Self;
}

map_on_number!(
    ($name:ident) =>
    {
        impl Zero for $name
        {
            const ZERO : Self = 0 as Self;
        }
    }
);

fn main()
{
    println!("This example impl the Zero trait for a lot of type with ease");
    dbg!(i16::ZERO);

    assert_eq!(i32::ZERO, 0);
    assert_eq!(usize::ZERO, 0);
    assert_eq!(f32::ZERO, 0.);

    assert_ne!(usize::ZERO, 42);
}
