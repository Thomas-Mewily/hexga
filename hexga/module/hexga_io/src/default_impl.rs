use crate::*;

macro_rules! impl_io_save {
    (
        $(
            $name:ident $(<$( $generic:ident ),+>)?
        ),* $(,)?
    ) => {
        $(
            impl$(<$( $generic: IoSave ),+>)? IoSave for $name$(<$( $generic ),+>)? {
                type BasedOn = IoNotBasedOn;
            }
        )*
    };
}


impl_io_save!(
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64,
    bool,
    char,
    Option<T>,
);

impl IoSave for String
{
    type BasedOn=str;
    fn save_from_based_on_ref(&self) -> Option<&Self::BasedOn> {
        Some(self.as_str())
    }
}
impl IoSave for str
{
    type BasedOn = IoNotBasedOn;
    fn save_own_extensions() -> impl Iterator<Item = &'static str> { ["txt", "md"].iter().copied() }

    fn save_to_with_own_extension_pathless<W, Fs>(&self, extension : &extension, w : W, fs : &mut Fs) -> IoResult
            where W : Write, Fs : IoFs {
                w.write_all(self.as_bytes()).map_err(|e| IoErrorKind::)
    }
}



/*
impl_io_save!(
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64,
    bool,
    char, String
);

impl<T> IoSave for Option<T> where T : IoSave
{
    type BasedOn = IoNotBasedOn;
}
*/