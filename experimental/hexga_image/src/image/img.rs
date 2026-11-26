use ::image::{
    ColorType, DynamicImage, GenericImageView, ImageEncoder, ImageFormat, Rgba,
    codecs::{bmp::BmpEncoder, gif::GifEncoder, ico::IcoEncoder, jpeg::JpegEncoder, png::PngEncoder, webp::WebPEncoder},
};
use hexga_encoding::{DeducedExtension, MediaType};
use hexga_math::NumberType;

use super::*;

pub(crate) mod prelude {
    pub use super::{Image, ImageBaseOf, ImageOf, ToImage};
}

pub type Image = ImageOf;
pub type ImageOf<C = ColorU8, A = AllocGlobal> = ImageBaseOf<C, int, A>;

pub type ImageError<Idx> = GridBaseError<Idx, 2>;

// Todo: make a trait for C, IColor ?

/// Image have a different type than Grid because:
///
/// - An image can be saved with more extension than a grid (png, gif...)
/// - The layout of the pixels match the saving format, saving an image don't create a temporary vector
//#[cfg_attr(feature = "serde", derive(Serialize), serde(rename = "Image"))]
#[derive(Clone)]
pub struct ImageBaseOf<C = ColorU8, Idx = int, A: Allocator = AllocGlobal>
where
    Idx: Integer,
{
    size: Vector2<Idx>,
    pub(crate) pixels: Vec<C, A>,
}

impl<C, Idx, A: Allocator, B: Allocator> PartialEq<ImageBaseOf<C, Idx, B>> for ImageBaseOf<C, Idx, A>
where
    Idx: Integer,
    C: PartialEq,
{
    fn eq(&self, other: &ImageBaseOf<C, Idx, B>) -> bool {
        self.size == other.size && self.pixels.as_slice() == other.pixels.as_slice()
    }
}

impl<C, Idx, A: Allocator> Eq for ImageBaseOf<C, Idx, A>
where
    Idx: Integer,
    C: Eq,
{
}

impl<C, Idx, A: Allocator> PartialOrd for ImageBaseOf<C, Idx, A>
where
    Idx: Integer,
    C: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        (&self.size, self.pixels.as_slice()).partial_cmp(&(&other.size, other.pixels.as_slice()))
    }
}

impl<C, Idx, A: Allocator> Ord for ImageBaseOf<C, Idx, A>
where
    Idx: Integer,
    C: Ord,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        (&self.size, self.pixels.as_slice()).cmp(&(&other.size, other.pixels.as_slice()))
    }
}

impl<C, Idx, A: Allocator> core::hash::Hash for ImageBaseOf<C, Idx, A>
where
    Idx: Integer,
    C: core::hash::Hash,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.size.hash(state);
        self.pixels.as_slice().hash(state);
    }
}

impl<C, Idx, A: Allocator + Default> Default for ImageBaseOf<C, Idx, A>
where
    Idx: Integer,
{
    fn default() -> Self {
        Self {
            size: ___(),
            pixels: Vec::new_in(A::default()),
        }
    }
}

/*
macro_rules! impl_img_fmt_method {
    ($trait_name :ident) => {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
        {
            writeln!(f)?;

            const SEP: &'static str = " ";
            let size = self.size();

            let strings = self
                .iter()
                .map(|(_, v)| {
                    let mut s = String::new();
                    let mut tmp_f = std::fmt::Formatter::new(&mut s, ___());
                    std::fmt::$trait_name::fmt(v, &mut tmp_f)?;
                    Ok(s)
                })
                .collect::<Result<Vec<_>, _>>()?;

            let width = strings.iter().map(|s| s.len()).max().unwrap_or(0);
            let g = unsafe { ImageBaseOf::<_, Idx>::from_vec_unchecked(size, strings) };

            for y in (0..size[1].to_usize()).rev()
            {
                for x in 0..size[0].to_usize()
                {
                    let mut idx = Vector::<Idx, 2>::ZERO;
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
*/

/*
map_on_std_fmt!(
    ($trait_name :ident) =>
    {
        impl<C, Idx> std::fmt::$trait_name for ImageBaseOf<C, Idx> where Idx: Integer, C: std::fmt::$trait_name
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
            {
                std::write!(f, "Image({}, {:?}x{:?})", std::any::type_name::<C>(), self.size().x, self.size().y)
            }
            impl_img_fmt_method!($trait_name);
        }
    }
);
*/

