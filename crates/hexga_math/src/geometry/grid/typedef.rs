use super::*;

pub type Grid1<T> = Grid<T, 1>;
pub type Grid2<T> = Grid<T, 2>;
pub type Grid3<T> = Grid<T, 3>;
pub type Grid4<T> = Grid<T, 4>;

pub type Grid<T, const N: usize, A = AllocGlobal> = GridOf<T, int, N, A>;
pub type GridError<const N: usize> = super::GridBaseError<int, N>;
