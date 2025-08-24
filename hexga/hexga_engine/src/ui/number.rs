use super::*;

pub type UiNumberRelative = UiNumberRelativeOf<float>;

#[derive(Debug, Clone, Copy, PartialEq, Default, PartialOrd)]
pub struct UiNumberRelativeOf<T> where T:Number
{
    /// Coef based on the minimal axis
    min : T,
    /// Coef based on the maximal axis
    max : T,

    /// Coef based on the current axis
    axis : T,
}
impl<T> Zero for UiNumberRelativeOf<T> where T:Number { const ZERO : Self = Self { min: T::ZERO, max: T::ZERO, axis: T::ZERO }; }
impl<T> One for UiNumberRelativeOf<T> where T:Number { const ONE : Self = Self { min: T::ONE, max: T::ONE, axis: T::ONE }; }
impl<T> MinusOne for UiNumberRelativeOf<T> where T:Number+MinusOne { const MINUS_ONE : Self = Self { min: T::MINUS_ONE, max: T::MINUS_ONE, axis: T::MINUS_ONE }; }
impl<T> Half for UiNumberRelativeOf<T> where T:Number+Half { const HALF : Self = Self { min: T::HALF, max: T::HALF, axis: T::HALF }; }

impl<T> UiNumberRelativeOf<T> where T:Number
{
    const AXIS : Self = Self { min: T::ZERO, max: T::ZERO, axis: T::ONE };

    fn for_window<P>(self) -> UiNumberOf<P,T> where P:Number+Default
    {
        UiNumberOf
        {
            window : self,
            .. zero()
        }
    }

    fn for_screen<P>(self) -> UiNumberOf<P,T> where P:Number+Default
    {
        UiNumberOf
        {
            screen : self,
            .. zero()
        }
    }
}

map_on_operator_binary_arithmetic!(
    (($trait_name: tt, $fn_name: tt)) => 
    {
        impl<T> std::ops::$trait_name for UiNumberRelativeOf<T> where T: Number + std::ops::$trait_name 
        {
            type Output = Self;
            fn $fn_name(self, rhs : Self) -> Self::Output 
            {
                Self
                {
                    min: self.min.$fn_name(rhs.min),
                    max: self.max.$fn_name(rhs.max),
                    axis: self.axis.$fn_name(rhs.axis),
                }
            }
        }
    }
);
map_on_operator_assign_arithmetic!
(
    (($trait_name: tt, $fn_name: tt)) => 
    {
        impl<T> std::ops::$trait_name for UiNumberRelativeOf<T> where T: Number + std::ops::$trait_name 
        {
            fn $fn_name(&mut self, rhs : Self)
            {
                self.min.$fn_name(rhs.min);
                self.max.$fn_name(rhs.max);
                self.axis.$fn_name(rhs.axis);
            }
        }
    }
);
impl<T> Sum for UiNumberRelativeOf<T> where T: Number
{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { iter.fold(Self::ZERO, Self::add) }
}
impl<T> Product for UiNumberRelativeOf<T> where T:Number
{
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { iter.fold(Self::ONE, Self::mul) }
}



impl<T> UiNumberRelativeOf<T> where T: Number
{
    pub fn new() -> Self where T:Default { ___() }
}


pub trait ToUiNumberRelative<T> where T:Number
{
    fn ui_min(self) -> UiNumberRelativeOf<T>;
    fn ui_max(self) -> UiNumberRelativeOf<T>;
    fn ui_axis(self) -> UiNumberRelativeOf<T>;
}
impl<T> ToUiNumberRelative<T> for T where T:Number
{
    fn ui_min(self) -> UiNumberRelativeOf<T> {
        UiNumberRelativeOf{ min: self, .. zero() }
    }

    fn ui_max(self) -> UiNumberRelativeOf<T> {
        UiNumberRelativeOf{ max: self, .. zero() }
    }

    fn ui_axis(self) -> UiNumberRelativeOf<T> {
        UiNumberRelativeOf{ axis: self, .. zero() }
    }
}



pub type UiNumber = UiNumberOf<int,float>;


#[derive(Debug, Clone, Copy, PartialEq, Default, PartialOrd)]
pub struct UiNumberOf<P,T> where P: Number, T: Number
{
    pub pixel : P,
    pub window : UiNumberRelativeOf<T>,
    pub screen : UiNumberRelativeOf<T>,
}


impl<P,T> Zero for UiNumberOf<P,T> where P: Number, T: Number { const ZERO : Self = Self { pixel : zero(), window : zero(), screen : zero() }; }
impl<P,T> One for UiNumberOf<P,T> where P: Number,T:Number { const ONE : Self = Self { pixel : one(), window : one(), screen : one() }; }
impl<P,T> MinusOne for UiNumberOf<P,T> where P: Number+MinusOne,T:Number+MinusOne { const MINUS_ONE : Self = Self { pixel : minus_one(), window : minus_one(), screen : minus_one() }; }
impl<P,T> Half for UiNumberOf<P,T> where P: Number+Half,T:Number+Half { const HALF : Self = Self { pixel : half(), window : half(), screen : half() }; }

