use super::*;


/// UB if the functions in this trait are not bijection
///
/// Fn(source) -> target
pub unsafe trait BijectionFn
{
    type Source;
    type Target;

    fn to_target(&self, src: Self::Source) -> Option<Self::Target>;
    fn to_source(&self, target: Self::Target) -> Option<Self::Source>;

    #[inline(always)]
    #[track_caller]
    fn to_target_unchecked(&self, src: Self::Source) -> Self::Target { self.to_target(src).expect("bijection fail") }
    #[inline(always)]
    #[track_caller]
    fn to_source_unchecked(&self, target: Self::Target) -> Self::Source { self.to_source(target).expect("bijection fail") }
}

/// UB if the functions in this trait are not bijection
pub unsafe trait TryBijectionFn: BijectionFn
{
    type SourceToTargetError;
    fn try_to_target(&self, src: Self::Source) -> Result<Self::Target, Self::SourceToTargetError>;
    type TargetToSrcError;
    fn try_to_source(&self, target: Self::Target) -> Result<Self::Source, Self::TargetToSrcError>;
}

pub struct BijectionIdentity<T> { phantom: PhantomData<T> }
impl<T> Default for BijectionIdentity<T> { fn default() -> Self { Self::new() }}
impl<T> Clone for BijectionIdentity<T> { fn clone(&self) -> Self { Self { phantom: PhantomData } } }
impl<T> Copy for BijectionIdentity<T> {}
impl<T> PartialEq for BijectionIdentity<T> { fn eq(&self, _: &Self) -> bool { true } }
impl<T> Eq for BijectionIdentity<T> {}
impl<T> Hash for BijectionIdentity<T> { fn hash<H: Hasher>(&self, _: &mut H) { } }
impl<T> PartialOrd for BijectionIdentity<T> { fn partial_cmp(&self, _: &Self) -> Option<Ordering> { Some(Ordering::Equal) } }
impl<T> Ord for BijectionIdentity<T> { fn cmp(&self, _: &Self) -> Ordering { Ordering::Equal } }
impl<T> Debug for BijectionIdentity<T>  { fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { f.write_str("Identity") } }

impl<T> BijectionIdentity<T>
{
    pub const fn new() -> Self { Self { phantom: PhantomData }}
}

unsafe impl<T> BijectionFn for BijectionIdentity<T>
{
    type Source=T;
    type Target=T;
    #[inline(always)]
    fn to_target(&self, src: Self::Source) -> Option<Self::Target> { Some(src) }
    #[inline(always)]
    fn to_source(&self, target: Self::Target) -> Option<Self::Source> { Some(target) }
    #[inline(always)]
    fn to_target_unchecked(&self, src: Self::Source) -> Self::Target { src }
    #[inline(always)]
    fn to_source_unchecked(&self, target: Self::Target) -> Self::Source { target }
}
unsafe impl<T> TryBijectionFn for BijectionIdentity<T>
{
    type SourceToTargetError=Never;
    #[inline(always)]
    fn try_to_target(&self, src: Self::Source) -> Result<Self::Target, Self::SourceToTargetError> { Ok(src) }
    type TargetToSrcError=Never;
    #[inline(always)]
    fn try_to_source(&self, target: Self::Target) -> Result<Self::Source, Self::TargetToSrcError> { Ok(target) }
}


#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct BijectionInverse<B> { pub bijection: B }

