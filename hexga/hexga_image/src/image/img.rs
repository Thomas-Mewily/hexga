use hexga_encoding::MediaType;
use ::image::{DynamicImage, GenericImageView, ImageFormat};

use super::*;


pub(crate) mod prelude
{
    pub use super::
    {
        Image,ImageOf,ToImage,
    };
}

pub type Image = ImageOf;
pub type ImageOf<C=ColorU8> = ImageBaseOf<C>;

pub type ImageError<Idx> = GridBaseError<Idx,2>;

/// Image have a different type than Grid because:
///
/// - An image can be saved with more extension than a grid (png, gif...)
/// - The layout of the pixels match the saving format, saving an image don't create a temporary vector
//#[cfg_attr(feature = "serde", derive(Serialize), serde(rename = "Image"))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageBaseOf<C=ColorU8,Idx=int> where Idx: Integer
{
    size  : Vector2<Idx>,
    pixels: Vec<C>,
}



macro_rules! impl_img_fmt_method {
    ($trait_name :ident) => {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
        {
            writeln!(f)?;

            const SEP :&'static str = " ";
            let size = self.size();

            let strings = self.iter()
                .map(|(_, v)| {
                    let mut s = String::new();
                    let mut tmp_f = std::fmt::Formatter::new(&mut s, ___());
                    std::fmt::$trait_name::fmt(v, &mut tmp_f)?;
                    Ok(s)
                })
                .collect::<Result<Vec<_>, _>>()?;

            let width = strings.iter().map(|s| s.len()).max().unwrap_or(0);
            let g = unsafe { ImageBaseOf::from_vec_unchecked(size, strings) };

            for y in (0..size[1].to_usize()).rev()
            {
                for x in 0..size[0].to_usize()
                {
                    let mut idx = Vector::<Idx,2>::ZERO;
                    idx[0] = Idx::cast_from(x);
                    idx[1] = Idx::cast_from(y);
                    write!(f, "{:>width$}", g[idx], width = width)?;
                    f.write_str(SEP)?;
                }
                writeln!(f)?;
            }
            writeln!(f, "size: {:?}", size)
        }
    };
}

map_on_std_fmt!(
    ($trait_name :ident) =>
    {
        impl<C, Idx> std::fmt::$trait_name for ImageBaseOf<C, Idx> where Idx: Integer, C: std::fmt::$trait_name
        {
            impl_img_fmt_method!($trait_name);
        }
    }
);


impl<C,Idx> ImageBaseOf<C,Idx> where Idx: Integer
{
    pub fn pixels(&self) -> &[C] { self.values() }
    pub fn pixels_mut(&mut self) -> &mut[C] { self.values_mut() }

    /// `self.view().crop_intersect(subrect).to_grid()`
    fn subimage(&self, rect: Rectangle2<Idx>) -> Self where C: Clone { self.subview_intersect(rect).to_image() }
}

impl<C,Idx> ImageBaseOf<C,Idx> where Idx: Integer
{
    pub(crate) fn flip_y<P>(&self, mut pos: P) -> P where P: Into<Vector2::<Idx>> + From<Vector2::<Idx>>
    { unsafe { Self::external_position_to_position_unchecked(pos.into(), self.size).into() } }
}





