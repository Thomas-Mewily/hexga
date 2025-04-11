use crate::*;

pub trait ToColorRep
{
    type ColorRGBA;
    fn to_rgba_coef(&self) -> Self::ColorRGBA;

    type ColorHSLA;
    fn to_hsla(&self) -> Self::ColorHSLA;

    type ColorRGBAByte;
    fn to_rgba_u8(&self) -> Self::ColorRGBAByte;

    fn to_color(&self) -> Self::ColorRGBA { self.to_rgba_coef() }
}

pub trait IColor : 
    From<Color> + From<ColorByte> + From<ColorHSLA> +
    ToColorRep<ColorRGBA = ColorRGBA, ColorHSLA = ColorHSLA, ColorRGBAByte = ColorRGBAByte>
    //Into<Color> + Into<ColorByte> + Into<ColorHSLA>
{
    const TRANSPARENT : Self;

    const BLACK : Self;
    const GRAY  : Self;
    const WHITE : Self;
    
    const RED    : Self; 
    const GREEN  : Self; 
    const BLUE   : Self; 
    
    const CYAN   : Self; 
    const PINK   : Self; 
    const YELLOW : Self;

    fn rgba_from_bytes_slice(rgba : &[u8]) -> Self { Self::rgba_from_bytes(rgba[0], rgba[1], rgba[2], rgba[3]) }
    fn rgba_from_bytes(r : u8, g : u8, b : u8, a : u8) -> Self;

    fn from_rgb_hex(hex: u32) -> Self 
    {
        let bytes: [u8; 4] = hex.to_be_bytes();
        Self::rgba_from_bytes_slice(&bytes)
    }
    fn from_rgba_hex(hex: u32) -> Self 
    {
        let mut bytes: [u8; 4] = hex.to_be_bytes();
        bytes[3] = u8::MAX;
        Self::rgba_from_bytes_slice(&bytes)
    }

    /// Cast to color byte and convert to u32 using : `#RRGGBBAA`
    fn to_rgba_hex(self) -> u32 
    { 
        let ColorByte { r, g, b, a } = self.to_rgba_u8(); 
        ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | (a as u32) 
    }

    /// Cast to color byte and format the color : `#RRGGBBAA`
    fn to_rgba_hex_string(self) -> String
    {
        let rgba = self.to_rgba_u8();
        format!(
            "#{:02X}{:02X}{:02X}{:02X}",
            rgba.r, 
            rgba.g,  
            rgba.b,       
            rgba.a,
        )
    }
}