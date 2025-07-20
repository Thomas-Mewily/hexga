use hexga_bitflags::*;

#[bitindex]
#[repr(u8)]
enum Color
{
    Red,
    Greene,
    Blue = 5,
    Yellow, // = 6
    RedAndBlue = Color::Red | Self::Blue, // only defined in ColorFlags
    Purple, // 7
    GreenAndYellowAndPurple = ((Color::Yellow | Self::Purple)) | Self::Greene, // only defined in ColorFlags
}

fn main()
{
    let mut flags = Color::Red | Color::Blue;
    assert_eq!(flags, ColorFlags::RedAndBlue);
    assert_eq!(flags, ColorFlags::Red | ColorFlags::Blue);

    assert!(flags.contains(Color::Red));

    for color in ColorFlags::GreenAndYellowAndPurple
    {
        println!("{:?}", color);
    }

    flags.remove(Color::Red);
    let _blue = Color::try_from(flags).unwrap();
}