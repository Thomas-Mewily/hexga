use super::*;

pub type Grid1<T> = Grid<T, 1>;
pub type Grid2<T> = Grid<T, 2>;
pub type Grid3<T> = Grid<T, 3>;
pub type Grid4<T> = Grid<T, 4>;

pub type Grid<T, const N : usize> = GridBase<T,int,N>;

pub type GridError<const N : usize> = grid::GridBaseError<int,N>;
pub type Iter<'a, T, const N : usize>  = grid::Iter<'a, T, int,N>;
pub type IterMut<'a, T, const N : usize>  = grid::IterMut<'a, T, int,N>;
pub type IntoIter<T, const N : usize>  = grid::IntoIter<T, int,N>;

pub use super::{GridView,GridViewMut,IGridView,IGridViewMut};