use hexga::prelude::*;


fn main()
{
    // Not possible in rust
    // let x = [1u8, 2u8] as [f32;2];

    let _ : [f32;2] = [1u8, 2u8].cast_into(); // manually typing it is needed
    let _ = [1u8, 2u8].to_f32();

    assert_eq!(
        rgba(1.0f32, 0.5, 0.25, 0.).to_u8_range(), 
        rgba(255u8, 127, 63, 0)
    );
}