pub trait ToImage
{
    type Output;
    fn to_image(self) -> Self::Output;
}
impl<C,Idx> From<GridOf<C,Idx,2>> for ImageBaseOf<C, Idx> where Idx: Integer
{
    fn from(value: GridOf<C,Idx,2>) -> Self { value.to_image() }
}
impl<'a,C,Idx> From<GridView<'a,GridOf<C,Idx,2>,C,Idx,2>> for ImageBaseOf<C, Idx> where Idx: Integer, C: Clone
{
    fn from(value: GridView<'a,GridOf<C,Idx,2>,C,Idx,2>) -> Self { value.to_image() }
}

impl<C,Idx> ToImage for ImageBaseOf<C,Idx> where Idx: Integer
{
    type Output=Self;
    fn to_image(self) -> Self::Output {
        self
    }
}


impl<C,Idx> ToImage for GridOf<C,Idx,2> where Idx: Integer
{
    type Output = ImageBaseOf<C, Idx>;
    fn to_image(mut self) -> Self::Output
    {
        let (w,h) = self.size().into();
        for x in Idx::iter(w)
        {
            for y in Idx::iter(h / Idx::two())
            {
                unsafe { self.swap_unchecked(vector2(x, y), vector2(x, h - y - Idx::ONE)) };
            }
        }
        let (size, values) = self.into_size_and_values();
        unsafe { ImageBaseOf::from_vec_unchecked(size, values) }
    }
}
impl<C,Idx> ToGrid<C,Idx,2> for ImageBaseOf<C, Idx> where Idx: Integer
{
    type Output = GridOf<C, Idx,2>;
    fn to_grid(mut self) -> Self::Output {
        let (w,h) = self.size().into();
        for x in Idx::iter(w)
        {
            for y in Idx::iter(h / Idx::two())
            {
                unsafe { self.swap_unchecked(vector2(x, y), vector2(x, h - y - Idx::ONE)) };
            }
        }
        let (size, values) = self.into_size_and_values();
        unsafe { GridOf::from_vec_unchecked(size, values) }
    }
}


impl<'a,C,Idx> ToImage for GridView<'a,GridOf<C,Idx,2>,C,Idx,2> where Idx: Integer, C: Clone
{
    type Output = ImageBaseOf<C, Idx>;
    fn to_image(self) -> Self::Output
    {
        let size = self.size();
        Self::Output::from_fn(size, |p|
            unsafe { self.get_unchecked(vector2(p.x, size.y - p.y - Idx::ONE)) }.clone()
        )
    }
}
impl<'a,C,Idx> ToImage for GridViewMut<'a,GridOf<C,Idx,2>,C,Idx,2> where Idx: Integer, C: Clone
{
    type Output = ImageBaseOf<C, Idx>;
    fn to_image(self) -> Self::Output { self.view().to_image() }
}


impl<'a,C,Idx> ToImage for GridView<'a,ImageBaseOf<C,Idx>,C,Idx,2> where Idx: Integer, C: Clone
{
    type Output = ImageBaseOf<C, Idx>;
    fn to_image(self) -> Self::Output
    {
        Self::Output::from_fn(self.size(), |p| unsafe { self.get_unchecked(p) }.clone() )
    }
}
impl<'a,C,Idx> ToImage for GridViewMut<'a,ImageBaseOf<C,Idx>,C,Idx,2> where Idx: Integer, C: Clone
{
    type Output = ImageBaseOf<C, Idx>;
    fn to_image(self) -> Self::Output { self.view().to_image() }
}


impl<C,Idx> IGrid<C,Idx,2> for ImageBaseOf<C,Idx> where Idx: Integer
{
    type WithType<U> = ImageBaseOf<U,Idx>;

    fn values(&self) -> &[C] { &self.pixels }
    fn values_mut(&mut self) -> &mut [C] { &mut self.pixels }

    fn into_size_and_values(self) -> (Vector2<Idx>, Vec<C>) { (self.size, self.pixels) }

    unsafe fn from_vec_unchecked<P>(size: P, pixels: Vec<C>) -> Self where P: Into<Vector2::<Idx>>
    {
        Self { size: size.into(), pixels }
    }

