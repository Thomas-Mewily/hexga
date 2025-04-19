#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::{io::Write, ops::{Index, IndexMut}};

use hexga_base::*;
use hexga_math::{grid::GridBase, grid_param::GridParamBase, prelude::*, Color, ColorByte, IColor};


#[allow(unused_imports)]
#[cfg(feature = "serde")]
pub(crate) use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};
// use hexga_math::prelude::*;

pub type EncodeResult<T=()> = Result<T,String>;

pub type Image<T=ColorByte>=ImageBase<T,int>;

/// Grid wrapper just for the serialization...
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct ImageBase<T, Idx> (GridParamBase<T,GraphicsParam,Idx, 2>)
    where Idx : IntegerIndex, T : IColor;

// To avoid conflict of impl with [IGrid], [IGridView] and [IGridViewMut] when calling get(), get_mut()...
impl<T,Idx> ImageBase<T, Idx> where Idx : IntegerIndex, T : IColor
{
    pub fn get(&self, pos : Vector2::<Idx>) -> Option<&T> { IGrid::get(self, pos) }
    pub fn get_mut(&mut self, pos : Vector2::<Idx>) -> Option<&mut T> { IGrid::get_mut(self, pos) }
    
    pub unsafe fn get_unchecked(&self, pos : Vector2::<Idx>) -> &T { unsafe { IGrid::get_unchecked(self, pos) } }
    pub unsafe fn get_unchecked_mut(&mut self, pos : Vector2::<Idx>) -> &mut T { unsafe { IGrid::get_unchecked_mut(self, pos) } }

    pub fn swap(&mut self, pos_a : Vector2::<Idx>, pos_b : Vector2::<Idx>) -> bool { IGrid::swap(self, pos_a, pos_b) }
    pub fn replace(&mut self, val : T, pos : Vector2::<Idx>) ->  Option<T> { IGrid::replace(self, val, pos) }
    pub fn set(&mut self, val : T, pos : Vector2::<Idx>) -> &mut Self { IGrid::set(self, val, pos) }

    pub fn len(&self) -> usize { IGrid::len(self) }
}

impl<T, Idx> IGridParam<T,GraphicsParam,Idx, 2> for ImageBase<T,Idx>
    where Idx : IntegerIndex, T : IColor
{
    fn grid(&self) -> &GridBase<T,Idx, 2> { self.0.grid() }
    fn grid_mut(&mut self) -> &mut GridBase<T,Idx, 2> { self.0.grid_mut() }

    fn param(&self) -> &GraphicsParam { self.0.param() }
    fn param_mut(&mut self) -> &mut GraphicsParam { self.0.param_mut() }

    fn from_grid_with_param(grid : GridBase<T,Idx, 2>, param : GraphicsParam) -> Self { Self(GridParamBase::from_grid_with_param(grid, param)) }
    fn unpack(self) -> (GridBase<T,Idx, 2>, GraphicsParam) { self.0.unpack() }
}

impl<T, Idx> IGrid<T,GraphicsParam,Idx, 2> for ImageBase<T,Idx>
    where Idx : IntegerIndex, T : IColor
{
    fn values(&self) -> &[T]  { self.0.values() }
    fn values_mut(&mut self) -> &mut [T] { self.0.values_mut() }

    fn into_values(self) -> Vec<T> { self.0.into_values() }

    type Map<Dest>=<GridParamBase<T, GraphicsParam, Idx, 2> as hexga_math::prelude::IGrid<T,GraphicsParam,Idx,2>>::Map<Dest>;

    fn map<Dest, F>(&self, f : F) -> Self::Map<Dest> where F : FnMut(&T) -> Dest, GraphicsParam : Clone {
        self.0.map(f)
    }

    fn transform<Dest, F>(self, f : F) -> Self::Map<Dest> where F : FnMut(T) -> Dest, GraphicsParam : Clone, Self : Sized {
        self.0.transform(f)
    }

    fn crop_margin(&self, margin_start : Vector2::<Idx>, margin_end : Vector2::<Idx>) -> Self where T : Clone, GraphicsParam : Clone, Self : Sized {
        Self(self.0.crop_margin(margin_start, margin_end))
    }

    type View<'a>  = <GridParamBase<T, GraphicsParam, Idx, 2> as hexga_math::prelude::IGrid<T,GraphicsParam,Idx,2>>::View<'a> where Self: 'a;
    fn view<'a>(&'a self) -> Self::View<'a> { self.0.view() }

    type ViewMut<'a>  = <GridParamBase<T, GraphicsParam, Idx, 2> as hexga_math::prelude::IGrid<T,GraphicsParam,Idx,2>>::ViewMut<'a> where Self: 'a;
    fn view_mut<'a>(&'a mut self) -> Self::ViewMut<'a> { self.0.view_mut() }
}

impl<T, Idx> IRectangle<Idx,2> for ImageBase<T,Idx>
where Idx : IntegerIndex, T : IColor
{
    fn size(&self) -> Vector2::<Idx> { self.0.size() }
    fn begin(&self) -> Vector2::<Idx> { self.0.begin() }
}


impl<T, Idx> Index<usize> for ImageBase<T,Idx> where Idx : IntegerIndex, T : IColor
{
    type Output=T;
    fn index(&self, index: usize) -> &Self::Output { self.get_index(index).unwrap() }
}
impl<T, Idx> IndexMut<usize> for ImageBase<T,Idx> where Idx : IntegerIndex, T : IColor
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { self.get_index_mut(index).unwrap() }
}

impl<T, Idx> Index<Vector2::<Idx>> for ImageBase<T,Idx> where Idx : IntegerIndex, T : IColor
{
    type Output=T;
    fn index(&self, index: Vector2::<Idx>) -> &Self::Output { self.get(index).unwrap() }
}
impl<T, Idx> IndexMut<Vector2::<Idx>> for ImageBase<T,Idx> where Idx : IntegerIndex, T : IColor
{
    fn index_mut(&mut self, index: Vector2::<Idx>) -> &mut Self::Output { self.get_mut(index).unwrap() }
}

impl<T, Idx> ImageBase<T,Idx> where Idx : IntegerIndex, T : IColor
{
    pub fn iter(&self) -> hexga_math::grid::Iter<'_,T,Idx,2> { self.0.iter() }
    pub fn iter_mut(&mut self) -> hexga_math::grid::IterMut<'_,T,Idx,2> { self.0.iter_mut() }
}

impl<T, Idx> Length for ImageBase<T,Idx> 
    where Idx : IntegerIndex, T : IColor
{ 
    fn len(&self) -> usize { self.grid().len() }
}

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


impl<T,Idx> ImageBase<T, Idx> where Idx : IntegerIndex, T : IColor
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