unsafe impl<B> BijectionFn for BijectionInverse<B> where B: BijectionFn
{
    type Source=B::Target;
    type Target=B::Source;
    #[inline(always)]
    fn to_target(&self, src: Self::Source) -> Option<Self::Target> { self.bijection.to_source(src) }
    #[inline(always)]
    fn to_source(&self, target: Self::Target) -> Option<Self::Source> { self.bijection.to_target(target) }
    #[inline(always)]
    fn to_target_unchecked(&self, src: Self::Source) -> Self::Target { self.bijection.to_source_unchecked(src) }
    #[inline(always)]
    fn to_source_unchecked(&self, target: Self::Target) -> Self::Source { self.bijection.to_target_unchecked(target) }
}
unsafe impl<B> TryBijectionFn for BijectionInverse<B> where B: TryBijectionFn
{
    type SourceToTargetError=B::TargetToSrcError;
    #[inline(always)]
    fn try_to_target(&self, src: Self::Source) -> Result<Self::Target, Self::SourceToTargetError> { self.bijection.try_to_source(src) }
    type TargetToSrcError=B::SourceToTargetError;
    #[inline(always)]
    fn try_to_source(&self, target: Self::Target) -> Result<Self::Source, Self::TargetToSrcError> { self.bijection.try_to_target(target) }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct BijectionMap<B1,B2>
{
    pub source: B1,
    pub target: B2,
}

unsafe impl<B1,B2> BijectionFn for BijectionMap<B1,B2>
    where
    B1: BijectionFn,
    B2: BijectionFn<Source=B1::Target>
{
    type Source=B1::Source;
    type Target=B2::Target;
    #[inline(always)]
    fn to_target(&self, src: Self::Source) -> Option<Self::Target> { self.target.to_target(self.source.to_target(src)?) }
    #[inline(always)]
    fn to_source(&self, target: Self::Target) -> Option<Self::Source> { self.source.to_source(self.target.to_source(target)?) }
    #[inline(always)]
    fn to_target_unchecked(&self, src: Self::Source) -> Self::Target { self.target.to_target_unchecked(self.source.to_target_unchecked(src)) }
    #[inline(always)]
    fn to_source_unchecked(&self, target: Self::Target) -> Self::Source { self.source.to_source_unchecked(self.target.to_source_unchecked(target)) }
}
unsafe impl<B1,B2> TryBijectionFn for BijectionMap<B1,B2>
    where
    B1: TryBijectionFn,
    B2: BijectionFn<Source=B1::Target> + TryBijectionFn
{
    type SourceToTargetError=BijectionError<B1::SourceToTargetError,B2::SourceToTargetError>;

    fn try_to_target(&self, src: Self::Source) -> Result<Self::Target, Self::SourceToTargetError> {
        let idx = self.source.try_to_target(src).map_err(|e| BijectionError::Bijection(e))?;
        self.target.try_to_target(idx).map_err(|e| BijectionError::Container(e))
    }

    type TargetToSrcError=BijectionError<B1::TargetToSrcError,B2::TargetToSrcError>;

    fn try_to_source(&self, target: Self::Target) -> Result<Self::Source, Self::TargetToSrcError> {
        let idx = self.target.try_to_source(target).map_err(|e| BijectionError::Container(e))?;
        self.source.try_to_source(idx).map_err(|e| BijectionError::Bijection(e))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct BijectionRev<Idx=usize>
{
    pub len: Idx
}
impl<T> From<T> for BijectionRev<usize> where T: Length
{
    fn from(value: T) -> Self {
        Self { len: value.len() }
    }
}
impl<Idx> BijectionRev<Idx>
{
    pub fn new(len: Idx) -> Self { Self { len }}
}
unsafe impl<Idx> BijectionFn for BijectionRev<Idx> where Idx: Copy + Sub<Idx,Output=Idx> + One
{
    type Source=Idx;
    type Target=Idx;
    #[inline(always)]
    fn to_target(&self, src: Self::Source) -> Option<Self::Target> { Some(self.to_target_unchecked(src)) }
    #[inline(always)]
    fn to_source(&self, target: Self::Target) -> Option<Self::Source> { Some(self.to_source_unchecked(target)) }
    #[inline(always)]
    fn to_target_unchecked(&self, src: Self::Source) -> Self::Target { self.len - src - one() }
    #[inline(always)]
    fn to_source_unchecked(&self, target: Self::Target) -> Self::Source { self.to_target_unchecked(target) }
}
unsafe impl<Idx> TryBijectionFn for BijectionRev<Idx> where Idx: Copy + Sub<Idx,Output=Idx> + One
{
    type SourceToTargetError=Never;

    #[inline(always)]
    fn try_to_target(&self, src: Self::Source) -> Result<Self::Target, Self::SourceToTargetError> {
        Ok(self.to_target_unchecked(src))
    }

    type TargetToSrcError=Never;

    #[inline(always)]
    fn try_to_source(&self, target: Self::Target) -> Result<Self::Source, Self::TargetToSrcError> {
        Ok(self.to_source_unchecked(target))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct BijectionPointToUsize<Idx,const N:usize>
    where Idx: Integer
{
    pub size: Vector<Idx,N>
}
impl<Idx, const N: usize> BijectionPointToUsize<Idx,N> where Idx: Integer
{
    pub const fn new(size: Vector<Idx,N>) -> Self { Self { size }}
}

unsafe impl<Idx, const N: usize> BijectionFn for BijectionPointToUsize<Idx,N>
    where Idx: Integer
{
    type Source=Vector<Idx,N>;
    type Target=usize;

    #[inline(always)]
    fn to_target_unchecked(&self, src: Self::Source) -> Self::Target {
        unsafe { Vector::<Idx,N>::to_index_unchecked(src, self.size) }
    }
    #[inline(always)]
    fn to_target(&self, src: Self::Source) -> Option<Self::Target> {
        src.is_inside(self.size).then(|| self.to_target_unchecked(src))
    }

    #[inline(always)]
    #[track_caller]
    fn to_source_unchecked(&self, target: Self::Target) -> Self::Source {
        unsafe { Vector::<Idx,N>::from_index_unchecked(target, self.size) }
    }
    #[inline(always)]
    #[track_caller]
    fn to_source(&self, dest: Self::Target) -> Option<Self::Source> {
        (dest < self.size.area_usize()).then(|| self.to_source_unchecked(dest))
    }
}
unsafe impl<Idx, const N: usize> TryBijectionFn for BijectionPointToUsize<Idx,N>
    where Idx: Integer
{
    type SourceToTargetError=IndexOutOfRange<Vector<Idx,N>>;
    #[inline(always)]
    fn try_to_target(&self, src: Self::Source) -> Result<Self::Target, Self::SourceToTargetError> {
        self.to_target(src).ok_or(IndexOutOfRange::new(src, zero()..self.size))
    }
    type TargetToSrcError=IndexOutOfRange;
    #[inline(always)]
    fn try_to_source(&self, target: Self::Target) -> Result<Self::Source, Self::TargetToSrcError> {
        self.to_source(target).ok_or(IndexOutOfRange::new(target, zero()..self.size.area_usize()) )
    }
}

impl<Idx, const N: usize> GetPosition<Idx, N> for BijectionPointToUsize<Idx,N> where Idx: Integer
{
    #[inline(always)]
    fn pos(&self) -> Vector<Idx,N> { zero() }
}
impl<Idx, const N: usize> GetSize<Idx, N> for BijectionPointToUsize<Idx,N> where Idx: Integer
{
    #[inline(always)]
    fn size(&self) -> Vector<Idx,N> { self.size }
}
// Todo: impl GetScale ?