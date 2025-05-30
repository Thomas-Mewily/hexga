use crate::*;

pub type Image<C=ColorRGBAByte> = ImageBase<C,int>;
pub struct ImageBase<C,Idx> where Idx : Integer
{
    pixels : Vec<C>,
    size   : Vector2<Idx>,
}

pub trait ToImage
{
    type Output;
    fn to_image(self) -> Self::Output;
}

impl<'a,T,Idx> ToImage for GridView<'a,GridBase<T,Idx,2>,T,Idx,2> where T : ToColor, Idx : Integer
{
    type Output = ImageBase<T, Idx>;
    fn to_image(mut self) -> Self::Output
    {
        /*
        let (w,h) = self.size().into();

        for y in (0..h /2).step_by()
        {
            for x in 0..w
            {
                self.swap(vec2(x, y), vec2(x, h-y-1));
            }
        }
        let (size, values) = self.into_size_and_values();
        */
        todo!();
    }
}

/*
impl<'a,G,T,Idx> ToImage for GridView<'a,G,T,Idx,2> where T : ToColor, Idx : Integer, G : IGrid<T,Idx,2>
{
    type Output = Self;
    fn to_image(self) -> Self::Output
    {
        self.to_grid()
    }
}
*/

impl<C,Idx> ImageBase<C,Idx> where Idx : Integer
{
    pub fn pixels(&self) -> &[C] { self.values() }
    pub fn pixels_mut(&mut self) -> &mut[C] { self.values_mut() }

    /// `self.view().crop_intersect(subrect).to_grid()`
    fn subimage(&self, rect : Rectangle2<Idx>) -> Self where C : Clone { self.subgrid(rect) }
}

impl<C,Idx> ImageBase<C,Idx> where Idx : Integer
{
    pub(crate) fn flip_y(&self, mut pos : Vector2::<Idx>) -> Vector2::<Idx>
    { unsafe { Self::external_position_to_position_unchecked(pos, self.size) } }
}


impl<C,Idx> IGrid<C,Idx,2> for ImageBase<C,Idx> where Idx : Integer
{
    type WithType<U> = ImageBase<U,Idx>;

    fn values(&self) -> &[C] { &self.pixels }
    fn values_mut(&mut self) -> &mut [C] { &mut self.pixels }

    fn into_size_and_values(self) -> (Vector2<Idx>, Vec<C>) { (self.size, self.pixels) }

    unsafe fn unchecked_from_vec(size : Vector2::<Idx>, pixels : Vec<C>) -> Self {
        Self { pixels, size }
    }

    unsafe fn position_to_index_unchecked(&self, pos : Vector2<Idx>) -> usize { unsafe { Vector2::<Idx>::to_index_unchecked(self.flip_y(pos), self.size()) } }
    unsafe fn index_to_position_unchecked(&self, index : usize) -> Vector2<Idx>
    {
        self.flip_y(unsafe { Vector2::<Idx>::from_index_unchecked(index, self.size()) })
    }
    unsafe fn external_position_to_position_unchecked(mut pos : Vector2<Idx>, size : Vector2<Idx>) -> Vector2<Idx> {
        pos.y = size.y - pos.y - Idx::ONE;
        pos
    }
}


impl<T, Idx> IRectangle<Idx, 2> for ImageBase<T, Idx> where Idx : Integer
{
    #[inline(always)]
    fn size(&self) -> Vector<Idx, 2> { self.size }
    #[inline(always)]
    fn pos(&self) -> Vector2<Idx> { zero() }

    fn iter_x(&self) -> Range<Idx> where Vector2<Idx> : HaveX<Idx>, Range<Idx> : IntoIterator { Idx::ZERO..self.size_x() }
    fn iter_y(&self) -> Range<Idx> where Vector2<Idx> : HaveY<Idx>, Range<Idx> : IntoIterator { Idx::ZERO..self.size_y() }
    fn iter_z(&self) -> Range<Idx> where Vector2<Idx> : HaveZ<Idx>, Range<Idx> : IntoIterator { Idx::ZERO..self.size_z() }
    fn iter_w(&self) -> Range<Idx> where Vector2<Idx> : HaveW<Idx>, Range<Idx> : IntoIterator { Idx::ZERO..self.size_w() }