impl<C, Idx, A: Allocator + Clone> Debug for ImageBaseOf<C, Idx, A>
where
    Idx: Integer,
    C: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Image({}, {:?}x{:?})", std::any::type_name::<C>(), self.size().x, self.size().y)?;
        if self.size().area_usize() <= 16 * 16 {
            writeln!(f)?;

            const SEP: &str = " ";
            let size = self.size();

            use ::core::fmt::{self, Write};

            let strings = self
                .iter()
                .map(|(_, v)| {
                    let mut s = String::new();
                    let wrapper = fmt::from_fn(|f| ::core::fmt::Debug::fmt(v, f));
                    write!(s, "{}", wrapper)?;
                    Ok(s)
                })
                .collect::<Result<Vec<_>, _>>()?;

            let width = strings.iter().map(|s| s.len()).max().unwrap_or(0);
            let g = unsafe { ImageBaseOf::<_, Idx>::from_vec_unchecked(size, strings) };

            for y in (0..size[1].to_usize()).rev() {
                for x in 0..size[0].to_usize() {
                    let mut idx = Vector::<Idx, 2>::ZERO;
                    idx[0] = Idx::cast_from(x);
                    idx[1] = Idx::cast_from(y);
                    write!(f, "{:>width$}", g[idx], width = width)?;
                    f.write_str(SEP)?;
                }
                writeln!(f)?;
            }
            writeln!(f, "size: {:?}", size)?;
        }
        Ok(())
    }
}

impl<C, Idx, A: Allocator + Clone> ImageBaseOf<C, Idx, A>
where
    Idx: Integer,
{
    pub fn pixels(&self) -> &[C] {
        self.values()
    }
    pub fn pixels_mut(&mut self) -> &mut [C] {
        self.values_mut()
    }

    /// `self.view().crop_intersect(subrect).to_grid()`
    fn subimage(&self, rect: Rectangle2<Idx>) -> Self
    where
        C: Clone,
    {
        self.subview_intersect(rect).to_image()
    }
}

impl<C, Idx, A: Allocator + Clone> ImageBaseOf<C, Idx, A>
where
    Idx: Integer,
{
    pub(crate) fn flip_y<P>(&self, mut pos: P) -> P
    where
        P: Into<Vector2<Idx>> + From<Vector2<Idx>>,
    {
        unsafe { Self::external_position_to_position_unchecked(pos.into(), self.size).into() }
    }
}

pub trait ToImage {
    type Output;
    fn to_image(self) -> Self::Output;
}
impl<C, Idx, A: Allocator + Clone> From<GridOf<C, Idx, 2, A>> for ImageBaseOf<C, Idx, A>
where
    Idx: Integer,
{
    fn from(value: GridOf<C, Idx, 2, A>) -> Self {
        value.to_image()
    }
}
impl<'a, C, Idx, A: Allocator + Clone> From<GridView<'a, GridOf<C, Idx, 2, A>, C, Idx, 2>> for ImageBaseOf<C, Idx, A>
where
    Idx: Integer,
    C: Clone,
{
    fn from(value: GridView<'a, GridOf<C, Idx, 2, A>, C, Idx, 2>) -> Self {
        value.to_image()
    }
}

impl<C, Idx, A: Allocator + Clone> ToImage for ImageBaseOf<C, Idx, A>
where
    Idx: Integer,
{
    type Output = Self;
    fn to_image(self) -> Self::Output {
        self
    }
}

fn reverse_rows<T>(pixels: &mut [T], width: usize, height: usize) {
    if width == 0 {
        return;
    }
    let (first, last) = pixels.split_at_mut(width * (height / 2));
    for (top, bottom) in first.chunks_exact_mut(width).zip(last.chunks_exact_mut(width).rev()) {
        top.swap_with_slice(bottom);
    }
}

impl<C, Idx, A: Allocator + Clone> ToImage for GridOf<C, Idx, 2, A>
where
    Idx: Integer,
{
    type Output = ImageBaseOf<C, Idx, A>;
    fn to_image(self) -> Self::Output {
        let (size, mut values) = self.into_size_and_values();
        reverse_rows(&mut values, size.x.to_usize(), size.y.to_usize());
        unsafe { ImageBaseOf::from_vec_unchecked(size, values) }
    }
}
impl<C, Idx, A: Allocator + Clone> ToGrid<C, Idx, 2> for ImageBaseOf<C, Idx, A>
where
    Idx: Integer,
{
    type Output = GridOf<C, Idx, 2, A>;
    fn to_grid(self) -> Self::Output {
        let (size, mut values) = self.into_size_and_values();
        reverse_rows(&mut values, size.x.to_usize(), size.y.to_usize());
        unsafe { GridOf::from_vec_unchecked(size, values) }
    }
}

