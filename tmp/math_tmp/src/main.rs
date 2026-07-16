use hexga::prelude::*;


fn main()
{
    // Not possible in rust
    // let x = [1u8, 2u8] as [f32;2];

    let _ : [f32;2] = [1u8, 2u8].cast_into(); // Need to manually type the binding
    let _ = [1u8, 2u8].to_f32(); // ok

    assert_eq!(
        Vector::<f32,3>::ONE.to_u8(),
        vector3(1u8, 1, 1)
    );

    assert_eq!(
        rgba(1.0f32, 0.5, 0.25, 0.).to_u8_range(), 
        rgba(255u8, 127, 63, 0)
    );

    let img_f32: ImageOf<RgbaOf<f32>> = ImageOf::<RgbaOf<f32>>::from_fn((4,4), |(_x,_y)| RgbaOf::<f32>::RED);
    let _img_u8 = img_f32.to_u8_range();
}
