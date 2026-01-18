
/// A powerful macro to impl other macros for the given types.
///
/// Can be used to impl trait to a lot of type using macro, where generic can't.
///
/// ```rust
/// use hexga_map_on::map_on;
///
/// trait Zero
/// {
///     const ZERO : Self;
/// }
///
/// macro_rules! impl_zero {
///     ($type_name:ty) => {
///         impl Zero for $type_name
///         {
///             const ZERO : Self = 0 as Self;
///         }
///     };
/// }
///
/// map_on!
/// (
///     (
///         i8, i16, i32, i64, isize,
///         u8, u16, u32, u64, usize,
///         f32, f64
///     ),
///     impl_zero
/// );
///
/// // ^^ this call impl Zero for all the given type
///
/// assert_eq!(i32::ZERO  , 0);
/// assert_eq!(usize::ZERO, 0);
/// assert_eq!(f32::ZERO  , 0.);
/// ```
#[macro_export]
macro_rules! map_on {
    // Base case: single type
    ( ($type_name:tt), $mac:ident $(, $args:tt)* ) => {
        $mac!($type_name $(, $args)*);
    };
    // Recursive case: multiple types
    ( ($first_type:tt, $($rest_type:tt),+), $mac:ident $(, $args:tt)* ) => {
        $crate::map_on!(($first_type), $mac $(, $args)*);
        $crate::map_on!(($($rest_type),+), $mac $(, $args)*);
    };

    // Limitation :
    // Can only be used in const context (ex: impl Trait).
    // Ex: this **won't** work inside a function :
    // ```
    // map_on!((i32, f64, bool),
    //    ($T:ident) => {
    //        println!("Type: {}", std::any::type_name::<$T>());
    //    }
    // );
    // ```
    ($tokens:tt, $($macro_arms:tt)+) => {
        const _: () = {
            macro_rules! __map_on_inliner {
                $($macro_arms)+
            }

            $crate::map_on!(@expand_tokens $tokens);
        };
    };

    // Recursive expansion
    (@expand_tokens ($first:tt $(, $rest:tt)*)) => {
        __map_on_inliner!($first);
        $crate::map_on!(@expand_tokens ($($rest),*))
    };

    (@expand_tokens ($last:tt)) => {
        __map_on_inliner!($last);
    };

    (@expand_tokens ()) => {};


    // Entry point for list of pairs with inline macro arms
    ( ( $(($a:tt, $b:tt)),* $(,)? ), ($($params:tt)*) => $body:block ) => {
        const _: () = {
            macro_rules! __map_on_inliner {
                ($($params)*) => $body
            }

            $(
                __map_on_inliner!($a, $b);
            )*
        };
    };

    // Simple case: single identifiers
    ( ($($types:tt),+), $mac:ident $(, $args:tt)* ) => {
        $(
            $mac!($types $(, $args)*);
        )+
    };
}
