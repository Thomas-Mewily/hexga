/// `i8`, `i16`, `i32`, `i64`, `isize`
#[macro_export]
macro_rules! map_on_integer_signed 
{
    ($mac:ident) => 
    {
        $mac!(i8   );
        $mac!(i16  );
        $mac!(i32  );
        $mac!(i64  );
        $mac!(isize);
    };

    ($mac:ident,     $($args:tt)*) => 
    {
        $mac!(i8   , $($args)*);
        $mac!(i16  , $($args)*);
        $mac!(i32  , $($args)*);
        $mac!(i64  , $($args)*);
        $mac!(isize, $($args)*);
    };
}

/// `u8`, `u16`, `u32`, `u64`, `usize`
#[macro_export]
macro_rules! map_on_integer_unsigned 
{
    ($mac:ident) => 
    {
        $mac!(u8   );
        $mac!(u16  );
        $mac!(u32  );
        $mac!(u64  );
        $mac!(usize);
    };

    ($mac:ident,     $($args:tt)*) => 
    {
        $mac!(u8   , $($args)*);
        $mac!(u16  , $($args)*);
        $mac!(u32  , $($args)*);
        $mac!(u64  , $($args)*);
        $mac!(usize, $($args)*);
    };
}

/// (`i8`, `i16`, `i32`, `i64`, `isize`) + (`u8`, `u16`, `u32`, `u64`, `usize`)
#[macro_export]
macro_rules! map_on_integer 
{
    ($mac:ident) => 
    {
        $crate::map_on_integer_unsigned!($mac);
        $crate::map_on_integer_signed!($mac);
    };

    ($mac:ident, $($args:tt)*) => 
    {
        $crate::map_on_integer_unsigned!($mac, $($args)*);
        $crate::map_on_integer_signed!($mac, $($args)*);
    };
}

/// `f32`, `f64`
#[macro_export]
macro_rules! map_on_float 
{
    ($mac:ident) => 
    {
        $mac!(f32);
        $mac!(f64);
    };

    ($mac:ident, $($args:tt)*) => 
    {
        $mac!(f32, $($args)*);
        $mac!(f64, $($args)*);
    };
}

/// (`i8`, `i16`, `i32`, `i64`, `isize`) + (`u8`, `u16`, `u32`, `u64`, `usize`) + (`f32`, `f64`)
#[macro_export]
macro_rules! map_on_number 
{
    ($mac:ident) => 
    {
        $crate::map_on_integer!($mac);
        $crate::map_on_float!($mac);
    };

    ($mac:ident, $($args:tt)*) => 
    {
        $crate::map_on_integer!($mac, $($args)*);
        $crate::map_on_float!($mac, $($args)*);
    };
}

/// (`i8`, `i16`, `i32`, `i64`, `isize`) + (`u8`, `u16`, `u32`, `u64`, `usize`) + (`f32`, `f64`) + (`bool`)
#[macro_export]
macro_rules! map_on_number_and_bool 
{
    ($mac:ident) => 
    {
        $crate::map_on_integer!($mac);
        $crate::map_on_float!($mac);
        $mac!(bool);
    };

    ($mac:ident, $($args:tt)*) => 
    {
        $crate::map_on_integer!($mac, $($args:tt)*);
        $crate::map_on_float!($mac, $($args:tt)*);
        $mac!(bool, $($args:tt)*);
    };
}