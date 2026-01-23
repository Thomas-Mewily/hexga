use super::*;

pub trait WithBijection : Sized + Collection
{
    fn with_bijection<B>(self, bijection: B) -> Bijection<Self,B>
    {
        Bijection::from_values_and_bijection(self, bijection)
    }
}
impl<T> WithBijection for T where T: Sized + Collection {}

pub trait WithBijectionExtension : WithBijection
{
    fn bijection_rev(self) -> Bijection<Self,BijectionRev> where Self: Length { let len = self.len(); self.with_bijection(BijectionRev::new(len)) }
}
impl<T> WithBijectionExtension for T where T: WithBijection {}

/// A View Type with bijection
///
/// Can be used for N-dimensional grid. Can have a finite or infinite size
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Bijection<C, B>
    //where C: GetManyMut<B::Target>, B: Bijection,
{
    pub(crate) values: C,
    pub(crate) bijection_fn: B,
}
impl<C, B> View for Bijection<C, B> where C: View, B: Copy
{
    type View<'v> = Bijection<C::View<'v>, B> where Self: 'v;
    fn view<'s>(&'s self) -> Self::View<'s> {
        Bijection::from_values_and_bijection(self.values.view(), self.bijection_fn)
    }
}
impl<C, B> ViewMut for Bijection<C, B> where C: ViewMut, B: Copy
{
    type ViewMut<'v> = Bijection<C::ViewMut<'v>, B> where Self: 'v;
    fn view_mut<'s,'v>(&'s mut self) -> Self::ViewMut<'v> where 's: 'v {
        Bijection::from_values_and_bijection(self.values.view_mut(), self.bijection_fn)
    }
}

impl<C, B> Bijection<C, B>
{
    pub fn from_values_and_bijection(values: C, bijection: B) -> Self
    {
        Self { values, bijection_fn: bijection }
    }
    pub fn into_values_and_bijection(self) -> (C, B)
    {
        let Self { values, bijection_fn: bijection } = self;
        (values, bijection)
    }

    pub fn values(&self) -> &C { &self.values }
    pub fn values_mut(&mut self) -> &mut C { &mut self.values }

    pub fn bijection_fn(&self) -> &B { &self.bijection_fn }
    pub fn bijection_fn_mut(&mut self) -> &mut B { &mut self.bijection_fn }
}



impl<C, B> MapIntern for Bijection<C, B> where C: MapIntern
{
    type Item=<C as MapIntern>::Item;
    fn map_intern<F>(mut self, f: F) -> Self where F: FnMut(Self::Item) -> Self::Item {
        self.values = self.values.map_intern(f);
        self
    }
}
impl<C, B> MapInternWith for Bijection<C, B> where C: MapInternWith
{
    fn map_with_intern<F>(mut self, other: Self, f: F) -> Self where F: FnMut(Self::Item, Self::Item) -> Self::Item {
        self.values = self.values.map_with_intern(other.values, f);
        self
    }
}
impl<C, B> Map for Bijection<C, B> where C: Map
{
    type WithType<R> = Bijection<C::WithType<R>,B>;

