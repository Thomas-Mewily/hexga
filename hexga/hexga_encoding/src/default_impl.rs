use std::cell::{Cell, RefCell};
use std::cmp::Reverse;
// do delegation on Serde serialize https://docs.rs/serde/latest/serde/trait.Serialize.html#impl-Serialize-for-str
use std::collections::*;
use std::hash::{BuildHasher, Hash};
use std::marker::PhantomData;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::num::*;
use std::ops::*;
#[allow(unused_imports)]
use std::rc::{Rc, Weak as RcWeak};
#[allow(unused_imports)]
use std::sync::{Arc, Mutex, RwLock, Weak as ArcWeak};

use super::*;

macro_rules! impl_load_and_save {
    (
        $(
            $name:ident $(<$( $generic:ident ),+>)?
        ),* $(,)?
    ) => {
        $(
            impl$(<$( $generic: Save ),+>)? SaveExtension for $name$(<$( $generic ),+>)?
            {
            }

            impl$(<$( $generic: Load ),+>)? LoadExtension for $name$(<$( $generic ),+>)?
            {
            }
        )*
    };
}

type Void = ();

impl_load_and_save!(
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


impl<K,V,S> SaveExtension for HashMap<K,V,S> where K: SaveExtension + Eq + Hash, V: SaveExtension, S: BuildHasher + Default {}
impl<K,V,S> LoadExtension for HashMap<K,V,S> where K: LoadExtension + Eq + Hash, V: LoadExtension, S: BuildHasher + Default {}

impl<K,  S> SaveExtension for HashSet<K,  S> where K: SaveExtension + Eq + Hash, S: BuildHasher + Default {}
impl<K,  S> LoadExtension for HashSet<K,  S> where K: LoadExtension + Eq + Hash, S: BuildHasher + Default {}

impl<K,V> SaveExtension for BTreeMap<K,V> where K: SaveExtension + Ord, V: SaveExtension {}
impl<K,V> LoadExtension for BTreeMap<K,V> where K: LoadExtension + Ord, V: LoadExtension {}

impl<K> SaveExtension for BTreeSet<K> where K: SaveExtension + Ord  {}
impl<K> LoadExtension for BTreeSet<K> where K: LoadExtension + Ord  {}

impl<T> SaveExtension for BinaryHeap<T> where T: SaveExtension + Ord  {}
impl<T> LoadExtension for BinaryHeap<T> where T: LoadExtension + Ord  {}


impl<T> SaveExtension for &[T] where T: SaveExtension {}

impl SaveExtension for String
{
    fn save_custom_extensions() -> impl Iterator<Item = &'static extension> {
        ["txt", "md", "cvs"].into_iter()
    }

    fn save_to_writer_with_custom_extension<W>(&self, writer: W, extension: &extension) -> EncodeResult where W: Write {
        self.as_str().save_to_writer_with_custom_extension(writer, extension)
    }
}
impl LoadExtension for String
{
    fn load_custom_extensions() -> impl Iterator<Item = &'static extension> {
        Self::save_custom_extensions()
    }

    fn load_from_reader_with_custom_extension<R>(mut reader: R, _extension: &extension) -> EncodeResult<Self> where Self: Sized, R: Read {
        let mut buf = Vec::with_capacity(16);
        reader.read_to_end(&mut buf)?;

        match std::str::from_utf8(&buf)
        {
            Ok(s) => Ok(s.to_owned()),
            Err(e) => Err(e.into()),
        }
    }
}
impl<'a> SaveExtension for &'a str
{
    fn save_custom_extensions() -> impl Iterator<Item = &'static extension> {
        String::save_custom_extensions()
    }

    fn save_to_writer_with_custom_extension<W>(&self, mut writer: W, _extension: &extension) -> EncodeResult where W: Write {
        writer.write(self.as_bytes())?;
        Ok(())
    }
}



#[cfg(feature = "rc")]
impl_load_and_save!(
    Rc<T>, RcWeak<T>,
    Arc<T>, ArcWeak<T>,
);

impl<T> SaveExtension for Cell<T> where T: SaveExtension + Copy {}
impl<T> LoadExtension for Cell<T> where T: LoadExtension + Copy {}

// https://docs.rs/serde/latest/serde/trait.Serialize.html#impl-Serialize-for-str
impl_load_and_save!(
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

impl<T: SaveExtension> SaveExtension for Saturating<T> {
}

impl<T: LoadExtension> LoadExtension for Saturating<T> where for<'de> Saturating<T>: CfgDeserialize<'de> {
}

impl<T, const N: usize> SaveExtension for [T; N] where [T; N]: CfgSerialize {}
impl<T, const N: usize> LoadExtension for [T; N] where Self: for<'de> CfgDeserialize<'de> {}


#[cfg_attr(docsrs, doc(fake_variadic))]
#[cfg_attr(
    docsrs,
    doc = "This trait is implemented for tuples up to 16 items long."
)]
impl<T> SaveExtension for (T,) where T: SaveExtension {}
impl<T> LoadExtension for (T,) where T: LoadExtension {}

macro_rules! tuple_impls {
    // Each line provides a count and a list of index-type pairs
    (
        $(
            $len:literal => ( $( $idx:tt $typ:ident )+ )
        )*
    ) => {
        $(
            impl<$( $typ: SaveExtension ),+> SaveExtension for ( $( $typ ),+ ) {
                        }

            impl<$( $typ: LoadExtension ),+> LoadExtension for ( $( $typ ),+ ) {
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