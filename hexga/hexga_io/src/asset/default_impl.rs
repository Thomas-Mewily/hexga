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

use super::*;

macro_rules! impl_io_save {
    (
        $(
            $name:ident $(<$( $generic:ident ),+>)?
        ),* $(,)?
    ) => {
        $(
            impl$(<$( $generic: Save ),+>)? Save for $name$(<$( $generic ),+>)?
            {
            }

            impl$(<$( $generic: Load ),+>)? Load for $name$(<$( $generic ),+>)?
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


impl<K,V,S> Save for HashMap<K,V,S> where K : Save + Eq + Hash, V : Save, S : BuildHasher + Default {}
impl<K,V,S> Load for HashMap<K,V,S> where K : Load + Eq + Hash, V : Load, S : BuildHasher + Default {}

impl<K,  S> Save for HashSet<K,  S> where K : Save + Eq + Hash,             S : BuildHasher + Default {}
impl<K,  S> Load for HashSet<K,  S> where K : Load + Eq + Hash,             S : BuildHasher + Default {}

impl<K,V> Save for BTreeMap<K,V> where K : Save + Ord, V : Save {}
impl<K,V> Load for BTreeMap<K,V> where K : Load + Ord, V : Load {}

impl<K> Save for BTreeSet<K> where K : Save + Ord  {}
impl<K> Load for BTreeSet<K> where K : Load + Ord  {}

impl<T> Save for BinaryHeap<T> where T: Save + Ord  {}
impl<T> Load for BinaryHeap<T> where T: Load + Ord  {}


impl<T> Save for &[T] where T: Save {}

impl Save for str
{
    fn save_custom_extensions() -> impl Iterator<Item = &'static extension> {
        [
            "txt",
            "md",
            "csv", // Comma Separated Values
            "tsv" // Tab-Separated Values
        ].iter().copied()
    }

    fn save_to_with_custom_extension<Fs>(&self, path: &path, _extension: &extension, fs: &mut Fs) -> IoResult where Fs: FsWrite {
        fs.write_str(path, self)
    }
}
impl Save for String
{
    fn save_custom_extensions() -> impl Iterator<Item = &'static extension> {
        str::save_custom_extensions()
    }
    fn save_default_extension() -> Option<&'static str> {
        str::save_default_extension()
    }
    fn save_to_with_custom_extension<Fs>(&self, path: &path, extension: &extension, fs: &mut Fs) -> IoResult where Fs: FsWrite {
        self.as_str().save_to_with_custom_extension(path, extension, fs)
    }
}
impl Load for String
{
    fn load_custom_extensions() -> impl Iterator<Item = &'static extension> {
        Self::save_custom_extensions()
    }
    fn load_from_with_custom_extension<Fs>(path: &path, _extension: &extension, fs: &mut Fs) -> IoResult<Self> where Fs: FsRead {
        fs.read_str(path)
    }
}



#[cfg(feature = "rc")]
impl_io_save!(
    Rc<T>, RcWeak<T>,
    Arc<T>, ArcWeak<T>,
);

impl<T> Save for Cell<T> where T: Save + Copy {}
impl<T> Load for Cell<T> where T: Load + Copy {}


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

impl<T: Save> Save for Saturating<T> {
}

impl<T: Load> Load for Saturating<T> where for<'de> Saturating<T>: serde::Deserialize<'de> {
}

impl<T, const N : usize> Save for [T; N] where [T; N] : Serialize {} //, T : Save
impl<T, const N : usize> Load for [T; N] where Self : for<'de> Deserialize<'de> {}


#[cfg_attr(docsrs, doc(fake_variadic))]
#[cfg_attr(
    docsrs,
    doc = "This trait is implemented for tuples up to 16 items long."
)]
impl<T> Save for (T,) where T: Save {}
impl<T> Load for (T,) where T: Load {}

macro_rules! tuple_impls {
    // Each line provides a count and a list of index-type pairs
    (
        $(
            $len:literal => ( $( $idx:tt $typ:ident )+ )
        )*
    ) => {
        $(
            impl<$( $typ: Save ),+> Save for ( $( $typ ),+ ) {
                        }

            impl<$( $typ: Load ),+> Load for ( $( $typ ),+ ) {
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