    fn map<R,F>(self, f: F) -> Self::WithType<R> where F: FnMut(Self::Item) -> R {
        Bijection::<C::WithType<R>,B>{ bijection_fn: self.bijection_fn, values: self.values.map(f) }
    }
}
impl<C, B> MapWith for Bijection<C, B> where C: MapWith
{
    fn map_with<R, Item2, F>(self, other: Self::WithType<Item2>, f : F) -> Self::WithType<R> where F: FnMut(Self::Item, Item2) -> R {
        Bijection::<C::WithType<R>,B>{ bijection_fn: self.bijection_fn, values: self.values.map_with(other.values, f) }
    }
}

/*
// Todo: re impl it
impl<C, B> ContainerBijection<C, B>
    where C: GetManyMut<B::Target>, B: Bijection,
{
    impl_number_basic_trait!();
}
*/

/*
impl<C, B> GetPosition<Idx, N> for ContainerBijection<C, B>
    where C: GetManyMut<B::Target>, B: Bijection,
{
    #[inline(always)]
    fn pos(&self) -> Vector<Idx,N> { zero() }
}
impl<C, B> GetSize<Idx, N> for ContainerBijection<C, B>
    where C: GetManyMut<B::Target>, B: Bijection,
{
    #[inline(always)]
    fn size(&self) -> Vector<Idx, N> { self.size }
}
*/

impl<C, B> Collection for Bijection<C, B>
    where C: Collection {}
impl<Idx, C, B> Get<Idx> for Bijection<C, B>
    where B: BijectionFn, C: Get<B::Target>, Idx: Into<B::Source>
{
    type Output=C::Output;
    fn get(&self, index: Idx) -> Option<&Self::Output> {
        let target_idx = self.bijection_fn.to_target(index.into())?;
        self.values.get(target_idx)
    }

    #[track_caller]
    unsafe fn get_unchecked(&self, index: Idx) -> &Self::Output {
        let target_idx = self.bijection_fn.to_target(index.into()).expect("bijection fail");
        unsafe { self.values.get_unchecked(target_idx) }
    }
}
impl<Idx, C, B> Index<Idx> for Bijection<C, B>
    where B: BijectionFn, C: Get<B::Target>, Idx: Into<B::Source>
{
    type Output=C::Output;
    #[inline(always)]
    #[track_caller]
    fn index(&self, index: Idx) -> &Self::Output { self.get_or_panic(index) }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum BijectionError<Bijection,Container>
{
    Bijection(Bijection),
    Container(Container),
}

impl<Idx, C, B> TryGet<Idx> for Bijection<C, B>
    where B: TryBijectionFn, C: TryGet<B::Target>, Idx: Into<B::Source>
{
    type Error=BijectionError<B::SourceToTargetError,C::Error>;
    fn try_get(&self, index: Idx) -> Result<&Self::Output, Self::Error> {
        let target_idx = self.bijection_fn.try_to_target(index.into())
            .map_err(|b| BijectionError::Bijection(b))?;
        self.values.try_get(target_idx).map_err(|c| BijectionError::Container(c))
    }
}

impl<Idx, C, B> GetMut<Idx> for Bijection<C, B>
    where B: BijectionFn, C: GetMut<B::Target>, Idx: Into<B::Source>
{
    fn get_mut(&mut self, index: Idx) -> Option<&mut Self::Output> {
        let target_idx = self.bijection_fn.to_target(index.into())?;
        self.values.get_mut(target_idx)
    }

    unsafe fn get_unchecked_mut(&mut self, index: Idx) -> &mut Self::Output {
        let target_idx = self.bijection_fn.to_target(index.into()).expect("bijection fail");
        unsafe { self.values.get_unchecked_mut(target_idx) }
    }
}
impl<Idx, C, B> IndexMut<Idx> for Bijection<C, B>
    where B: BijectionFn, C: GetMut<B::Target>, Idx: Into<B::Source>
{
    #[inline(always)]
    #[track_caller]
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output { self.get_mut_or_panic(index) }
}


impl<Idx, C, B> TryGetMut<Idx> for Bijection<C, B>
    where B: TryBijectionFn, C: TryGetMut<B::Target>, Idx: Into<B::Source>
{
    fn try_get_mut(&mut self, index: Idx) -> Result<&mut Self::Output, Self::Error> {
        let target_idx = self.bijection_fn.try_to_target(index.into())
            .map_err(|b| BijectionError::Bijection(b))?;
        self.values.try_get_mut(target_idx).map_err(|c| BijectionError::Container(c))
    }
}

impl<Idx, C, B> GetManyMut<Idx> for Bijection<C, B>
    where B: TryBijectionFn, C: GetManyMut<B::Target>, Idx: Into<B::Source>
{
    fn try_get_many_mut<const N: usize>(&mut self, indices: [Idx; N]) -> Result<[&mut Self::Output;N], ManyMutError> {
        let idx = indices.try_map(|idx| { self.bijection_fn.to_target(idx.into()) }).ok_or(ManyMutError::IndexOutOfBounds)?;
        self.values.try_get_many_mut(idx)
    }
}

impl<C, B> Length for Bijection<C, B> where C: Length
{
    #[inline(always)]
    fn len(&self) -> usize { self.values.len() }
}

/*
impl<C, B> IntoIterator for Bijection<C, B> where C: IntoIterator
{
    type Item=C::Item;
    type IntoIter=C::IntoIter;
    fn into_iter(self) -> Self::IntoIter { self.values.into_iter() }
}
*/

/*
impl<C, B> Crop<Idx,N> for ContainerBijection<C, B>
    where C: GetManyMut<B::Target>, B: Bijection,, T: Clone
{
    fn crop(self, subrect: Rectangle<Idx, N>) -> Option<Self> {
        self.view().crop(subrect).map(|v| v.to_grid())
    }
}

pub struct Grid2ViewOf<T, Idx, N>
{

}

impl<C, B> View for ContainerBijection<C, B>
    where C: GetManyMut<B::Target>, B: Bijection,,
{
    type View<'v> where Self: 'v;

    fn view<'s>(&'s self) -> Self::View<'s> {
        todo!()
    }
}
*/