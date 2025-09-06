pub mod prelude
{
    pub use super::{Composite,CompositeGeneric};
}

pub trait Composite
{   
    type Inside;
    fn transform<F>(self, f: F) -> Self where F: FnMut(Self::Inside) -> Self::Inside;
}
// Can't auto impl Composte for every S that have GenericComposite, because can't express that Self::WithType<Self::Inside> = Self
pub trait CompositeGeneric
{
    type WithType<T2> : CompositeGeneric<Inside = T2>;
    type Inside;
    fn transform<T2,F>(self, f: F) -> Self::WithType<T2> where F: FnMut(Self::Inside) -> T2;
}


impl<T, const N:usize> Composite for [T;N]
{
    type Inside=T;
    fn transform<F>(self, f: F) -> Self where F: FnMut(Self::Inside) -> Self::Inside { self.map(f) }
}
impl<T, const N:usize> CompositeGeneric for [T;N]
{
    type WithType<T2> = [T2;N];
    type Inside = T;
    fn transform<T2,F>(self, f: F) -> Self::WithType<T2> where F: FnMut(Self::Inside) -> T2 { self.map(f) }
}


impl<T> Composite for Vec<T>
{
    type Inside=T;
    fn transform<F>(mut self, mut f: F) -> Self where F: FnMut(Self::Inside) -> Self::Inside { self.iter_mut().map(|v| *v = f(*v)); self }
}
impl<T> CompositeGeneric for Vec<T>
{
    type WithType<T2> = Vec<T2>;
    type Inside = T;

    fn transform<T2,F>(self, mut f: F) -> Self::WithType<T2> where F: FnMut(Self::Inside) -> T2 {
        self.into_iter().map(|v| f(v)).collect()
    }
}



/*
macro_rules! impl_composite_types_and_methods_and_constants_for_external_type
{
    ($trait_name:ident $( < $($generic_params:tt),* > )?,
        { $( $type_name:ident ),* },
        { $( ($method_name:ident, $output_name:path) ),* },
        { $( $constant_name:ident ),* }
    ) =>
    {
        impl<T, const N: usize $(, $($generic_params),* )?> $trait_name $(< $($generic_params),* >)? for [T; N]
        where
            T: $trait_name $(< $($generic_params),* >)?
        {
            $(
                type $type_name = [T::$type_name; N];
            )*

            $(
                fn $method_name(self) -> $output_name
                {
                    self.map(|v| v.$method_name())
                }
            )*

            $(
                const $constant_name: Self = [T::$constant_name; N];
            )*
        }

        /* // Vector don't support constant
        impl<T $(, $($generic_params),* )?> $trait_name $(< $($generic_params),* >)? for Vec<T>
        where
            T: $trait_name $(< $($generic_params),* >)?
        {
            $(
                type $type_name = Vec<T::$type_name>;
            )*

            $(
                fn $method_name(self) -> $output_name
                {
                    // Todo
                }
            )*

            $(
                const $constant_name: Self = [T::$constant_name; N];
            )*
        }
        */
    };
}
pub(crate) use impl_composite_types_and_methods_and_constants_for_external_type;

macro_rules! impl_composite_types_and_methods_and_constants_for_internal_type
{
    ($trait_name:ident $( < $($generic_params:tt),* > )?,
        { $( $type_name:ident ),* },
        { $( ($method_name:ident, $output_name:path) ),* },
        { $( $constant_name:ident ),* }
    ) =>
    {
        impl<T, const N: usize $(, $($generic_params),* )?> $trait_name $(< $($generic_params),* >)? for Vector<T,N>
        where
            T: $trait_name $(< $($generic_params),* >)?
        {
            $(
                type $type_name = Vector<T::$type_name, N>;
            )*

            $(
                fn $method_name(self) -> $output_name
                {
                    self.map(|v| v.$method_name())
                }
            )*

            $(
                const $constant_name: Self = Vector::from_array([T::$constant_name; N]);
            )*
        }

        impl<T $(, $($generic_params),* )?> $trait_name $(< $($generic_params),* >)? for $crate::rectangle::RectangleOf<T>
            where
            T : $trait_name $(< $($generic_params),* >)?
        {
            $(
                type $type_name = $crate::rectangle::RectangleOf<T::$type_name>;
            )*

            $(
                fn $method_name(self) -> $output_name
                {
                    self.map(|v| v.$method_name())
                }
            )*

            $(
                const $constant_name: Self = $crate::rectangle::RectangleOf::new(T::$constant_name, T::$constant_name);
            )*
        }
    };
}
pub(crate) use impl_composite_types_and_methods_and_constants_for_internal_type;