    #[inline(always)] fn is_inside_x(&self, x : Idx) -> bool where Vector2<Idx> : HaveX<Idx> { x >= Idx::ZERO && x < self.size_x() }
    #[inline(always)] fn is_inside_y(&self, y : Idx) -> bool where Vector2<Idx> : HaveY<Idx> { y >= Idx::ZERO && y < self.size_y() }
    #[inline(always)] fn is_inside_z(&self, z : Idx) -> bool where Vector2<Idx> : HaveZ<Idx> { z >= Idx::ZERO && z < self.size_z() }
    #[inline(always)] fn is_inside_w(&self, w : Idx) -> bool where Vector2<Idx> : HaveW<Idx> { w >= Idx::ZERO && w < self.size_w() }
}


impl<T, Idx> Get<Vector2<Idx>> for ImageBase<T, Idx>  where Idx : Integer
{
    type Output = <Self as Index<Vector2<Idx>>>::Output;
    #[inline(always)]
    fn try_get(&self, pos : Vector2<Idx>) -> Result<&Self::Output, ()> { self.get(pos).ok_or_void() }
    #[inline(always)]
    fn get(&self, pos : Vector2<Idx>) -> Option<&Self::Output> { self.position_to_index(pos).and_then(|idx| self.get(idx)) }
    #[inline(always)]
    unsafe fn get_unchecked(&self, pos : Vector2<Idx>) -> &Self::Output { unsafe { let idx = self.position_to_index_unchecked(pos); self.get_unchecked(idx) } }
}

impl<T, Idx> GetMut<Vector2<Idx>> for ImageBase<T, Idx> where Idx : Integer
{
    #[inline(always)]
    fn try_get_mut(&mut self, pos : Vector2<Idx>) -> Result<&mut Self::Output, ()> { self.get_mut(pos).ok_or_void() }
    #[inline(always)]
    fn get_mut(&mut self, pos : Vector2<Idx>) -> Option<&mut Self::Output> { self.position_to_index(pos).and_then(|i| self.get_mut(i)) }
    #[inline(always)]
    unsafe fn get_unchecked_mut(&mut self, pos : Vector2<Idx>) -> &mut Self::Output{ unsafe { let idx = self.position_to_index_unchecked(pos); self.values_mut().get_unchecked_mut(idx)} }
}

impl<T, Idx> GetManyMut<Vector2<Idx>> for ImageBase<T, Idx> where Idx : Integer
{
    #[inline(always)]
    fn try_get_many_mut<const N2: usize>(&mut self, indices: [Vector2<Idx>; N2]) -> Result<[&mut Self::Output;N2], ()> {
        // Use try_map https://doc.rust-lang.org/std/primitive.array.html#method.try_map when #stabilized
        let indices = indices.map(|pos| self.position_to_index(pos));
        if indices.any(|x| x.is_none())
        {
            Err(())
        } else
        {
            self.try_get_many_mut(indices.map(|idx| idx.unwrap()))
        }
    }
}

impl<T, Idx> Get<usize> for ImageBase<T, Idx>
    where Idx : Integer
{
    type Output = <Self as Index<usize>>::Output;
    #[inline(always)]
    fn try_get(&self, index : usize) -> Result<&T, ()> { self.get(index).ok_or_void() }
    #[inline(always)]
    fn get(&self, index : usize) -> Option<&T> { self.values().get(index) }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked(&self, index : usize) -> &T { unsafe { self.values().get_unchecked(index) } }
}

impl<T, Idx> GetMut<usize> for ImageBase<T, Idx> where Idx : Integer
{
    #[inline(always)]
    fn try_get_mut(&mut self, index : usize) -> Result<&mut T, ()> { self.get_mut(index).ok_or_void() }
    #[inline(always)]
    fn get_mut(&mut self, index : usize) -> Option<&mut T> { self.values_mut().get_mut(index) }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked_mut(&mut self, index : usize) -> &mut T{ unsafe { self.values_mut().get_unchecked_mut(index)} }
}

