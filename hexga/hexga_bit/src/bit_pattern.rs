use super::*;


/// [bytemuck::CheckedBitPattern](https://docs.rs/bytemuck/latest/bytemuck/trait.CheckedBitPattern.html) trait equivalent.
pub trait BitPattern: Copy
{
    type Bits: BitAnyPattern;

    fn from_bits(bits: Self::Bits) -> Option<Self> {
        if Self::is_bit_pattern_valid(&bits) {
            Some(unsafe { *(&bits as *const Self::Bits as *const Self) })
        } else {
            None
        }
    }
    fn is_bit_pattern_valid(bits: &Self::Bits) -> bool;
}

impl<T: BitAnyPattern> BitPattern for T {
    type Bits = T;

    #[inline(always)]
    fn from_bits(bits: Self::Bits) -> Option<Self> {
        Some(bits)
    }
    #[inline(always)]
    fn is_bit_pattern_valid(_bits: &Self::Bits) -> bool {
        true
    }
}

impl BitPattern for char
{
    type Bits = u32;

    #[inline]
    fn from_bits(bits: Self::Bits) -> Option<Self> {
        core::char::from_u32(bits)
    }
    #[inline]
    fn is_bit_pattern_valid(bits: &Self::Bits) -> bool {
        Self::from_bits(*bits).is_some()
    }
}

impl BitPattern for bool
{
    type Bits = u8;

    #[inline]
    fn from_bits(bits: Self::Bits) -> Option<Self> {
        match bits
        {
            0 => Some(false),
            1 => Some(true),
            _ => None,
        }
    }

    fn is_bit_pattern_valid(bits: &Self::Bits) -> bool {
        match bits
        {
            0 | 1 => true,
            _ => false,
        }
    }
}

macro_rules! impl_checked_for_nonzero {
    ($($nonzero:ty: $primitive:ty),* $(,)?) => {
        $(
            impl BitPattern for $nonzero {
                type Bits = $primitive;

                #[inline]
                fn from_bits(bits: Self::Bits) -> Option<Self> {
                    <$nonzero>::new(bits)
                }

                fn is_bit_pattern_valid(bits: &Self::Bits) -> bool {
                    *bits != 0
                }
            }
        )*
    };
}
//TODO: put a similar macro in hexga_map_on ? Map for NonZero + base type. Map for Atomic ?
impl_checked_for_nonzero! {
    core::num::NonZeroU8: u8,
    core::num::NonZeroI8: i8,
    core::num::NonZeroU16: u16,
    core::num::NonZeroI16: i16,
    core::num::NonZeroU32: u32,
    core::num::NonZeroI32: i32,
    core::num::NonZeroU64: u64,
    core::num::NonZeroI64: i64,
    //core::num::NonZeroI128: i128,
    //core::num::NonZeroU128: u128,
    core::num::NonZeroUsize: usize,
    core::num::NonZeroIsize: isize,
}

#[inline(always)]
#[track_caller]
pub(crate) fn is_aligned_to(ptr: *const (), align: usize) -> bool {
    /*
    #[cfg(feature = "align_offset")]
    {
    // This is in a way better than `ptr as usize % align == 0`,
    // because casting a pointer to an integer has the side effect that it
    // exposes the pointer's provenance, which may theoretically inhibit
    // some compiler optimizations.
    ptr.align_offset(align) == 0
    }
    */
    //#[cfg(not(feature = "align_offset"))]
    {
        ((ptr as usize) % align) == 0
    }
}

/// Re-interprets `&[u8]` as `&T`.
///
/// ## Failure
///
/// * If the slice isn't aligned for the new type
/// * If the slice's length isn’t exactly the size of the new type
///
/// Don't check if the bit pattern is valid.
#[inline]
pub unsafe fn try_from_bytes_unchecked<T>(bytes: &[u8]) -> BitResult<&T>
{
    if bytes.len() != size_of::<T>()
    {
        Err(BitError::SizeMismatch)
    } else if !is_aligned_to(bytes.as_ptr() as *const (), align_of::<T>()) {
        Err(BitError::TargetAlignmentGreaterAndInputNotAligned)
    } else {
        Ok(unsafe { &*(bytes.as_ptr() as *const T) })
    }
}