    unsafe fn position_to_index_unchecked<P>(&self, pos: P) -> usize where P: Into<Vector2<Idx>> { unsafe { Vector2::<Idx>::to_index_unchecked(self.flip_y(pos.into()), self.size()) } }
    unsafe fn index_to_position_unchecked(&self, index: usize) -> Vector2<Idx>
    {
        self.flip_y(unsafe { Vector2::<Idx>::from_index_unchecked(index, self.size()) })
    }
    unsafe fn external_position_to_position_unchecked<P>(mut pos: P, size: P) -> P where P: Into<Vector2<Idx>> + From<Vector2<Idx>>
    {
        let mut pos = pos.into();
        let size = size.into();
        pos.y = size.y - pos.y - Idx::ONE;
        pos.into()
    }
}


impl<T, Idx> GetPosition<Idx, 2> for ImageBaseOf<T, Idx> where Idx: Integer
{
    #[inline(always)]
    fn pos(&self) -> Vector2<Idx> { zero() }
}
impl<T, Idx> GetSize<Idx, 2> for ImageBaseOf<T, Idx> where Idx: Integer
{
    #[inline(always)]
    fn size(&self) -> Vector<Idx, 2> { self.size }
}
/*
impl<T, Idx> SetSize<Idx, 2> for ImageBaseOf<T, Idx> where Idx: Integer
{
    fn set_size(&mut self, size: Vector<Idx, 2>) -> &mut Self {

    }
}
*/


impl<P, T, Idx> TryGet<P> for ImageBaseOf<T, Idx>  where Idx: Integer, P: Into<Vector2<Idx>>
{
    type Error = IndexOutOfRange<Vector2<Idx>,Vector2<Idx>>;
    #[inline(always)]
    fn try_get(&self, pos: P) -> Result<&Self::Output, Self::Error>
    {
        let p = pos.into();
        self.get(p).ok_or_else(|| IndexOutOfRange::new(p, self.size()))
    }
}
impl<P, T, Idx> Get<P> for ImageBaseOf<T, Idx>  where Idx: Integer, P: Into<Vector2<Idx>>
{
    type Output = T;
    #[inline(always)]
    fn get(&self, pos: P) -> Option<&Self::Output> { self.position_to_index(pos).and_then(|idx| Some(unsafe { self.pixels().get_unchecked(idx) })) }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked(&self, pos: P) -> &Self::Output { unsafe { let idx = self.position_to_index_unchecked(pos.into()); self.pixels().get_unchecked(idx) } }
}


impl<P, T, Idx> TryGetMut<P> for ImageBaseOf<T, Idx> where Idx: Integer, P: Into<Vector2<Idx>>
{
    #[inline(always)]
    fn try_get_mut(&mut self, pos: P) -> Result<&mut Self::Output, Self::Error>
    {
        let p = pos.into();
        let size = self.size();
        self.get_mut(p).ok_or_else(|| IndexOutOfRange::new(p, size))
    }
}
impl<P, T, Idx> GetMut<P> for ImageBaseOf<T, Idx> where Idx: Integer, P: Into<Vector2<Idx>>
{
    #[inline(always)]
    fn get_mut(&mut self, pos: P) -> Option<&mut Self::Output> { self.position_to_index(pos).and_then(|i| Some(unsafe { self.pixels_mut().get_unchecked_mut(i) })) }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked_mut(&mut self, pos: P) -> &mut Self::Output{ unsafe { let idx = self.position_to_index_unchecked(pos); self.values_mut().get_unchecked_mut(idx)} }
}

impl<P, T, Idx> GetManyMut<P> for ImageBaseOf<T, Idx> where Idx: Integer, P: Into<Vector2<Idx>>
{
    #[inline(always)]
    fn try_get_many_mut<const N2: usize>(&mut self, indices: [P; N2]) -> Result<[&mut Self::Output;N2], ManyMutError> {
        // Use try_map https://doc.rust-lang.org/std/primitive.array.html#method.try_map when #stabilized
        let indices = indices.map(|pos| self.position_to_index(pos));
        if indices.any(|x| x.is_none())
        {
            Err(ManyMutError::IndexOutOfBounds)
        } else
        {
            self.pixels_mut().try_get_many_mut(indices.map(|idx| idx.unwrap()))
        }
    }