impl<P,T> UiNumberOf<P,T> where P: Number, T: Number
{
    fn new() -> Self where P: Default, T: Default { ___() }
}

pub trait UiConstant
{
    const ONE_PIXEL : Self;
    const FULL_WINDOW : Self;
    const FULL_SCREEN : Self;
}

impl<P,T> UiConstant for UiNumberOf<P,T> where P: Number, T: Number
{
    const ONE_PIXEL : Self = Self { pixel: P::ONE, window: zero(), screen: zero() };
    const FULL_WINDOW : Self = Self { pixel: zero(), window: UiNumberRelativeOf::<T>::AXIS, screen: zero() };
    const FULL_SCREEN : Self = Self { pixel: zero(), window: zero(), screen: UiNumberRelativeOf::<T>::AXIS };
}

map_on_operator_binary_arithmetic!(
    (($trait_name: tt, $fn_name: tt)) => 
    {
        impl<P,T> std::ops::$trait_name for UiNumberOf<P,T> where P: Number + std::ops::$trait_name, T: Number, UiNumberRelativeOf<T>: std::ops::$trait_name<UiNumberRelativeOf<T>,Output=UiNumberRelativeOf<T>>
        {
            type Output = Self;
            fn $fn_name(self, rhs : Self) -> Self::Output 
            {
                Self
                {
                    pixel: self.pixel.$fn_name(rhs.pixel),
                    window: self.window.$fn_name(rhs.window),
                    screen: self.screen.$fn_name(rhs.screen),
                }
            }
        }
    }
);
map_on_operator_assign_arithmetic!
(
    (($trait_name: tt, $fn_name: tt)) => 
    {
        impl<P,T> std::ops::$trait_name for UiNumberOf<P,T> where P: Number + std::ops::$trait_name, T: Number, UiNumberRelativeOf<T>: std::ops::$trait_name<UiNumberRelativeOf<T>>
        {
            fn $fn_name(&mut self, rhs : Self)
            {
                self.pixel.$fn_name(rhs.pixel);
                self.window.$fn_name(rhs.window);
                self.screen.$fn_name(rhs.screen);
            }
        }
    }
);
impl<P,T> Sum for UiNumberOf<P,T> where P: Number, T: Number
{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { iter.fold(Self::ZERO, Self::add) }
}
impl<P,T> Product for UiNumberOf<P,T> where P: Number, T:Number
{
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self { iter.fold(Self::ONE, Self::mul) }
}

pub type UiVectorOf<P,T,const N:usize> = Vector<UiNumberOf<P,T>,N>;
pub type UiRectangleOf<P,T,const N:usize> = Rectangle<UiNumberOf<P,T>,N>;

pub type UiVector<const N:usize> = Vector<UiNumber,N>;
pub type UiRectangle<const N:usize> = Rectangle<UiNumber,N>;

pub type UiVec2 = UiVector<2>;
pub type UiRect2 = UiRectangle<2>;



impl<P,T, const N:usize> UiConstant for [UiNumberOf<P,T>;N] where P:Number, T:Number
{
    const ONE_PIXEL   : Self = [UiNumberOf::ONE_PIXEL;  N];
    const FULL_WINDOW : Self = [UiNumberOf::FULL_WINDOW;N];
    const FULL_SCREEN : Self = [UiNumberOf::FULL_SCREEN;N];
}

impl<P,T, const N:usize> UiConstant for UiVectorOf<P,T,N> where P:Number, T:Number
{
    const ONE_PIXEL : Self  = Self::from_array(<[UiNumberOf<P,T>;N]>::ONE_PIXEL);
    const FULL_WINDOW : Self = Self::from_array(<[UiNumberOf<P,T>;N]>::FULL_WINDOW);
    const FULL_SCREEN : Self = Self::from_array(<[UiNumberOf<P,T>;N]>::FULL_SCREEN);
}
impl<P,T, const N:usize> UiConstant for UiRectangleOf<P,T,N> where P:Number, T:Number
{
    const ONE_PIXEL : Self  = Self::new(UiVectorOf::ZERO, UiVectorOf::ONE_PIXEL);
    const FULL_WINDOW : Self = Self::new(UiVectorOf::ZERO, UiVectorOf::FULL_WINDOW);
    const FULL_SCREEN : Self = Self::new(UiVectorOf::ZERO, UiVectorOf::FULL_SCREEN);
}


pub trait UiRectangleExtension
{
    fn centered(self) -> Self;
}
impl<P,T, const N:usize> UiRectangleExtension for UiRectangleOf<P,T,N> where P:Number, T:Number
{
    fn centered(mut self) -> Self {
        self.pos = UiVectorOf::FULL_SCREEN - self.size * UiNumberOf::one() / UiNumberOf::two();
        self
    }
}

pub mod prelude
{
    pub use super::{UiNumber,UiVec2,UiRect2,UiConstant,UiRectangleExtension};
}