impl<'a, C, Idx, A: Allocator + Clone> ToImage for GridView<'a, GridOf<C, Idx, 2, A>, C, Idx, 2>
where
    Idx: Integer,
    C: Clone,
{
    type Output = ImageBaseOf<C, Idx, A>;
    fn to_image(self) -> Self::Output {
        let size = self.size();
        let allocator = unsafe { self.grid_unchecked() }.allocator().clone();
        Self::Output::from_fn_in(size, |p| unsafe { self.get_unchecked(p) }.clone(), allocator)
    }
}
impl<'a, C, Idx, A: Allocator + Clone> ToImage for GridViewMut<'a, GridOf<C, Idx, 2, A>, C, Idx, 2>
where
    Idx: Integer,
    C: Clone,
{
    type Output = ImageBaseOf<C, Idx, A>;
    fn to_image(self) -> Self::Output {
        self.view().to_image()
    }
}

impl<'a, C, Idx, A: Allocator + Clone> ToImage for GridView<'a, ImageBaseOf<C, Idx, A>, C, Idx, 2>
where
    Idx: Integer,
    C: Clone,
{
    type Output = ImageBaseOf<C, Idx, A>;
    fn to_image(self) -> Self::Output {
        let allocator = unsafe { self.grid_unchecked() }.allocator().clone();
        Self::Output::from_fn_in(self.size(), |p| unsafe { self.get_unchecked(p) }.clone(), allocator)
    }
}
impl<'a, C, Idx, A: Allocator + Clone> ToImage for GridViewMut<'a, ImageBaseOf<C, Idx, A>, C, Idx, 2>
where
    Idx: Integer,
    C: Clone,
{
    type Output = ImageBaseOf<C, Idx, A>;
    fn to_image(self) -> Self::Output {
        self.view().to_image()
    }
}

impl<C, Idx, A: Allocator + Clone> IGrid<C, Idx, 2> for ImageBaseOf<C, Idx, A>
where
    Idx: Integer,
{
    type WithType<U> = ImageBaseOf<U, Idx, A>;
    type Allocator = A;

    fn allocator(&self) -> &A {
        self.pixels.allocator()
    }

    fn values(&self) -> &[C] {
        &self.pixels
    }
    fn values_mut(&mut self) -> &mut [C] {
        &mut self.pixels
    }

    fn into_size_and_values(self) -> (Vector2<Idx>, Vec<C, A>) {
        (self.size, self.pixels)
    }

    unsafe fn from_vec_unchecked<P>(size: P, pixels: Vec<C, A>) -> Self
    where
        P: Into<Vector2<Idx>>,
    {
        let size = size.into();
        debug_assert_eq!(size.area_usize_checked(), Some(pixels.len()));
        Self { size, pixels }
    }

    unsafe fn position_to_index_unchecked<P>(&self, pos: P) -> usize
    where
        P: Into<Vector2<Idx>>,
    {
        unsafe { Vector2::<Idx>::to_index_unchecked(self.flip_y(pos.into()), self.size()) }
    }
    unsafe fn index_to_position_unchecked(&self, index: usize) -> Vector2<Idx> {
        self.flip_y(unsafe { Vector2::<Idx>::from_index_unchecked(index, self.size()) })
    }
    unsafe fn external_position_to_position_unchecked<P>(mut pos: P, size: P) -> P
    where
        P: Into<Vector2<Idx>> + From<Vector2<Idx>>,
    {
        let mut pos = pos.into();
        let size = size.into();
        pos.y = size.y - pos.y - Idx::ONE;
        pos.into()
    }
}

impl<T, Idx, A: Allocator + Clone> GetPosition<Idx, 2> for ImageBaseOf<T, Idx, A>
where
    Idx: Integer,
{
    #[inline(always)]
    fn pos(&self) -> Vector2<Idx> {
        zero()
    }
}
impl<T, Idx, A: Allocator + Clone> GetSize<Idx, 2> for ImageBaseOf<T, Idx, A>
where
    Idx: Integer,
{
    #[inline(always)]
    fn size(&self) -> Vector<Idx, 2> {
        self.size
    }
}
/*
impl<T, Idx> SetSize<Idx, 2> for ImageBaseOf<T, Idx> where Idx: Integer
{
    fn set_size(&mut self, size: Vector<Idx, 2>) -> &mut Self {

    }
}
*/

impl<T, Idx, A: Allocator + Clone> CollectionBijective for ImageBaseOf<T, Idx, A> where Idx: Integer {}
impl<T, Idx: Integer, A: Allocator + Clone> Collection<T> for ImageBaseOf<T, Idx, A> {
    type Ref<'a>
        = &'a T
    where
        Self: 'a,
        T: 'a;
}
impl<T, Idx: Integer, A: Allocator + Clone> CollectionMut<T> for ImageBaseOf<T, Idx, A> {
    type RefMut<'a>
        = &'a mut T
    where
        Self: 'a,
        T: 'a;
}