    fn get_many_mut<const N: usize>(&mut self, indices: [P; N]) -> Option<[&mut Self::Output;N]> {
        // Use try_map https://doc.rust-lang.org/std/primitive.array.html#method.try_map when #stabilized
        let indices = indices.map(|pos| self.position_to_index(pos));
        if indices.any(|x| x.is_none())
        {
            None
        } else
        {
            self.pixels_mut().get_many_mut(indices.map(|idx| idx.unwrap()))
        }
    }
}

impl<P, T, Idx> Index<P> for ImageBaseOf<T, Idx> where Idx: Integer, P: Into<Vector2<Idx>>
{
    type Output=T;
    #[inline(always)]
    fn index(&self, index: P) -> &Self::Output { self.get_or_panic(index) }
}
impl<P, T, Idx> IndexMut<P> for ImageBaseOf<T, Idx> where Idx: Integer, P: Into<Vector2<Idx>>
{
    #[inline(always)]
    fn index_mut(&mut self, index: P) -> &mut Self::Output { self.get_mut_or_panic(index) }
}



impl<T, Idx> Length for ImageBaseOf<T, Idx>
    where Idx: Integer
{
    #[inline(always)]
    fn len(&self) -> usize { self.values().len() }
}



impl<T, Idx> Crop<Idx,2> for ImageBaseOf<T, Idx>
    where Idx: Integer, T:Clone
{
    fn crop(self, subrect: Rectangle2<Idx>) -> Option<Self> {
        self.view().crop(subrect).map(|v| v.to_image())
    }
}


/*
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
pub struct GraphicsParam
{
    aa: AntiAliasing,
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

    pub fn is_same_as(self, other: Self) -> bool { (self.is_same() || other.is_same()) || (self == other) }
}
*/

/*
impl<C,Idx> ToColorComposite for ImageBase<C, Idx> where Idx: Integer, C: ToColorComposite
{
    type RgbaF32 = ImageBase<C::RgbaF32, Idx>;
    fn to_rgba_f32(&self) -> Self::RgbaF32 {
        Self::RgbaF32 { pixels: self.pixels.to_rgba_f32(), size: self.size() }
    }

    type RgbaF64 = ImageBase<C::RgbaF64, Idx>;
    fn to_rgba_f64(&self) -> Self::RgbaF64 {
        Self::RgbaF64 { pixels: self.pixels.to_rgba_f64(), size: self.size() }
    }

    type RgbaU8 = ImageBase<C::RgbaU8, Idx>;
    fn to_rgba_u8(&self) -> Self::RgbaU8 {
        Self::RgbaU8 { pixels: self.pixels.to_rgba_u8(), size: self.size() }
    }

    type RgbaU16 = ImageBase<C::RgbaU16, Idx>;
    fn to_rgba_u16(&self) -> Self::RgbaU16 {
        Self::RgbaU16 { pixels: self.pixels.to_rgba_u16(), size: self.size() }
    }

    type RgbaBool = ImageBase<C::RgbaBool, Idx>;
    fn to_rgba_bool(&self) -> Self::RgbaBool {
        Self::RgbaBool { pixels: self.pixels.to_rgba_bool(), size: self.size() }
    }

    type HslaF32 = ImageBase<C::HslaF32, Idx>;
    fn to_hsla_f32(&self) -> Self::HslaF32 {
        Self::HslaF32 { pixels: self.pixels.to_hsla_f32(), size: self.size() }
    }

    type HslaF64 = ImageBase<C::HslaF64, Idx>;
    fn to_hsla_f64(&self) -> Self::HslaF64 {
        Self::HslaF64 { pixels: self.pixels.to_hsla_f64(), size: self.size() }
    }

    const COLOR_INSIDE: ColorKind = C::COLOR_INSIDE;
}
*/

impl<T, Idx> MapIntern for ImageBaseOf<T, Idx> where Idx: Integer
{
    type Item=T;
    fn map_intern<F>(mut self, f: F) -> Self where F: FnMut(Self::Item) -> Self::Item {
        self.pixels = self.pixels.map_intern(f);
        self
    }
}
impl<T, Idx> MapInternWith for ImageBaseOf<T, Idx> where Idx: Integer
{
    fn map_with_intern<F>(mut self, other: Self, f: F) -> Self where F: FnMut(Self::Item, Self::Item) -> Self::Item {
        assert_eq!(self.size(), other.size(), "size mismatch");
        self.pixels = self.pixels.map_with_intern(other.pixels, f);
        self
    }
}
impl<T, Idx> Map for ImageBaseOf<T, Idx> where Idx: Integer
{
    type WithType<R> = ImageBaseOf<R, Idx>;

    fn map<R,F>(self, f: F) -> Self::WithType<R> where F: FnMut(Self::Item) -> R {
        unsafe { Self::WithType::<R>::from_vec_unchecked(self.size, self.pixels.map(f)) }
    }
}
impl<T, Idx> MapWith for ImageBaseOf<T, Idx> where Idx: Integer
{
    fn map_with<R, Item2, F>(self, other: Self::WithType<Item2>, f: F) -> Self::WithType<R> where F: FnMut(Self::Item, Item2) -> R {
        assert_eq!(self.size(), other.size(), "size mismatch");
        unsafe { ImageBaseOf::from_vec_unchecked(self.size(), self.pixels.map_with(other.pixels, f)) }
    }
}




impl <C,Idx> SaveExtension for ImageBaseOf<C,Idx>
    where
    Idx : Integer + CfgSerialize,
    C : Clone + IColor<ToRgba<u8>=RgbaOf<u8>> + IColor<ToRgba<u16>=RgbaOf<u16>> + CfgSerialize,
    u8: CastRangeFrom<C::Component>,
    u16: CastRangeFrom<C::Component>
{
    fn save_custom_extensions() -> impl Iterator<Item = &'static extension> {
        [ "png" ].into_iter()
    }

