use super::*;

pub type GridParam1<T,Param> = GridParam<T, Param, 1>;
pub type GridParam2<T,Param> = GridParam<T, Param, 2>;
pub type GridParam3<T,Param> = GridParam<T, Param, 3>;
pub type GridParam4<T,Param> = GridParam<T, Param, 4>;

pub type GridParam<T, Param, const N : usize> = GridParamBase<T,Param,int,N>;

pub use super::{GridParamView,GridParamViewMut};