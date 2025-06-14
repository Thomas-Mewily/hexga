#[cfg(test)]
mod tests {
    use hexga_bitflags_macro::*;

    #[bitflags]
    #[repr(u16)]
    enum Color
    {
        Red,
        Green,
        Blue = 5,
        Yellow, // = 6
        RedAndBlue = Color::Red | Self::Blue,
        Purple, // 7
        GreenAndYellowAndPurple = ((Color::Yellow | Self::Purple)) | Self::Green,
    }

    #[test]
    fn increment_variant()
    {
        assert_eq!(Color::Red   as u8,  0);
        assert_eq!(Color::Red.index(),  0);
        assert_eq!(Color::Red.bits(),   0b1);
        assert_eq!(Color::Red.flags().bits(), 0b1);

        assert_eq!(Color::Green   as u8,  1);
        assert_eq!(Color::Green.index(),  1);
        assert_eq!(Color::Green.bits(),   0b10);
        assert_eq!(Color::Green.flags().bits(), 0b10);

        assert_eq!(Color::Blue   as u8,  5);
        assert_eq!(Color::Blue.index(),  5);
        assert_eq!(Color::Blue.bits(),   0b100000);
        assert_eq!(Color::Blue.flags().bits(), 0b100000);

        assert_eq!(Color::Yellow   as u8,  6);
        assert_eq!(Color::Yellow.index(),  6);
        assert_eq!(Color::Yellow.bits(),   0b1000000);
        assert_eq!(Color::Yellow.flags().bits(), 0b1000000);

        assert_eq!(Color::Purple   as u8,  7);
        assert_eq!(Color::Purple.index(),  7);
        assert_eq!(Color::Purple.bits(),   0b10000000);
        assert_eq!(Color::Purple.flags().bits(), 0b10000000);


        assert_eq!(Color::Red | Color::Blue, ColorFlags::RedAndBlue);
        assert_eq!(ColorFlags::Red | Color::Blue, ColorFlags::RedAndBlue);
        assert_eq!(ColorFlags::Blue | Color::Red, ColorFlags::RedAndBlue);
        assert_eq!(Color::Red | ColorFlags::Blue, ColorFlags::RedAndBlue);
        assert_eq!(Color::Blue | ColorFlags::Red, ColorFlags::RedAndBlue);
        assert_eq!(ColorFlags::Blue | ColorFlags::Red, ColorFlags::RedAndBlue);
        assert_eq!(ColorFlags::Red | ColorFlags::Blue, ColorFlags::RedAndBlue);

        assert_eq!(ColorFlags::GreenAndYellowAndPurple, Color::Green | Color::Yellow | Color::Purple);

        assert_eq!(ColorFlags::RedAndBlue.collect::<Vec<_>>(), vec![Color::Red, Color::Blue]);
        assert_eq!(ColorFlags::GreenAndYellowAndPurple.collect::<Vec<_>>(), vec![Color::Green, Color::Yellow, Color::Purple]);
    }
}