/// Re-interprets `&[u8]` as `&T`.
///
/// ## Failure
///
/// * If the slice isn't aligned for the new type
/// * If the slice's length isn’t exactly the size of the new type
/// * If the slice contains an invalid bit pattern for `T`
#[inline]
pub fn try_from_bytes<T: BitPattern>(bytes: &[u8]) -> BitResult<&T>
{
    let pod = unsafe { try_from_bytes_unchecked(bytes) }?;
    if <T as BitPattern>::is_bit_pattern_valid(pod)
    {
        Ok(unsafe { &*(pod as *const <T as BitPattern>::Bits as *const T) })
    }else
    {
        Err(BitError::InvalidBitPattern)
    }
}

/// Re-interprets `&[u8]` as `&T`.
///
/// ## Panics
///
/// This is [`try_from_bytes`] but will panic on error.
#[inline(always)]
#[track_caller]
pub fn from_bytes<T: BitPattern>(bytes: &[u8]) -> &T
{
    try_from_bytes(bytes).expect("invalid bits pattern")
}


/// Re-interprets `&mut [u8]` as `&mut T`.
///
/// ## Failure
///
/// * If the slice isn't aligned for the new type
/// * If the slice's length isn’t exactly the size of the new type
///
/// Don't check if the bit pattern is valid.
#[inline]
pub unsafe fn try_from_bytes_mut_unchecked<T>(bytes: &mut [u8]) -> BitResult<&mut T>
{
    if bytes.len() != size_of::<T>()
    {
        Err(BitError::SizeMismatch)
    } else if !is_aligned_to(bytes.as_mut_ptr() as *mut (), align_of::<T>()) {
        Err(BitError::TargetAlignmentGreaterAndInputNotAligned)
    } else {
        Ok(unsafe { &mut *(bytes.as_mut_ptr() as *mut T) })
    }
}

/// Re-interprets `&mut [u8]` as `&mut T`.
///
/// ## Failure
///
/// * If the slice isn't aligned for the new type
/// * If the slice's length isn’t exactly the size of the new type
/// * If the slice contains an invalid bit pattern for `T`
#[inline]
pub fn try_from_bytes_mut<T: BitPattern>(bytes: &mut [u8]) -> BitResult<&mut T>
{
    let pod = unsafe { try_from_bytes_mut_unchecked(bytes) }?;
    if <T as BitPattern>::is_bit_pattern_valid(pod)
    {
        Ok(unsafe { &mut *(pod as *mut <T as BitPattern>::Bits as *mut T) })
    }else
    {
        Err(BitError::InvalidBitPattern)
    }
}

/// Re-interprets `&mut [u8]` as `&mut T`.
///
/// ## Panics
///
/// This is [`try_from_bytes`] but will panic on error.
#[inline(always)]
#[track_caller]
pub fn from_bytes_mut<T: BitPattern>(bytes: &mut [u8]) -> &mut T
{
    try_from_bytes_mut(bytes).expect("invalid bits pattern")
}



/// Try to convert `&[Src]` into `&[Dst]` (possibly with a change in length).
///
/// * `input.as_ptr() as usize == output.as_ptr() as usize`
/// * `input.len() * size_of::<Src>() == output.len() * size_of::<Dst>()`
///
/// ## Failure
///
/// * If the target type has a greater alignment requirement and the input slice
///   isn't aligned.
/// * If the target element type is a different size from the current element
///   type, and the output slice wouldn't be a whole number of elements when
///   accounting for the size change (eg: 3 `u16` values is 1.5 `u32` values, so
///   that's a failure).
#[inline]
pub unsafe fn try_cast_slice_unchecked<Src: Copy, Dst: Copy>(src: &[Src]) -> BitResult<&[Dst]>
{
    let input_bytes = core::mem::size_of_val::<[Src]>(src);

    if align_of::<Dst>() > align_of::<Src>() && !is_aligned_to(src.as_ptr() as *const (), align_of::<Dst>())
    {
        Err(BitError::TargetAlignmentGreaterAndInputNotAligned)
    } else if size_of::<Dst>() == size_of::<Src>()
    {
        Ok(unsafe { core::slice::from_raw_parts(src.as_ptr() as *const Dst, src.len()) })
    } else if (size_of::<Dst>() != 0 && input_bytes % size_of::<Dst>() == 0) || (size_of::<Dst>() == 0 && input_bytes == 0)
    {
    let new_len = if size_of::<Dst>() != 0 { input_bytes / size_of::<Dst>() } else { 0 };
        Ok(unsafe { core::slice::from_raw_parts(src.as_ptr() as *const Dst, new_len) })
    } else {
        Err(BitError::OutputSliceWouldHaveSlop)
    }
}

