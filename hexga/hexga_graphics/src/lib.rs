#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::io::Write;

use hexga_math::{grid_param::GridParamBase, prelude::*};


#[allow(unused_imports)]
#[cfg(feature = "serde")]
pub(crate) use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};
// use hexga_math::prelude::*;

pub type EncodeResult<T=()> = Result<T,String>;
/* 

pub trait IoWrite
{
    fn to_bytes(&self, extension : &str) -> EncodeResult<Vec<u8>> 
    { 
        let mut r = Vec::new();
        self.to_bytes_in(extension, &mut r);
        Ok(r)
    }
    fn to_bytes_in<W: Write>(&self, extension : &str, inside : &mut W) -> EncodeResult;
}

pub struct IoWrapper<'a, T>(&'a T) where T : ToIo;


pub trait ToIo
{
    fn to_io<'a>(&'a self) -> IoWrapper<'a> { IoWrapper::new() }
}

impl IoWrite for Image<>

pub trait ImageExtension
{
    fn to_bytes_inside(&self)

    fn to_png_bytes_inside<W: Write>(&self, inside : &mut W) -> EncodeResult
    {
        let buffered_write = &mut std::io::BufWriter::new(inside);

        image::ImageEncoder::write_image(LibImageSave::codecs::png::PngEncoder::new(buffered_write), &*self.raw_bytes_rgba(), self.width() as _, self.height() as _, LibImageSave::ExtendedColorType::Rgba8)

        let r = match extension
        {
            "png" =>  ,
            "jpg" | "jpeg" | "jpe" |"jif" | "jfif" | "jfi" =>  LibImageSave::ImageEncoder::write_image(LibImageSave::codecs::jpeg::JpegEncoder::new(buffered_write), &*self.raw_bytes_rgb(), self.width() as _, self.height() as _, LibImageSave::ExtendedColorType::Rgb8), // jpeg don't support alpha
            _ => { return Err(Self::save_file_extension_not_supported(extension)); }
        };

        todo!()
    }
}

pub type Image<T>=ImageBase<T,int>;
pub type ImageBase<T,Idx> = GridParamBase<T,GraphicsParam,Idx,2>;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Clone, Copy, Hash)]
pub struct GraphicsParam
{
    aa : AntiAliasing,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum AntiAliasing
{
    #[default]
    Linear,
    /// Ideal for Pixel Art
    Nearest,
}
*/