impl<T, Idx> GetManyMut<usize> for ImageBase<T, Idx> where Idx : Integer
{
    #[inline(always)]
    fn try_get_many_mut<const N2: usize>(&mut self, indices: [usize; N2]) -> Result<[&mut Self::Output;N2], ()> {
        self.values_mut().try_get_many_mut(indices)
    }
}

impl<T, Idx> Index<usize> for ImageBase<T, Idx> where Idx : Integer
{
    type Output=T;
    #[inline(always)]
    #[track_caller]
    fn index(&self, index: usize) -> &Self::Output { self.get_or_panic(index) }
}
impl<T, Idx> IndexMut<usize> for ImageBase<T, Idx> where Idx : Integer
{
    #[inline(always)]
    #[track_caller]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { self.get_mut_or_panic(index) }
}

impl<T, Idx> Index<Vector2<Idx>> for ImageBase<T, Idx> where Idx : Integer
{
    type Output=T;
    #[inline(always)]
    fn index(&self, index: Vector2<Idx>) -> &Self::Output { self.get_or_panic(index) }
}
impl<T, Idx> IndexMut<Vector2<Idx>> for ImageBase<T, Idx> where Idx : Integer
{
    #[inline(always)]
    fn index_mut(&mut self, index: Vector2<Idx>) -> &mut Self::Output { self.get_mut_or_panic(index) }
}

impl<T, Idx> Length for ImageBase<T, Idx>
    where Idx : Integer
{
    #[inline(always)]
    fn len(&self) -> usize { self.values().len() }
}



impl<T, Idx> Crop<Idx,2> for ImageBase<T, Idx>
    where Idx : Integer,
    T : Clone
{
    fn crop(self, subrect : Rectangle2<Idx>) -> Option<Self> {
        self.view().crop(subrect).map(|v| v.to_grid())
    }
}


//type TestG = Grid2<i32>;


/*

// use hexga_math::prelude::*;

pub type EncodeResult<T=()> = Result<T,String>;

pub type Image<T=ColorRGBAByte> = ImageBase<T,int>;

pub type ImageRGBAByte = Image<ColorRGBAByte>;
pub type ImageRGBAByteView<'a> = ImageBaseView<'a,ColorRGBAByte>;

// Todo : wrap the grid and reverse the indexing convention
// Pixels should be arranged in memory in rows, with the top row first (like in miniquad/SDL...)

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ImageBase<T=ColorRGBAByte,Idx=int> where T : IColor, Idx : Integer
{
    pub grid : ImageBase<T,Idx,2>,
}

pub type ImageBaseView<'a, T=ColorRGBAByte,Idx=int> = GridView<'a,T,Idx,2>;


impl<T, Idx> Deref for ImageBase<T, Idx> where T : IColor, Idx : Integer {
    type Target=ImageBase<T,Idx,2>;
    fn deref(&self) -> &Self::Target { &self.grid }
}
impl<T, Idx> DerefMut for ImageBase<T, Idx> where T : IColor, Idx : Integer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.grid }
}

impl<T, Idx> ImageBase<T, Idx> where T : IColor, Idx : Integer
{
    pub(crate) fn raw_bytes_rgba(&self) -> Vec<u8>
    {
        let mut v = Vec::with_capacity(self.area_usize() * 4);

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
        let mut v = Vec::with_capacity(self.area_usize() * 3);

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
        let mut v = Vec::with_capacity(self.area_usize() * 4);

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
        let mut v = Vec::with_capacity(self.area_usize() * 3);

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

/*
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
    /// Use the same anti-aliasing as the previous rendering pass
    Same,

    Linear,
    /// Ideal for Pixel Art
    Nearest,
}

impl AntiAliasing
{
    pub const fn is_same(self) -> bool { matches!(self, AntiAliasing::Same) }
    pub const fn is_linear(self) -> bool { matches!(self, AntiAliasing::Linear) }
    pub const fn is_nearest(self) -> bool { matches!(self, AntiAliasing::Nearest) }

    pub fn is_same_as(self, other : Self) -> bool { (self.is_same() || other.is_same()) || (self == other) }
}
    */
    */

    /*
impl<C,Idx> ImageBase<C,Idx> where Idx : Integer, C : IColor
{
    pub fn tmp_write_to_png_bytes_inside(&self, path : &str)
    {
        let file = std::fs::File::create(path).expect("Failed to create file");
        let buffered_write = &mut std::io::BufWriter::new(file);

        /*
        image::ImageEncoder::write_image(
            image::codecs::png::PngEncoder::new(buffered_write),
            &*self.raw_bytes_rgba(),
            self.width().to_usize() as _,
            self.height().to_usize() as _,
            image::ExtendedColorType::Rgba8,
        ).expect("Failed to write PNG image");
    */
    }
}