macro_rules! impl_composite_types_and_methods_and_constants
{
    ($trait_name:ident $( < $($generic_params:tt),* > )?,
        { $( $type_name:ident ),* },
        { $( ($method_name:ident, $output_name:path) ),* },
        { $( $constant_name:ident ),* }
    ) =>
    {
        impl_composite_types_and_methods_and_constants_for_internal_type!(
            $trait_name $(< $($generic_params),* >)?,
            { $( $type_name ),* },
            { $( ($method_name, $output_name) ),* },
            { $( $constant_name ),* }
        );
        impl_composite_types_and_methods_and_constants_for_external_type!(
            $trait_name $(< $($generic_params),* >)?,
            { $( $type_name ),* },
            { $( ($method_name, $output_name) ),* },
            { $( $constant_name ),* }
        );
    };
}
pub(crate) use impl_composite_types_and_methods_and_constants;

/// To impl a trait that only expose constant value
macro_rules! impl_composite_constant_for_internal_type {
    ($trait_name:ident $( < $($generic_params:tt),* > )?, $( $constant_name:ident ),+ ) => {
        impl_composite_types_and_methods_and_constants_for_internal_type!($trait_name $(< $($generic_params),* >)?, { }, {}, { $( $constant_name ),+ });
    };
}
pub(crate) use impl_composite_constant_for_internal_type;

#[allow(unused)]
/// To impl a trait that only expose method and an associate Self::Output type
macro_rules! impl_composite_output_with_methods_for_internal_type {
    ($trait_name:ident $( < $($generic_params:tt),* > )?, $( $method_name:ident ),+ ) => {
        impl_composite_types_and_methods_and_constants_for_internal_type!($trait_name $(< $($generic_params),* >)?, { Output }, { $( ($method_name, Self::Output) ),+ }, { });
    };
}
pub(crate) use impl_composite_output_with_methods_for_internal_type;



/// To impl a trait that only expose constant value
macro_rules! impl_composite_constant {
    ($trait_name:ident $( < $($generic_params:tt),* > )?, $( $constant_name:ident ),+ ) => {
        impl_composite_types_and_methods_and_constants!($trait_name $(< $($generic_params),* >)?, { }, {}, { $( $constant_name ),+ });
    };
}
pub(crate) use impl_composite_constant;

/// To impl a trait that only expose method and an associate Self::Output type
macro_rules! impl_composite_output_with_methods {
    ($trait_name:ident $( < $($generic_params:tt),* > )?, $( $method_name:ident ),+ ) => {
        impl_composite_types_and_methods_and_constants!($trait_name $(< $($generic_params),* >)?, { Output }, { $( ($method_name, Self::Output) ),+ }, { });
    };
}
pub(crate) use impl_composite_output_with_methods;

/*
/// To impl a trait that only expose method with a custom non associate output type
macro_rules! impl_composite_with_methods {
    ($trait_name:ident $( < $($generic_params:tt),* > )?, $( ($method_name:ident, $output_name:path) ),+ ) => {
        impl_composite_types_and_methods_and_constants!($trait_name $(< $($generic_params),* >)?, {}, { $( ($method_name, $output_name) ),+ }, { });
    };
}
pub(crate) use impl_composite_with_methods;
*/
*/