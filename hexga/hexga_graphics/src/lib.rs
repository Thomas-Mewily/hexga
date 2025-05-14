#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::{io::Write, ops::{Deref, DerefMut, Index, IndexMut, Range}};

use hexga_core::prelude::*;
use hexga_math::{grid::{GridBase, Iter, IterMut}, grid_param::GridParamBase, prelude::*, rectangle::Rectangle, Color, ColorRGBAByte, IColor};


#[allow(unused_imports)]
#[cfg(feature = "serde")]
pub(crate) use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};
// use hexga_math::prelude::*;

pub type EncodeResult<T=()> = Result<T,String>;

pub type Image<T=ColorRGBAByte> = ImageBase<T,int>;

pub struct ImageBase<T=ColorRGBAByte,Idx=int> where T : IColor, Idx : IntegerIndex
{
    pub grid   : GridBase<T,Idx,2>,
    pub param  : GraphicsParam,
}


impl<T, Idx> Deref for ImageBase<T, Idx> where T : IColor, Idx : IntegerIndex {
    type Target=GridBase<T,Idx,2>;
    fn deref(&self) -> &Self::Target { &self.grid }
}
impl<T, Idx> DerefMut for ImageBase<T, Idx> where T : IColor, Idx : IntegerIndex {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.grid }
}

impl<T, Idx> ImageBase<T, Idx> where T : IColor, Idx : IntegerIndex
{
    pub(crate) fn raw_bytes_rgba(&self) -> Vec<u8>
    {
        let mut v = Vec::with_capacity(self.area().to_usize() * 4);

        for y in (0..self.size_y().to_usize()).rev()
        {
            for x in 0..self.size_x().to_usize()
            {
                let pixel = self[vector2(Idx::cast_from(x), Idx::cast_from(y))].to_color_byte();

                v.push(pixel.r);
                v.push(pixel.g);
                v.push(pixel.b);
                v.push(pixel.a);
            }
        }
        v
    }

    pub(crate) fn raw_bytes_rgb(&self) -> Vec<u8>
    {
        let mut v = Vec::with_capacity(self.area().to_usize() * 3);

        for y in (0..self.size_y().to_usize()).rev()
        {
            for x in 0..self.size_x().to_usize()
            {
                let pixel = self[vector2(Idx::cast_from(x), Idx::cast_from(y))].to_color_byte();
                
                v.push(pixel.r);
                v.push(pixel.g);
                v.push(pixel.b);
            }
        }
        v
    }

    pub fn tmp_write_to_png_bytes_inside(&self, path : &str)
    {
        let file = std::fs::File::create(path).expect("Failed to create file");
        let buffered_write = &mut std::io::BufWriter::new(file);

        image::ImageEncoder::write_image(
            image::codecs::png::PngEncoder::new(buffered_write),
            &*self.raw_bytes_rgba(),
            self.width().to_usize() as _,
            self.height().to_usize() as _,
            image::ExtendedColorType::Rgba8,
        ).expect("Failed to write PNG image");
    }
}

//pub type Image<T=ColorByte>=ImageBase<T,int>;


/* 
Todo : Make a declare_grid! macro and declare_gridparam! macro
*/









/* 
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
*/

/* 
impl<T,Idx> ImageBase<T, Idx> where Idx : IntegerIndex
{
    
    pub(crate) fn raw_bytes_rgba(&self) -> Vec<u8>
    {
        let mut v = Vec::with_capacity(self.area().to_usize() * 4);

        for y in (0..self.size_y().to_usize()).rev()
        {
            for x in 0..self.size_x().to_usize()
            {
                let pixel = self[vector2(Idx::cast_from(x), Idx::cast_from(y))].to_rgba_u8();

                v.push(pixel.r);
                v.push(pixel.g);
                v.push(pixel.b);
                v.push(pixel.a);
            }
        }
        v
    }

    pub(crate) fn raw_bytes_rgb(&self) -> Vec<u8>
    {
        let mut v = Vec::with_capacity(self.area().to_usize() * 3);

        for y in (0..self.size_y().to_usize()).rev()
        {
            for x in 0..self.size_x().to_usize()
            {
                let pixel = self[vector2(Idx::cast_from(x), Idx::cast_from(y))].to_rgba_u8();
                
                v.push(pixel.r);
                v.push(pixel.g);
                v.push(pixel.b);
            }
        }
        v
    }

    pub fn tmp_write_to_png_bytes_inside(&self, path : &str)
    {
        let file = std::fs::File::create(path).expect("Failed to create file");
        let buffered_write = &mut std::io::BufWriter::new(file);

        image::ImageEncoder::write_image(
            image::codecs::png::PngEncoder::new(buffered_write),
            &*self.raw_bytes_rgba(),
            self.width().to_usize() as _,
            self.height().to_usize() as _,
            image::ExtendedColorType::Rgba8,
        ).expect("Failed to write PNG image");
    }
}



*/


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
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