use crate::*;

const X_INDEX : usize = 0;
const Y_INDEX : usize = 1;
const Z_INDEX : usize = 2;
const W_INDEX : usize = 3;

impl<T,const N : usize> Vector<T,N>
{
    pub fn min(self, other : Self) -> Self where T: PartialOrd { self.map_with(other, |a, b| a.min_partial(b)).into() }
    pub fn max(self, other : Self) -> Self where T: PartialOrd { self.map_with(other, |a, b| a.max_partial(b)).into() }

    pub fn have_x(&self) -> bool { self.is_index_valid(X_INDEX) }
    pub fn have_y(&self) -> bool { self.is_index_valid(Y_INDEX) }
    pub fn have_z(&self) -> bool { self.is_index_valid(Z_INDEX) }
    pub fn have_w(&self) -> bool { self.is_index_valid(W_INDEX) }
}

pub trait HaveX<T> : Sized
{
    const X_INDEX : usize = X_INDEX;
    fn iter_x<'a>(&'a self) -> impl Iterator<Item=&'a T> where T: 'a;
    fn iter_x_mut<'a>(&'a mut self) -> impl Iterator<Item=&'a mut T> where T: 'a;
}
pub trait HaveXAndOne<T> : HaveX<T>
{
    const X : Self;

    /// `+ X`
    const RIGHT : Self = Self::X;
    /// `+ X`
    #[must_use]
    fn right(self) -> Self where Self : Add<Self,Output=Self> { self + Self::X }
    /// `- X`
    #[must_use]
    fn left (self) -> Self where Self : Sub<Self,Output=Self> { self - Self::X }
}
pub trait HaveXAndMinusOne<T> : HaveX<T>
{
    const MINUS_X : Self;
    /// `- X`
    const LEFT : Self = Self::MINUS_X;
}


pub trait HaveY<T> : HaveX<T>
{
    const Y_INDEX : usize = Y_INDEX;
    fn iter_xy<'a>(&'a self) -> impl Iterator<Item=&'a T> where T: 'a;
    fn iter_xy_mut<'a>(&'a mut self) -> impl Iterator<Item=&'a mut T> where T: 'a;
}
pub trait HaveYAndOne<T> : HaveY<T>
{
    const Y : Self;

    /// `+ Y`
    const UP : Self = Self::Y;
    /// `+ Y`
    #[must_use]
    fn up(self) -> Self where Self : Add<Self,Output=Self> { self + Self::Y }
    /// `- Y`
    #[must_use]
    fn down(self) -> Self where Self : Sub<Self,Output=Self> { self - Self::Y }
}
pub trait HaveYAndMinusOne<T> : HaveY<T>
{
    const MINUS_Y : Self;
    /// `- Y`
    const DOWN : Self = Self::MINUS_Y;
}


pub trait HaveZ<T> : HaveY<T>
{
    const Z_INDEX : usize = Z_INDEX;
    fn iter_xyz<'a>(&'a self) -> impl Iterator<Item=&'a T> where T: 'a;
    fn iter_xyz_mut<'a>(&'a mut self) -> impl Iterator<Item=&'a mut T> where T: 'a;
}
pub trait HaveZAndOne<T> : HaveZ<T>
{
    const Z : Self;

    /// `+ Z`
    const FORWARD : Self = Self::Z;
    /// `+ Z`
    #[must_use]
    fn forward(self) -> Self where Self : Add<Self,Output=Self> { self + Self::Z }
    /// `- Z`
    #[must_use]
    fn backward(self) -> Self where Self : Sub<Self,Output=Self> { self - Self::Z }
}
pub trait HaveZAndMinusOne<T> : HaveZ<T>
{
    const MINUS_Z : Self;
    /// `- Z`
    const BACKWARD : Self = Self::MINUS_Z;
}


pub trait HaveW<T> : HaveZ<T>
{
    const W_INDEX : usize = W_INDEX;
    fn iter_xyzw<'a>(&'a self) -> impl Iterator<Item=&'a T> where T: 'a;
    fn iter_xyzw_mut<'a>(&'a mut self) -> impl Iterator<Item=&'a mut T> where T: 'a;
}
pub trait HaveWAndOne<T> : HaveW<T>
{
    const W : Self;

    /// `+ W`
    const ANA : Self = Self::W;
    /// `+ W`
    #[must_use]
    fn ana(self) -> Self where Self : Add<Self,Output=Self> { self + Self::W }
    /// `- W`
    #[must_use]
    fn kata (self) -> Self where Self : Sub<Self,Output=Self> { self - Self::W }
}
pub trait HaveWAndMinusOne<T> : HaveW<T>
{
    const MINUS_W : Self;
    /// `- W`
    const KATA : Self = Self::MINUS_W;
}