    fn save_to_writer_with_custom_extension<W>(&self, writer: W, extension: &extension) -> EncodeResult where W: Write {
        match extension
        {
            "png" => {
                match C::Component::PRIMITIVE_TYPE
                {
                    NumberType::IntegerSigned =>
                    {
                        if std::mem::size_of::<C::Component>() * 8 <= 8
                        {
                            self.clone().to_rgba_u8().save_to_writer_with_custom_extension(writer, extension)
                        }else
                        {
                            self.clone().to_rgba_u16().save_to_writer_with_custom_extension(writer, extension)
                        }
                    },
                    NumberType::IntegerUnsigned => match std::mem::size_of::<C::Component>() * 8
                    {
                        8 =>
                        {
                            ::image::ImageEncoder::write_image(
                            ::image::codecs::png::PngEncoder::new(writer),
                            unsafe {
                                std::slice::from_raw_parts(
                                    self.pixels().as_ptr() as *const u8,
                                    self.pixels().len() * std::mem::size_of::<C>(),
                                )
                            },
                            self.width().to_usize() as _,
                            self.height().to_usize() as _,
                            ::image::ExtendedColorType::Rgba8,
                            ).map_err(|e| EncodeError::custom(format!("Failed to encode .png rgba8 image : {}", e)))
                        }
                        16 =>
                        {
                            ::image::ImageEncoder::write_image(
                            ::image::codecs::png::PngEncoder::new(writer),
                            unsafe {
                                std::slice::from_raw_parts(
                                    self.pixels().as_ptr() as *const u8,
                                    self.pixels().len() * std::mem::size_of::<C>(),
                                )
                            },
                            self.width().to_usize() as _,
                            self.height().to_usize() as _,
                            ::image::ExtendedColorType::Rgba16,
                            ).map_err(|e| EncodeError::custom(format!("Failed to encode .png rgba16 image : {}", e)))
                        }
                        _ =>
                        {
                            self.clone().to_rgba_u8().save_to_writer_with_custom_extension(writer, extension)
                        }
                    }
                    NumberType::Float => self.clone().to_rgba_u16().save_to_writer_with_custom_extension(writer, extension),
                    NumberType::Bool => self.clone().to_rgba_u8().save_to_writer_with_custom_extension(writer, extension),
                }
            }
            _ => Err(EncodeError::save_unsupported_extension_with_name::<Self>(extension.to_owned(), "Image")),
        }
    }

}

impl <C,Idx> MediaType for ImageBaseOf<C,Idx>
    where
    Idx : Integer,
    C : Clone + IColor<ToRgba<u8>=RgbaOf<u8>> + IColor<ToRgba<u16>=RgbaOf<u16>>,
    u8: CastRangeFrom<C::Component>,
    u16: CastRangeFrom<C::Component>
{
    fn media_type() -> &'static str { "image" }
}

