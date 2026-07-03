use std::borrow::Cow::Owned;
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

impl<K, V, S> Save for HashMap<K, V, S>
where
    K: Save + Eq + Hash,
    V: Save,
    S: BuildHasher + Default,
{
}
impl<K, V, S> Load for HashMap<K, V, S>
where
    K: Load + Eq + Hash,
    V: Load,
    S: BuildHasher + Default,
{
}

impl<K, S> Save for HashSet<K, S>
where
    K: Save + Eq + Hash,
    S: BuildHasher + Default,
{
}
impl<K, S> Load for HashSet<K, S>
where
    K: Load + Eq + Hash,
    S: BuildHasher + Default,
{
}

impl<K, V> Save for BTreeMap<K, V>
where
    K: Save + Ord,
    V: Save,
{
}
impl<K, V> Load for BTreeMap<K, V>
where
    K: Load + Ord,
    V: Load,
{
}

impl<K> Save for BTreeSet<K> where K: Save + Ord {}
impl<K> Load for BTreeSet<K> where K: Load + Ord {}

impl<T> Save for BinaryHeap<T> where T: Save + Ord {}
impl<T> Load for BinaryHeap<T> where T: Load + Ord {}

impl<T> Save for &[T] where T: Save {}

impl Save for String
{
    fn save_custom_extensions() -> impl Iterator<Item = &'static extension> { ["txt", "md", "cvs"].into_iter() }

    fn save_to_writer_with_custom_extension<'ext, W>(&self, writer: W, extension: Option<&'ext extension>) -> EncodeResult<DeducedExtension<'ext>>
    where
        W: Write,
    {
        self.as_str().save_to_writer_with_custom_extension(writer, extension)
    }
}
impl Load for String
{
    fn load_custom_extensions() -> impl Iterator<Item = &'static extension> { Self::save_custom_extensions() }

    fn load_from_reader_with_custom_extension<R>(mut reader: R, _extension: Option<&extension>) -> EncodeResult<Self>
    where
        Self: Sized,
        R: Read,
    {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        reader.read_to_end(&mut buf)?;

        match std::str::from_utf8(&buf)
        {
            Ok(s) => Ok(s.to_owned()),
            Err(e) => Err(e.into()),
        }
    }
}
impl<'a> Save for &'a str
{
    fn save_custom_extensions() -> impl Iterator<Item = &'static extension> { String::save_custom_extensions() }

    fn save_to_writer_with_custom_extension<'ext, W>(&self, mut writer: W, extension: Option<&'ext extension>) -> EncodeResult<DeducedExtension<'ext>>
    where
        W: Write,
    {
        writer.write(self.as_bytes())?;
        match extension
        {
            Some(ex) => Ok(DeducedExtension::Borrowed(ex)),
            None => Ok(Owned("txt".to_owned())),
        }
    }
}

#[cfg(feature = "serde_rc")]
impl_load_and_save!(Rc<T>, RcWeak<T>, Arc<T>, ArcWeak<T>,);

impl<T> Save for Cell<T> where T: Save + Copy {}
impl<T> Load for Cell<T> where T: Load + Copy {}

// https://docs.rs/serde/latest/serde/trait.Serialize.html#impl-Serialize-for-str
impl_load_and_save!(
    IpAddr,
    Ipv4Addr,
    Ipv6Addr,
    SocketAddr,
    Range<Idx>,
    RangeFrom<Idx>,
    RangeInclusive<Idx>,
    RangeTo<Idx>,
    Bound<T>,
    RefCell<T>,
    Reverse<T>,
    PhantomData<T>,
    Wrapping<T>,
    Mutex<T>,
    RwLock<T>,
);

impl<T: Save> Save for Saturating<T> {}

impl<T: Load> Load for Saturating<T> where for<'de> Saturating<T>: CfgDeserialize<'de> {}

impl<T, const N: usize> Save for [T; N] where [T; N]: CfgSerialize {}
impl<T, const N: usize> Load for [T; N] where Self: for<'de> CfgDeserialize<'de> {}

map_on_tuple!(
    (
        $(
            $len:literal => ( $( $idx:tt $typ:ident )+ )
        )*
    ) => {
        $(
            #[cfg_attr(docsrs, doc(fake_variadic))]
            impl<$( $typ: Save ),+> Save for ( $( $typ ),+ ,) { }
            #[cfg_attr(docsrs, doc(fake_variadic))]
            impl<$( $typ: Load ),+> Load for ( $( $typ ),+ ,) { }
        )*
    };
);