/// Try to convert `&[A]` into `&[B]` (possibly with a change in length).
///
/// * `input.as_ptr() as usize == output.as_ptr() as usize`
/// * `input.len() * size_of::<A>() == output.len() * size_of::<B>()`
///
/// ## Failure
///
/// * If the target type has a greater alignment requirement and the input slice
///   isn't aligned.
/// * If the target element type is a different size from the current element
///   type, and the output slice wouldn't be a whole number of elements when
///   accounting for the size change (eg: 3 `u16` values is 1.5 `u32` values, so
///   that's a failure).
/// * If any element of the converted slice would contain an invalid bit pattern
///   for `B` this fails.
#[inline]
pub fn try_cast_slice<Src: BitAllUsed, Dst: BitPattern>(a: &[Src]) -> BitResult<&[Dst]>
{
    let pod = unsafe { crate::try_cast_slice_unchecked(a) }?;

    if pod.iter().all(|pod| <Dst as BitPattern>::is_bit_pattern_valid(pod)) {
        Ok(unsafe {
            core::slice::from_raw_parts(pod.as_ptr() as *const Dst, pod.len())
        })
    } else {
        Err(BitError::InvalidBitPattern)
    }
}

/// Cast `&[Src]` into `&[Dst]`.
///
/// ## Panics
///
/// This is [`try_cast_slice`] but will panic on error.
#[inline(always)]
#[track_caller]
pub fn cast_slice<Src: BitAllUsed, Dst: BitPattern>(a: &[Src]) -> &[Dst] {
    try_cast_slice(a).expect("invalid bits pattern")
}

/// Cast `&mut [Src]` into `&mut [Dst]`.
///
/// ## Panics
///
/// This is [`try_cast_slice_mut`] but will panic on error.
#[inline(always)]
#[track_caller]
pub fn cast_slice_mut<A: BitAllUsed + BitAnyPattern, B: BitAllUsed + BitPattern>(a: &mut [A]) -> &mut [B] {
    try_cast_slice_mut(a).expect("invalid bits pattern")
}


/// Try to convert `&mut [Src]` into `&mut [Dst]` (possibly with a change in
/// length).
///
/// As [`try_cast_slice`], but `&mut`.
#[inline]
pub fn try_cast_slice_mut<Src: BitAllUsed + BitAnyPattern, Dst: BitPattern + BitAllUsed>(a: &mut [Src]) -> BitResult<&mut [Dst]>
{
    let pod = unsafe { crate::try_cast_slice_mut_unchecked(a) }?;

    if pod.iter().all(|pod| <Dst as BitPattern>::is_bit_pattern_valid(pod)) {
        Ok(unsafe {
            core::slice::from_raw_parts_mut(pod.as_mut_ptr() as *mut Dst, pod.len())
        })
    } else {
        Err(BitError::InvalidBitPattern)
    }
}


/// Try to convert `&mut [Src]` into `&mut [Dst]` (possibly with a change in length).
///
/// As [`try_cast_slice`], but `&mut`.
#[inline]
pub unsafe fn try_cast_slice_mut_unchecked<Src: Copy, Dst: Copy>(src: &mut [Src]) -> BitResult<&mut [Dst]>
{
    let input_bytes = core::mem::size_of_val::<[Src]>(src);

    if align_of::<Dst>() > align_of::<Src>() && !is_aligned_to(src.as_mut_ptr() as *mut (), align_of::<Dst>())
    {
        Err(BitError::TargetAlignmentGreaterAndInputNotAligned)
    } else if size_of::<Dst>() == size_of::<Src>()
    {
        Ok(unsafe { core::slice::from_raw_parts_mut(src.as_mut_ptr() as *mut Dst, src.len()) })
    } else if (size_of::<Dst>() != 0 && input_bytes % size_of::<Dst>() == 0) || (size_of::<Dst>() == 0 && input_bytes == 0)
    {
    let new_len = if size_of::<Dst>() != 0 { input_bytes / size_of::<Dst>() } else { 0 };
        Ok(unsafe { core::slice::from_raw_parts_mut(src.as_mut_ptr() as *mut Dst, new_len) })
    } else {
        Err(BitError::OutputSliceWouldHaveSlop)
    }
}