impl <C,Idx> LoadExtension for ImageBaseOf<C,Idx>
    where
    Idx : Integer,
    C : Clone + IColor<ToRgba<u8>=RgbaOf<u8>> + IColor<ToRgba<u16>=RgbaOf<u16>>,
    u8: CastRangeFrom<C::Component>,
    u16: CastRangeFrom<C::Component>
{
    fn load_custom_extensions() -> impl Iterator<Item = &'static extension> {
        [
            "png",
            "jpg", "jpeg",
            "bmp",
            "gif",
            "webp",
            "ico",
            "tiff"
        ].into_iter()
    }

    fn load_from_reader_with_custom_extension<R>(mut reader: R, extension: &extension) -> EncodeResult<Self> where Self: Sized, R: std::io::Read
    {
        // let format = match extension
        // {
        //     "png" => ImageFormat::Png,
        //     "jpg" | "jpeg" => ImageFormat::Jpeg,
        //     "bmp" => ImageFormat::Bmp,
        //     "gif" => ImageFormat::Gif,
        //     "webp" => ImageFormat::WebP,
        //     "ico" => ImageFormat::Ico,
        //     "tiff" => ImageFormat::Tiff,
        //     other =>  Err(EncodeError::decode_unsupported_extension_with_name::<Self>(extension, "Image"))?,
        // };

        let mut bytes = Vec::with_capacity(262144); // 0.25 Mo
        reader.read_to_end(&mut bytes)?;

        let img = ::image::load_from_memory(&bytes);
        // let img = ::image::load_from_memory_with_format(&bytes, format);
        let img = match img
        {
            Ok(dyn_img) => dyn_img,
            Err(e) => Err(EncodeError::load_unsupported_extension_with_name::<Self>(extension.to_owned(), "Image"))?,
        };

        let (width, height) : (u32, u32) = img.dimensions();
        let w = Idx::cast_from(width);
        let h = Idx::cast_from(height);
        let casted_width = w.to_u32();
        let casted_height = h.to_u32();
        if casted_width != width || height != casted_height
        {
            return Err(EncodeError::custom(format!("Image is too big: {}", vector2(width, height))));
        }

        let error_invalid_size = || EncodeError::custom("Invalid bytes len");

        match C::Component::PRIMITIVE_TYPE
        {
            NumberType::IntegerSigned => {}, // Todo: handle >= 16 bits signed to use u16 precision ?
            NumberType::IntegerUnsigned =>
            {
                if std::mem::size_of::<C::Component>() * 8 >= 16
                {
                    let bytes = match img
                    {
                        DynamicImage::ImageRgba16(rgba) => rgba,
                        x => x.to_rgba16(),
                    }.into_raw();
                    let multiple = 4 * std::mem::size_of::<u16>(); // 4 components (rbga)
                    if bytes.len() % multiple != 0 || bytes.len() / 4 != vector2(w, h).area_usize()
                    {
                        return Err(error_invalid_size());
                    }

                    let rgba_vec: Vec<RgbaU16> = bytes
                        .chunks_exact(4)
                        .map(|chunk| RgbaU16 {
                            r: chunk[0],
                            g: chunk[1],
                            b: chunk[2],
                            a: chunk[3],
                        })
                        .collect();

                    let pixels = rgba_vec.into_iter().map(|v| C::from_rgba_u16(v)).collect();
                    let size = vector2(w, h);

                    return Self::from_vec(size, pixels).ok_or_else(error_invalid_size);
                }
            },
            NumberType::Float =>
            {
                let bytes = match img
                    {
                        DynamicImage::ImageRgba32F(rgba) => rgba,
                        x => x.to_rgba32f(),
                    }.into_raw();

                    let multiple = 4 * std::mem::size_of::<float>(); // 4 components (rbga)
                    if bytes.len() % multiple != 0 || bytes.len() / 4 != vector2(w, h).area_usize()
                    {
                        return Err(error_invalid_size());
                    }

                    let rgba_vec: Vec<RgbaF32> = bytes
                        .chunks_exact(4)
                        .map(|chunk| RgbaF32 {
                            r: chunk[0],
                            g: chunk[1],
                            b: chunk[2],
                            a: chunk[3],
                        })
                        .collect();

                    let pixels = rgba_vec.into_iter().map(|v| C::from_rgba_f32(v)).collect();
                    let size = vector2(w, h);

                    return Self::from_vec(size, pixels).ok_or_else(error_invalid_size);
            },
            NumberType::Bool => {},
        }

        // fallback on u8
        let bytes = match img
        {
            DynamicImage::ImageRgba8(rgba8) => rgba8,
            x => x.to_rgba8(),
        }.into_raw();
        let multiple = 4 * std::mem::size_of::<u8>(); // 4 components (rbga)
        if bytes.len() % multiple != 0 || bytes.len() / 4 != vector2(w, h).area_usize()
        {
            return Err(error_invalid_size());
        }

        let rgba_vec: Vec<RgbaU8> = bytes
            .chunks_exact(4)
            .map(|chunk| RgbaU8 {
                r: chunk[0],
                g: chunk[1],
                b: chunk[2],
                a: chunk[3],
            })
            .collect();

        let pixels = rgba_vec.into_iter().map(|v| C::from_rgba_u8(v)).collect();
        let size = vector2(w, h);

        Self::from_vec(size, pixels).ok_or_else(error_invalid_size)
    }
}

