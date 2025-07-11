use std::cell::{Cell, RefCell};
use std::cmp::Reverse;
// do delegation on Serde serialize https://docs.rs/serde/latest/serde/trait.Serialize.html#impl-Serialize-for-str
use std::collections::*;
use std::hash::{BuildHasher, Hash};
use std::marker::PhantomData;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::num::*;
use std::ops::*;
use std::rc::{Rc, Weak as RcWeak};
use std::sync::{Arc, Mutex, RwLock, Weak as ArcWeak};

use crate::*;

macro_rules! impl_io_save {
    (
        $(
            $name:ident $(<$( $generic:ident ),+>)?
        ),* $(,)?
    ) => {
        $(
            impl$(<$( $generic: IoSave ),+>)? IoSave for $name$(<$( $generic ),+>)?
            {
            }

            impl$(<$( $generic: IoLoad ),+>)? IoLoad for $name$(<$( $generic ),+>)?
            {
            }
        )*
    };
}

type Void = ();

impl_io_save!(
    Void,
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,

    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize,
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,

    f32, f64,
    bool,
    char,
    Option<T>, Result<T,E>,
    Vec<T>, LinkedList<T>, VecDeque<T>,
);


impl<K,V,S> IoSave for HashMap<K,V,S> where K : IoSave + Eq + Hash, V : IoSave, S : BuildHasher + Default {}
impl<K,V,S> IoLoad for HashMap<K,V,S> where K : IoLoad + Eq + Hash, V : IoLoad, S : BuildHasher + Default {}

impl<K,  S> IoSave for HashSet<K,  S> where K : IoSave + Eq + Hash,             S : BuildHasher + Default {}
impl<K,  S> IoLoad for HashSet<K,  S> where K : IoLoad + Eq + Hash,             S : BuildHasher + Default {}

impl<K,V> IoSave for BTreeMap<K,V> where K : IoSave + Ord, V : IoSave {}
impl<K,V> IoLoad for BTreeMap<K,V> where K : IoLoad + Ord, V : IoLoad {}

impl<K> IoSave for BTreeSet<K> where K : IoSave + Ord  {}
impl<K> IoLoad for BTreeSet<K> where K : IoLoad + Ord  {}

impl<T> IoSave for BinaryHeap<T> where T: IoSave + Ord  {}
impl<T> IoLoad for BinaryHeap<T> where T: IoLoad + Ord  {}


impl<T> IoSave for &[T] where T: IoSave {}

impl IoSave for str
{
    fn save_own_extensions() -> impl Iterator<Item = &'static str> { ["txt", "md"].iter().copied() }

    fn save_to_with_own_extension_pathless<W, Fs>(&self, _ : &extension, mut w : W, _ : &mut Fs) -> IoResult where W : Write, Fs : IoFsWrite
    {
        w.write_all(self.as_bytes()).map_err(|e| IoErrorKind::from_internal_error(e))
    }
}
impl IoSave for String
{
    fn save_own_extensions() -> impl Iterator<Item = &'static str> { str::save_own_extensions() }

    fn save_to_with_own_extension_pathless<W, Fs>(&self, extension : &extension, w : W, fs : &mut Fs) -> IoResult
            where W : Write, Fs : IoFsWrite
    { self.as_str().save_to_with_own_extension_pathless(extension, w, fs) }
}
impl IoLoad for String
{
    const CAN_BE_LOADED_FROM_TEXT : bool = true;
    fn load_own_extensions() -> impl Iterator<Item = &'static str> { Self::save_own_extensions() }
    fn load_from_str_with_own_extension_pathless(data : &str, _ : &extension) -> IoResult<Self> { Ok(data.to_owned()) }
}



#[cfg(feature = "rc")]
impl_io_save!(
    Rc<T>, RcWeak<T>,
    Arc<T>, ArcWeak<T>,
);

impl<T> IoSave for Cell<T> where T: IoSave + Copy {}
impl<T> IoLoad for Cell<T> where T: IoLoad + Copy {}


// https://docs.rs/serde/latest/serde/trait.Serialize.html#impl-Serialize-for-str
impl_io_save!(
    IpAddr, Ipv4Addr, Ipv6Addr,
    SocketAddr,

    Range<Idx>, RangeFrom<Idx>, RangeInclusive<Idx>, RangeTo<Idx>,

    Bound<T>,

    RefCell<T>,

    Reverse<T>,

    PhantomData<T>,

    Wrapping<T>,

    Mutex<T>, RwLock<T>,
);

impl<T: IoSave> IoSave for Saturating<T> {
}

impl<T: IoLoad> IoLoad for Saturating<T> where for<'de> Saturating<T>: serde::Deserialize<'de> {
}

impl<T, const N : usize> IoSave for [T; N] where [T; N] : Serialize {} //, T : IoSave
impl<T, const N : usize> IoLoad for [T; N] where Self : for<'de> Deserialize<'de> {}


#[cfg_attr(docsrs, doc(fake_variadic))]
#[cfg_attr(
    docsrs,
    doc = "This trait is implemented for tuples up to 16 items long."
)]
impl<T> IoSave for (T,) where T: IoSave {}
impl<T> IoLoad for (T,) where T: IoLoad {}

macro_rules! tuple_impls {
    // Each line provides a count and a list of index-type pairs
    (
        $(
            $len:literal => ( $( $idx:tt $typ:ident )+ )
        )*
    ) => {
        $(
            impl<$( $typ: IoSave ),+> IoSave for ( $( $typ ),+ ) {
                        }

            impl<$( $typ: IoLoad ),+> IoLoad for ( $( $typ ),+ ) {
                        }
        )*
    };
}


tuple_impls! {
    2 => (0 T0 1 T1)
    3 => (0 T0 1 T1 2 T2)
    4 => (0 T0 1 T1 2 T2 3 T3)
    5 => (0 T0 1 T1 2 T2 3 T3 4 T4)
    6 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
    7 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    8 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    9 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    10 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
    11 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
    12 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
    13 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
    14 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
    15 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
    16 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}