impl<P, T, Idx, A: Allocator + Clone> CollectionIndex<P> for ImageBaseOf<T, Idx, A>
where
    Idx: Integer,
    P: Into<Vector2<Idx>>,
{
    type Output = T;
    type Ref<'a>
        = &'a T
    where
        Self: 'a,
        Self::Output: 'a;
}
impl<P, T, Idx, A: Allocator + Clone> Get<P> for ImageBaseOf<T, Idx, A>
where
    Idx: Integer,
    P: Into<Vector2<Idx>>,
{
    #[inline(always)]
    fn get<'a>(&'a self, pos: P) -> Option<Self::Ref<'a>>
    where
        Self::Output: 'a,
    {
        self.position_to_index(pos).map(|idx| unsafe { self.pixels().get_unchecked(idx) })
    }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked<'a>(&'a self, pos: P) -> Self::Ref<'a>
    where
        Self::Output: 'a,
    {
        unsafe {
            let idx = self.position_to_index_unchecked(pos.into());
            self.pixels().get_unchecked(idx)
        }
    }
}
impl<P, T, Idx, A: Allocator + Clone> TryGet<P> for ImageBaseOf<T, Idx, A>
where
    Idx: Integer,
    P: Into<Vector2<Idx>>,
{
    type Error = IndexOutOfBounds<Vector2<Idx>, Vector2<Idx>>;
    #[inline(always)]
    fn try_get<'a>(&'a self, pos: P) -> Result<Self::Ref<'a>, Self::Error>
    where
        Self::Output: 'a,
    {
        let p = pos.into();
        self.get(p).ok_or_else(|| IndexOutOfBounds::new(p, self.size()))
    }
}

impl<P, T, Idx, A: Allocator + Clone> TryGetMut<P> for ImageBaseOf<T, Idx, A>
where
    Idx: Integer,
    P: Into<Vector2<Idx>>,
{
    #[inline(always)]
    fn try_get_mut<'a>(&'a mut self, pos: P) -> Result<Self::RefMut<'a>, Self::Error>
    where
        Self::Output: 'a,
    {
        let p = pos.into();
        let size = self.size();
        self.get_mut(p).ok_or_else(|| IndexOutOfBounds::new(p, size))
    }
}
impl<P, T, Idx, A: Allocator + Clone> CollectionIndexMut<P> for ImageBaseOf<T, Idx, A>
where
    Idx: Integer,
    P: Into<Vector2<Idx>>,
{
    type RefMut<'a>
        = &'a mut T
    where
        Self: 'a,
        Self::Output: 'a;
}
impl<P, T, Idx, A: Allocator + Clone> GetMut<P> for ImageBaseOf<T, Idx, A>
where
    Idx: Integer,
    P: Into<Vector2<Idx>>,
{
    #[inline(always)]
    fn get_mut<'a>(&'a mut self, pos: P) -> Option<Self::RefMut<'a>>
    where
        Self::Output: 'a,
    {
        self.position_to_index(pos).map(|i| unsafe { self.pixels_mut().get_unchecked_mut(i) })
    }
    #[inline(always)]
    #[track_caller]
    unsafe fn get_unchecked_mut<'a>(&'a mut self, pos: P) -> Self::RefMut<'a>
    where
        Self::Output: 'a,
    {
        unsafe {
            let idx = self.position_to_index_unchecked(pos);
            self.values_mut().get_unchecked_mut(idx)
        }
    }
}

impl<P, T, Idx, A: Allocator + Clone> GetManyMut<P> for ImageBaseOf<T, Idx, A>
where
    Idx: Integer,
    P: Into<Vector2<Idx>>,
{
    #[inline(always)]
    fn try_get_many_mut<'a, const N2: usize>(&'a mut self, indices: [P; N2]) -> Result<[Self::RefMut<'a>; N2], ManyMutError>
    where
        Self::Output: 'a,
    {
        // Use try_map https://doc.rust-lang.org/std/primitive.array.html#method.try_map when #stabilized
        let indices = indices.map(|pos| self.position_to_index(pos));
        if indices.any(|x| x.is_none()) {
            Err(ManyMutError::IndexOutOfBounds)
        } else {
            self.pixels_mut().try_get_many_mut(indices.map(|idx| idx.unwrap()))
        }
    }

    fn get_many_mut<'a, const N: usize>(&'a mut self, indices: [P; N]) -> Option<[Self::RefMut<'a>; N]>
    where
        Self::Output: 'a,
    {
        // Use try_map https://doc.rust-lang.org/std/primitive.array.html#method.try_map when #stabilized
        let indices = indices.map(|pos| self.position_to_index(pos));
        if indices.any(|x| x.is_none()) {
            None
        } else {
            self.pixels_mut().get_many_mut(indices.map(|idx| idx.unwrap()))
        }
    }
}