*/


impl<C,Idx> ToColor for ImageBase<C, Idx> where Idx : Integer, C : ToColor
{
    type ColorRGBAF32 = ImageBase<C::ColorRGBAF32, Idx>;
    fn to_color_rgba_f32(&self) -> Self::ColorRGBAF32 {
        Self::ColorRGBAF32 { pixels: self.pixels.to_color_rgba_f32(), size: self.size() }
    }

    type ColorRGBAF64 = ImageBase<C::ColorRGBAF64, Idx>;
    fn to_color_rgba_f64(&self) -> Self::ColorRGBAF64 {
        Self::ColorRGBAF64 { pixels: self.pixels.to_color_rgba_f64(), size: self.size() }
    }

    type ColorRGBAByte = ImageBase<C::ColorRGBAByte, Idx>;
    fn to_color_rgba_byte(&self) -> Self::ColorRGBAByte {
        Self::ColorRGBAByte { pixels: self.pixels.to_color_rgba_byte(), size: self.size() }
    }

    type ColorRGBABool = ImageBase<C::ColorRGBABool, Idx>;
    fn to_color_rgba_bool(&self) -> Self::ColorRGBABool {
        Self::ColorRGBABool { pixels: self.pixels.to_color_rgba_bool(), size: self.size() }
    }

    type ColorHSLAF32 = ImageBase<C::ColorHSLAF32, Idx>;
    fn to_color_hsla_f32(&self) -> Self::ColorHSLAF32 {
        Self::ColorHSLAF32 { pixels: self.pixels.to_color_hsla_f32(), size: self.size() }
    }

    type ColorHSLAF64 = ImageBase<C::ColorHSLAF64, Idx>;
    fn to_color_hsla_f64(&self) -> Self::ColorHSLAF64 {
        Self::ColorHSLAF64 { pixels: self.pixels.to_color_hsla_f64(), size: self.size() }
    }

    const COLOR_INSIDE : ColorKind = C::COLOR_INSIDE;
}

impl<C,Idx> ImageBase<C,Idx> where Idx : Integer, C : ToColor
{
    pub fn tmp_write_to_png_bytes_inside(&self, path : &str)
    {
        let file = std::fs::File::create(path).expect("Failed to create file");
        let buffered_write = &mut std::io::BufWriter::new(file);

        match C::COLOR_INSIDE
        {
            ColorKind::RGBAByte =>
            {
                ::image::ImageEncoder::write_image(
                    ::image::codecs::png::PngEncoder::new(buffered_write),
                    unsafe {
                        std::slice::from_raw_parts(
                            self.pixels().as_ptr() as *const u8,
                            self.pixels().len() * std::mem::size_of::<C>(),
                        )
                    },
                    self.width().to_usize() as _,
                    self.height().to_usize() as _,
                    ::image::ExtendedColorType::Rgba8,
                ).expect("Failed to write PNG Rgba8 image");
            },
            ColorKind::RGBAU16 =>
            {
                ::image::ImageEncoder::write_image(
                    ::image::codecs::png::PngEncoder::new(buffered_write),
                    unsafe {
                        std::slice::from_raw_parts(
                            self.pixels().as_ptr() as *const u8,
                            self.pixels().len() * std::mem::size_of::<C>(),
                        )
                    },
                    self.width().to_usize() as _,
                    self.height().to_usize() as _,
                    ::image::ExtendedColorType::Rgba16,
                ).expect("Failed to write PNG Rgba16 mage");
            },
            _ => todo!(),
        }
    }
}