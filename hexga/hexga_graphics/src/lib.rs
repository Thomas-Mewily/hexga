#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::{io::Write, ops::{Index, IndexMut}};

use hexga_math::{grid::GridBase, grid_param::GridParamBase, prelude::*, IColor};


#[allow(unused_imports)]
#[cfg(feature = "serde")]
pub(crate) use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};
// use hexga_math::prelude::*;

pub type EncodeResult<T=()> = Result<T,String>;

/// Grid wrapper just for the serialization...
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct ImageBase<T, Idx, const N : usize> (GridParamBase<T,GraphicsParam,Idx, N>)
    where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>, T : IColor;

// To avoid conflict of impl with [IGrid], [IGridView] and [IGridViewMut] when calling get(), get_mut()...
impl<T,Idx, const N : usize> ImageBase<T, Idx, N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>, T : IColor
{
    pub fn get(&self, pos : Vector<Idx,N>) -> Option<&T> { IGrid::get(self, pos) }
    pub fn get_mut(&mut self, pos : Vector<Idx,N>) -> Option<&mut T> { IGrid::get_mut(self, pos) }
    
    pub unsafe fn get_unchecked(&self, pos : Vector<Idx,N>) -> &T { unsafe { IGrid::get_unchecked(self, pos) } }
    pub unsafe fn get_unchecked_mut(&mut self, pos : Vector<Idx,N>) -> &mut T { unsafe { IGrid::get_unchecked_mut(self, pos) } }

    pub fn swap(&mut self, pos_a : Vector<Idx,N>, pos_b : Vector<Idx,N>) -> bool { IGrid::swap(self, pos_a, pos_b) }
    pub fn replace(&mut self, val : T, pos : Vector<Idx,N>) ->  Option<T> { IGrid::replace(self, val, pos) }
    pub fn set(&mut self, val : T, pos : Vector<Idx,N>) -> &mut Self { IGrid::set(self, val, pos) }

    pub fn len(&self) -> usize { IGrid::len(self) }
}

impl<T, Idx, const N : usize> IGridParam<T,GraphicsParam,Idx, N> for ImageBase<T,Idx,N>
    where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>, T : IColor
{
    fn grid(&self) -> &GridBase<T,Idx, N> { self.0.grid() }
    fn grid_mut(&mut self) -> &mut GridBase<T,Idx, N> { self.0.grid_mut() }

    fn param(&self) -> &GraphicsParam { self.0.param() }
    fn param_mut(&mut self) -> &mut GraphicsParam { self.0.param_mut() }

    fn from_grid_with_param(grid : GridBase<T,Idx, N>, param : GraphicsParam) -> Self { Self(GridParamBase::from_grid_with_param(grid, param)) }
    fn unpack(self) -> (GridBase<T,Idx, N>, GraphicsParam) { self.0.unpack() }
}

impl<T, Idx, const N : usize> IGrid<T,GraphicsParam,Idx, N> for ImageBase<T,Idx,N>
    where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>, T : IColor
{
    fn values(&self) -> &[T]  { self.0.values() }
    fn values_mut(&mut self) -> &mut [T] { self.0.values_mut() }

    fn into_values(self) -> Vec<T> { self.0.into_values() }

    type Map<Dest>=<GridParamBase<T, GraphicsParam, Idx, N> as hexga_math::prelude::IGrid<T,GraphicsParam,Idx,N>>::Map<Dest>;

    fn map<Dest, F>(&self, f : F) -> Self::Map<Dest> where F : FnMut(&T) -> Dest, GraphicsParam : Clone {
        self.0.map(f)
    }

    fn transform<Dest, F>(self, f : F) -> Self::Map<Dest> where F : FnMut(T) -> Dest, GraphicsParam : Clone, Self : Sized {
        self.0.transform(f)
    }

    fn crop_margin(&self, margin_start : Vector<Idx,N>, margin_end : Vector<Idx,N>) -> Self where T : Clone, GraphicsParam : Clone, Self : Sized {
        Self(self.0.crop_margin(margin_start, margin_end))
    }

    type View<'a>  = <GridParamBase<T, GraphicsParam, Idx, N> as hexga_math::prelude::IGrid<T,GraphicsParam,Idx,N>>::View<'a> where Self: 'a;
    fn view<'a>(&'a self) -> Self::View<'a> { self.0.view() }

    type ViewMut<'a>  = <GridParamBase<T, GraphicsParam, Idx, N> as hexga_math::prelude::IGrid<T,GraphicsParam,Idx,N>>::ViewMut<'a> where Self: 'a;
    fn view_mut<'a>(&'a mut self) -> Self::ViewMut<'a> { self.0.view_mut() }
}

impl<T, Idx, const N : usize> IRectangle<Idx,N> for ImageBase<T,Idx,N>
where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>, T : IColor
{
    fn size(&self) -> Vector<Idx,N> { self.0.size() }
    fn begin(&self) -> Vector<Idx,N> { self.0.begin() }
}


impl<T, Idx, const N : usize> Index<usize> for ImageBase<T,Idx, N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>, T : IColor
{
    type Output=T;
    fn index(&self, index: usize) -> &Self::Output { self.get_index(index).unwrap() }
}
impl<T, Idx, const N : usize> IndexMut<usize> for ImageBase<T,Idx, N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>, T : IColor
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { self.get_index_mut(index).unwrap() }
}

impl<T, Idx, const N : usize> Index<Vector<Idx,N>> for ImageBase<T,Idx, N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>, T : IColor
{
    type Output=T;
    fn index(&self, index: Vector<Idx,N>) -> &Self::Output { self.get(index).unwrap() }
}
impl<T, Idx, const N : usize> IndexMut<Vector<Idx,N>> for ImageBase<T,Idx, N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>, T : IColor
{
    fn index_mut(&mut self, index: Vector<Idx,N>) -> &mut Self::Output { self.get_mut(index).unwrap() }
}

impl<T, Idx, const N : usize> ImageBase<T,Idx, N> where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>, T : IColor
{
    pub fn iter(&self) -> hexga_math::grid::Iter<'_,T,Idx,N> { self.0.iter() }
    pub fn iter_mut(&mut self) -> hexga_math::grid::IterMut<'_,T,Idx,N> { self.0.iter_mut() }
}

impl<T, Idx, const N : usize> have_len::HaveLen for ImageBase<T,Idx, N> 
    where Idx : IntegerIndex, usize : CastTo<Idx>, isize : CastTo<Idx>, T : IColor
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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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