impl<P, T, Idx, A: Allocator + Clone> Index<P> for ImageBaseOf<T, Idx, A>
where
    Idx: Integer,
    P: Into<Vector2<Idx>>,
{
    type Output = T;
    #[inline(always)]
    fn index(&self, index: P) -> &Self::Output {
        self.get_or_panic(index)
    }
}
impl<P, T, Idx, A: Allocator + Clone> IndexMut<P> for ImageBaseOf<T, Idx, A>
where
    Idx: Integer,
    P: Into<Vector2<Idx>>,
{
    #[inline(always)]
    fn index_mut(&mut self, index: P) -> &mut Self::Output {
        self.get_mut_or_panic(index)
    }
}

impl<T, Idx, A: Allocator + Clone> Length for ImageBaseOf<T, Idx, A>
where
    Idx: Integer,
{
    #[inline(always)]
    fn len(&self) -> usize {
        self.values().len()
    }
}

impl<T, Idx, A: Allocator + Clone> Crop<Idx, 2> for ImageBaseOf<T, Idx, A>
where
    Idx: Integer,
    T: Clone,
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

impl<T, Idx, A: Allocator + Clone> Map for ImageBaseOf<T, Idx, A>
where
    Idx: Integer,
{
    type Item = T;
    fn map<F>(mut self, f: F) -> Self
    where
        F: FnMut(T) -> T,
    {
        self.pixels = self.pixels.map(f);
        self
    }
}
impl<T, Idx, A: Allocator + Clone> MapInto for ImageBaseOf<T, Idx, A>
where
    Idx: Integer,
{
    type WithType<R> = ImageBaseOf<R, Idx, A>;
    fn map_into<F, R>(self, f: F) -> Self::WithType<R>
    where
        F: FnMut(T) -> R,
    {
        unsafe { Self::WithType::<R>::from_vec_unchecked(self.size, self.pixels.map_into(f)) }
    }
}
impl<T, Idx, A: Allocator + Clone + Default> MapZip for ImageBaseOf<T, Idx, A>
where
    Idx: Integer,
{
    fn map_zip<F, const M: usize>(this: [Self; M], f: F) -> Self
    where
        F: FnMut([T; M]) -> T,
    {
        if M == 0 {
            return ___();
        }
        let size = this[0].size();
        assert!((&this).all(|g| g.size() == size), "size mismatch");
        unsafe { Self::from_vec_unchecked(size, <Vec<T, A> as MapZip>::map_zip(this.map(|g| g.pixels), f)) }
    }
}
impl<T, Idx, A: Allocator + Clone + Default> MapZipInto for ImageBaseOf<T, Idx, A>
where
    Idx: Integer,
{
    fn map_zip_into<F, R, const M: usize>(this: [Self; M], f: F) -> Self::WithType<R>
    where
        F: FnMut([T; M]) -> R,
    {
        if M == 0 {
            return ___();
        }
        let size = this[0].size();
        assert!((&this).all(|g| g.size() == size), "size mismatch");
        unsafe { Self::WithType::<R>::from_vec_unchecked(size, <Vec<T, A> as MapZipInto>::map_zip_into(this.map(|g| g.pixels), f)) }
    }
}

//fn save_custom_extensions() -> impl Iterator<Item = &'static extension> { ["png"].into_iter() }
fn save_extensions() -> impl Iterator<Item = &'static extension> {
    ["png", "jpg", "jpeg", "gif", "ico", "webp", "bmp"].into_iter()
}
fn load_extensions() -> impl Iterator<Item = &'static extension> {
    save_extensions()
}

fn extension_2_image_enum(extension: &str) -> Option<ImageFormat> {
    //ImageFormat::from_extension(ext) // for full support
    Some(match extension {
        "png" => ImageFormat::Png,
        "jpg" | "jpeg" => ImageFormat::Jpeg,
        //"gif" => ImageFormat::Gif,
        "ico" => ImageFormat::Ico,
        "webp" => ImageFormat::WebP,
        "bmp" => ImageFormat::Bmp,
        "gif" => ImageFormat::Gif,
        other => return None,
    })
}

