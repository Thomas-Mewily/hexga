use hexga_bitflags_macro::*;

#[bitflags]
#[repr(u8)]
enum Team {
    Blue,
    Red,
    Yellow = 5,
    Green, // = 6
    BlueOrRed = (Team::Blue | Self::Red),
    Purple, // 7
}


fn main()
{
    let mut acc = TeamFlags::EMPTY;
    for x in Team::ALL
    {
        acc |= x;
        dbg!(x);
        dbg!(acc);
    }
    println!("hello");
}
