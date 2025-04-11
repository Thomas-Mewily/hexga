use crate::*;

/*
/// ParameterizedGrid
pub struct ParamGrid<T, const N : usize, Param=()>
{
    pub(crate) grid    : Grid<T,N>,
    pub(crate) param   : Param,
}

impl<T,const N : usize,Param> ParamGrid<T,N,Param>
{
    pub fn new_with_param(grid : Grid<T,N>, param : Param) -> Self { Self { grid, param }}
    pub fn new(grid : Grid<T,N>) -> Self where Param : Default { Self::new_with_param(grid, Param::default()) }
}

 */