impl<C, Idx, A: Allocator + Clone> ImageBaseOf<C, Idx, A>
where
    Idx: Integer,
    C: BitAllUsed,
{
    fn _save_to_writer<'ext, W>(&self, mut writer: W, extension: Option<&'ext extension>, color: ColorType) -> EncodeResult<DeducedExtension<'ext>>
    where
        W: Write,
        Self: Save,
        Idx: CfgSerialize,
        C: IColor<ToRgba<u8> = RgbaOf<u8>>,
        u8: CastRangeFrom<C::Component>,
    {
        let unknow_extension = || EncodeError::save_unsupported_extension::<Self>(extension.map(|e| e.to_owned().into()));
        let deduced_extension = extension.unwrap_or(&"png");
        let image_format = extension_2_image_enum(deduced_extension).ok_or_else(unknow_extension)?;

        let w = self.width().to_usize() as _;
        let h = self.height().to_usize() as _;

        let data: &[u8] = hexga_core::bit::transmute_slice(&self.pixels);

        let r = match image_format {
            ImageFormat::Png => PngEncoder::new(&mut writer).write_image(data, w, h, color.into()),
            ImageFormat::Jpeg => JpegEncoder::new_with_quality(&mut writer, 95).write_image(data, w, h, color.into()),
            ImageFormat::Bmp => BmpEncoder::new(&mut writer).write_image(data, w, h, color.into()),
            ImageFormat::Ico => IcoEncoder::new(&mut writer).write_image(data, w, h, color.into()),
            ImageFormat::WebP => WebPEncoder::new_lossless(&mut writer).write_image(data, w, h, color.into()),
            ImageFormat::Gif => {
                if color != ColorType::Rgba8 {
                    // Gif only support RgbaU8, so cast range other color.
                    return self.to_rgba_u8()._save_to_writer(writer, extension, color);
                }
                GifEncoder::new(&mut writer).write_image(data, w, h, color.into())
            }
            _ => return Err(unknow_extension()),
        };

        match r {
            Ok(_) => Ok(deduced_extension.into()),
            Err(e) => Err(EncodeError::custom(format!(
                "Failed to encode .{deduced_extension} {image_format:?} image : {}",
                e
            ))),
        }
    }

    fn _load_from_reader<R>(
        mut reader: R, extension: Option<&extension>, f: impl FnOnce(DynamicImage, Vector2<Idx>) -> EncodeResult<Self>,
    ) -> EncodeResult<Self>
    where
        Self: Sized + Load,
        R: std::io::Read,
    {
        // Todo can check image extension (orhterwise more extension are supported than the one declared)

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

        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes)?;

        let img = ::image::load_from_memory(&bytes).map_err(|e| EncodeError::load_unsupported_extension::<Self>(extension.map(|e| e.to_owned().into())))?;

        let (width, height): (u32, u32) = img.dimensions();
        let w = Idx::cast_from(width);
        let h = Idx::cast_from(height);
        let casted_width = w.to_u32();
        let casted_height = h.to_u32();
        if casted_width != width || height != casted_height {
            return Err(EncodeError::custom(format!("Image is too big: {}", vector2(width, height))));
        }

        f(img, vector2(w, h))
    }
}

impl<Idx, A: Allocator + Clone> Save for ImageBaseOf<RgbaOf<bool>, Idx, A>
where
    Idx: Integer + CfgSerialize,
{
    fn save_custom_extensions() -> impl Iterator<Item = &'static extension> {
        save_extensions()
    }
    fn save_to_writer_with_custom_extension<'ext, W>(&self, writer: W, extension: Option<&'ext extension>) -> EncodeResult<DeducedExtension<'ext>>
    where
        W: Write,
    {
        // Todo Can remove this clone()
        self.clone().to_u8_range().save_to_writer_with_custom_extension(writer, extension)
    }
}
impl<Idx, A: Allocator + Clone> Save for ImageBaseOf<ColorU8, Idx, A>
where
    Idx: Integer + CfgSerialize,
{
    fn save_custom_extensions() -> impl Iterator<Item = &'static extension> {
        save_extensions()
    }
    fn save_to_writer_with_custom_extension<'ext, W>(&self, writer: W, extension: Option<&'ext extension>) -> EncodeResult<DeducedExtension<'ext>>
    where
        W: Write,
    {
        self._save_to_writer(writer, extension, ColorType::Rgba8)
    }
}
impl<Idx, A: Allocator + Clone> Save for ImageBaseOf<ColorU16, Idx, A>
where
    Idx: Integer + CfgSerialize,
{
    fn save_custom_extensions() -> impl Iterator<Item = &'static extension> {
        save_extensions()
    }
    fn save_to_writer_with_custom_extension<'ext, W>(&self, writer: W, extension: Option<&'ext extension>) -> EncodeResult<DeducedExtension<'ext>>
    where
        W: Write,
    {
        self._save_to_writer(writer, extension, ColorType::Rgba16)
    }
}
impl<Idx, A: Allocator + Clone> Save for ImageBaseOf<ColorF32, Idx, A>
where
    Idx: Integer + CfgSerialize,
{
    fn save_custom_extensions() -> impl Iterator<Item = &'static extension> {
        save_extensions()
    }
    fn save_to_writer_with_custom_extension<'ext, W>(&self, writer: W, extension: Option<&'ext extension>) -> EncodeResult<DeducedExtension<'ext>>
    where
        W: Write,
    {
        self._save_to_writer(writer, extension, ColorType::Rgba32F)
    }
}
impl<Idx, A: Allocator + Clone> Save for ImageBaseOf<ColorF64, Idx, A>
where
    Idx: Integer + CfgSerialize,
{
    fn save_custom_extensions() -> impl Iterator<Item = &'static extension> {
        save_extensions()
    }
    fn save_to_writer_with_custom_extension<'ext, W>(&self, writer: W, extension: Option<&'ext extension>) -> EncodeResult<DeducedExtension<'ext>>
    where
        W: Write,
    {
        self.to_rgba_f32().save_to_writer_with_custom_extension(writer, extension)
    }
}

