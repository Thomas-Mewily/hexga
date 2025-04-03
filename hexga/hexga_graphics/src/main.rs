#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(dead_code)]

use hexga_math::prelude::*;

pub struct Image
{

}

fn main()
{
    let g = Grid2::from_fn(64.splat2(), |p| 
    {
        if (p.x + p.y)/2%2==0 { Color::WHITE } else { Color::BLACK }
    });

    let mut raw = Vec::with_capacity(8192);
    let buffered_write = &mut std::io::BufWriter::new(raw);

    /* 
    let r = match "png"
    {
        "png" =>  ::image::ImageEncoder::write_image(::image::codecs::png::PngEncoder::new(buffered_write), &*self.raw_bytes_rgba(), self.width() as _, self.height() as _, LibImageSave::ExtendedColorType::Rgba8),
        "jpg" | "jpeg" | "jpe" |"jif" | "jfif" | "jfi" =>  ::image::ImageEncoder::write_image(::image::codecs::jpeg::JpegEncoder::new(buffered_write), &*self.raw_bytes_rgb(), self.width() as _, self.height() as _, LibImageSave::ExtendedColorType::Rgb8), // jpeg don't support alpha
        _ => { panic!() }
    };
    */
}