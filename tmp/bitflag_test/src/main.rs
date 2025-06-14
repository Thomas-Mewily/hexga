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
    assert_eq!(TeamFlags::BlueOrRed, Team::Blue | Team::Red);
    dbg!(Team::Blue);
    dbg!(TeamFlags::Blue);
    dbg!(TeamFlags::BlueOrRed);


    //let x = Team::Blue | Team::Green;
    //TeamFlags::BlueOrRed

    let x = TeamFlags::from(Team::Blue);


    let t = Team::Green;
    for t in TeamFlags::BlueOrRed | Team::Green
    {
        dbg!(t);
    }

    //dbg!(x);

    //let combo = TeamFlags::Blue | TeamFlags::Red;

    /*
    println!("Combo: {}", combo.flags);
    for team in combo.iter() {
        println!("Contains: {:?}", team as u8);
    }
    */
    println!("hello");
}