impl<Idx, A: Allocator + Clone> Save for ImageBaseOf<HslaF32, Idx, A>
where
    Idx: Integer + CfgSerialize,
{
    fn save_custom_extensions() -> impl Iterator<Item = &'static extension> {
        save_extensions()
    }
    fn save_to_writer_with_custom_extension<'ext, W>(&self, writer: W, extension: Option<&'ext extension>) -> EncodeResult<DeducedExtension<'ext>>
    where
        W: Write,
    {
        self.to_rgba_f32().save_to_writer_with_custom_extension(writer, extension)
    }
}
impl<Idx, A: Allocator + Clone> Save for ImageBaseOf<HslaF64, Idx, A>
where
    Idx: Integer + CfgSerialize,
{
    fn save_custom_extensions() -> impl Iterator<Item = &'static extension> {
        save_extensions()
    }
    fn save_to_writer_with_custom_extension<'ext, W>(&self, writer: W, extension: Option<&'ext extension>) -> EncodeResult<DeducedExtension<'ext>>
    where
        W: Write,
    {
        self.to_rgba_f32().save_to_writer_with_custom_extension(writer, extension)
    }
}

impl<C, Idx, A: Allocator + Clone> MediaType for ImageBaseOf<C, Idx, A>
where
    Idx: Integer,
{
    fn media_type() -> &'static str {
        "image"
    }
}

impl<Idx, A: Allocator + Clone + Default> Load for ImageBaseOf<ColorU8, Idx, A>
where
    Idx: Integer + for<'de> CfgDeserialize<'de>,
{
    fn load_custom_extensions() -> impl Iterator<Item = &'static extension> {
        load_extensions()
    }

    fn load_from_reader_with_custom_extension<R>(reader: R, extension: Option<&extension>) -> EncodeResult<Self>
    where
        Self: Sized,
        R: std::io::Read,
    {
        Self::_load_from_reader(reader, extension, |img, size| {
            let rgba_bytes = match img {
                DynamicImage::ImageRgba8(rgba8) => rgba8,
                x => x.to_rgba8(),
            }
            .into_raw();

            if rgba_bytes.len() != core::mem::size_of::<ColorU8>() * size.area_usize() {
                return Err(EncodeError::custom("Invalid bytes len"));
            }

            let mut pixels = Vec::with_capacity_in(size.area_usize(), A::default());
            pixels.extend(rgba_bytes.as_chunks::<4>().0.iter().map(|c| ColorU8::rgba(c[0], c[1], c[2], c[3])));
            Self::from_vec(size, pixels).ok_or_else(|| EncodeError::custom("Invalid bytes len"))
        })
    }
}

impl<Idx, A: Allocator + Clone + Default> Load for ImageBaseOf<ColorU16, Idx, A>
where
    Idx: Integer + for<'de> CfgDeserialize<'de>,
{
    fn load_custom_extensions() -> impl Iterator<Item = &'static extension> {
        load_extensions()
    }

    fn load_from_reader_with_custom_extension<R>(reader: R, extension: Option<&extension>) -> EncodeResult<Self>
    where
        Self: Sized,
        R: std::io::Read,
    {
        Self::_load_from_reader(reader, extension, |img, size| {
            let rgba_bytes = match img {
                DynamicImage::ImageRgba16(rgba16) => rgba16,
                x => x.to_rgba16(),
            }
            .into_raw();

            if rgba_bytes.len() != core::mem::size_of::<ColorU16>() * size.area_usize() {
                return Err(EncodeError::custom("Invalid bytes len"));
            }

            let mut pixels = Vec::with_capacity_in(size.area_usize(), A::default());
            pixels.extend(rgba_bytes.as_chunks::<4>().0.iter().map(|c| ColorU16::rgba(c[0], c[1], c[2], c[3])));
            Self::from_vec(size, pixels).ok_or_else(|| EncodeError::custom("Invalid bytes len"))
        })
    }
}
impl<Idx, A: Allocator + Clone + Default> Load for ImageBaseOf<RgbaOf<bool>, Idx, A>
where
    Idx: Integer + for<'de> CfgDeserialize<'de>,
{
    fn load_custom_extensions() -> impl Iterator<Item = &'static extension> {
        load_extensions()
    }

    fn load_from_reader_with_custom_extension<R>(reader: R, extension: Option<&extension>) -> EncodeResult<Self>
    where
        Self: Sized,
        R: std::io::Read,
    {
        Self::_load_from_reader(reader, extension, |img, size| {
            let rgba_bytes = match img {
                DynamicImage::ImageRgba8(rgba8) => rgba8,
                x => x.to_rgba8(),
            }
            .into_raw();

            if rgba_bytes.len() != core::mem::size_of::<ColorU8>() * size.area_usize() {
                return Err(EncodeError::custom("Invalid bytes len"));
            }

            let mut pixels = Vec::with_capacity_in(size.area_usize(), A::default());
            pixels.extend(
                rgba_bytes
                    .as_chunks::<4>()
                    .0
                    .iter()
                    .map(|c| RgbaOf::<bool>::rgba(c[0] != 0, c[1] != 0, c[2] != 0, c[3] != 0)),
            );
            Self::from_vec(size, pixels).ok_or_else(|| EncodeError::custom("Invalid bytes len"))
        })
    }
}

