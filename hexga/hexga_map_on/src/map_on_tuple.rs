/// A powerful macro to impl other macros for tuple, up to 16 element (may change).
///
/// Can be used to impl trait to a lot of tuple using macro, where generic can't.
///
/// # Examples
///
/// Using an existing macro:
/// ```rust
/// use hexga_map_on::map_on_tuple;
///
/// trait Foo { fn foo(); }
///
/// impl Foo for i32 { fn foo() { println!("foo from i32"); } }
/// impl Foo for bool { fn foo() { println!("foo from bool"); } }
///
/// macro_rules! impl_foo {
///     ( $( $len:literal => ( $( $idx:tt $typ:ident )+ ) )* ) => {
///         $(
///             #[cfg_attr(docsrs, doc(fake_variadic))]
///             impl<$( $typ: Foo ),+> Foo for ( $( $typ ),+ ,) {
///                 fn foo() { println!("Foo from tuple size {}", $len); }
///             }
///         )*
///     };
/// }
///
/// map_on_tuple!(impl_foo);
/// ```
///
/// Using inline definition:
/// ```rust
/// use hexga_map_on::map_on_tuple;
///
/// trait Foo { fn foo(); }
///
/// impl Foo for i32 { fn foo() { println!("foo from i32"); } }
/// impl Foo for bool { fn foo() { println!("foo from bool"); } }
///
/// map_on_tuple!(
///     ( $( $len:literal => ( $( $idx:tt $typ:ident )+ ) )* ) => {
///         $(
///             #[cfg_attr(docsrs, doc(fake_variadic))]
///             impl<$( $typ: Foo ),+> Foo for ( $( $typ ),+ ,) {
///                 fn foo() { println!("Foo from tuple size {}", $len); }
///             }
///         )*
///     };
/// );
/// ```
#[macro_export]
macro_rules! map_on_tuple {
    // Case 1: Use an existing macro name
    ($macro_name:ident) => {
        $macro_name! {
            1 => (0 T0)
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
    };

    // Case 2: Inline macro definition
    ( $($macro_arms:tt)+ ) => {
        const _: () = {
            macro_rules! __map_on_tuple_inliner {
                $($macro_arms)+
            }

            $crate::map_on_tuple!(__map_on_tuple_inliner);
        };
    };
}