impl<Idx, A: Allocator + Clone + Default> Load for ImageBaseOf<ColorF32, Idx, A>
where
    Idx: Integer + for<'de> CfgDeserialize<'de>,
{
    fn load_custom_extensions() -> impl Iterator<Item = &'static extension> {
        load_extensions()
    }

    fn load_from_reader_with_custom_extension<R>(reader: R, extension: Option<&extension>) -> EncodeResult<Self>
    where
        Self: Sized,
        R: std::io::Read,
    {
        Self::_load_from_reader(reader, extension, |img, size| {
            let rgba_bytes = match img {
                DynamicImage::ImageRgba32F(rgba32) => rgba32,
                x => x.to_rgba32f(),
            }
            .into_raw();

            if rgba_bytes.len() != core::mem::size_of::<ColorF32>() * size.area_usize() {
                return Err(EncodeError::custom("Invalid bytes len"));
            }

            let mut pixels = Vec::with_capacity_in(size.area_usize(), A::default());
            pixels.extend(rgba_bytes.as_chunks::<4>().0.iter().map(|c| ColorF32::rgba(c[0], c[1], c[2], c[3])));
            Self::from_vec(size, pixels).ok_or_else(|| EncodeError::custom("Invalid bytes len"))
        })
    }
}

impl<Idx, A: Allocator + Clone + Default> Load for ImageBaseOf<ColorF64, Idx, A>
where
    Idx: Integer + for<'de> CfgDeserialize<'de>,
{
    fn load_custom_extensions() -> impl Iterator<Item = &'static extension> {
        load_extensions()
    }

    fn load_from_reader_with_custom_extension<R>(reader: R, extension: Option<&extension>) -> EncodeResult<Self>
    where
        Self: Sized,
        R: std::io::Read,
    {
        Self::_load_from_reader(reader, extension, |img, size| {
            let rgba_bytes = match img {
                DynamicImage::ImageRgba32F(rgba32) => rgba32,
                x => x.to_rgba32f(),
            }
            .into_raw();

            if rgba_bytes.len() != core::mem::size_of::<ColorF32>() * size.area_usize() {
                return Err(EncodeError::custom("Invalid bytes len"));
            }

            let mut pixels = Vec::with_capacity_in(size.area_usize(), A::default());
            pixels.extend(
                rgba_bytes
                    .as_chunks::<4>()
                    .0
                    .iter()
                    .map(|c| ColorF64::rgba(c[0].cast_into(), c[1].cast_into(), c[2].cast_into(), c[3].cast_into())),
            );
            Self::from_vec(size, pixels).ok_or_else(|| EncodeError::custom("Invalid bytes len"))
        })
    }
}

impl<Idx, A: Allocator + Clone + Default> Load for ImageBaseOf<HslaF32, Idx, A>
where
    Idx: Integer + for<'de> CfgDeserialize<'de>,
{
    fn load_custom_extensions() -> impl Iterator<Item = &'static extension> {
        load_extensions()
    }

    fn load_from_reader_with_custom_extension<R>(reader: R, extension: Option<&extension>) -> EncodeResult<Self>
    where
        Self: Sized,
        R: std::io::Read,
    {
        ImageBaseOf::<ColorF32, Idx, A>::load_from_reader_with_custom_extension(reader, extension).map(|img| img.map_into(|c| c.to_hsla_f32()))
    }
}
impl<Idx, A: Allocator + Clone + Default> Load for ImageBaseOf<HslaF64, Idx, A>
where
    Idx: Integer + for<'de> CfgDeserialize<'de>,
{
    fn load_custom_extensions() -> impl Iterator<Item = &'static extension> {
        load_extensions()
    }

    fn load_from_reader_with_custom_extension<R>(reader: R, extension: Option<&extension>) -> EncodeResult<Self>
    where
        Self: Sized,
        R: std::io::Read,
    {
        ImageBaseOf::<ColorF32, Idx, A>::load_from_reader_with_custom_extension(reader, extension).map(|img| img.map_into(|c| c.to_hsla_f64()))
    }
}
