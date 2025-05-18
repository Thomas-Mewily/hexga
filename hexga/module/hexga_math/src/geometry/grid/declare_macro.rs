use crate::*;

/* 
#[macro_export]
macro_rules! declare_grid {
    ($name:ident) => {
        pub struct $name<T, Idx, const N: usize>
        where
            Idx: $crate:IntegerIndex,
        {
            pub grid: hexga_math::geometry::grid::GridBase<T, Idx, N>,
        }

        impl<T, Idx, const N : usize> $name<T, Idx, N>
        {
            pub fn from_grid(grid: $name<T, Idx, N>) -> Self { Self { grid } }
        }

        impl<T, Idx, const N : usize> $name<T, Idx, N> where Idx : $crate:IntegerIndex
        {
            pub fn from_vec(size : Vector::<Idx,N>, value : Vec<T>) -> Option<Self> 
            { Self::from_grid(hexga_math::geometry::grid::GridBase::from_vec(size, value)) }
            pub fn try_from_vec(size : Vector::<Idx,N>, value : Vec<T>) -> Result<Self, hexga_math::geometry::grid::GridBaseError<Idx,N>> 
            { Self::from_grid(hexga_math::geometry::grid::GridBase::try_from_vec(size, value)) }

            /// Create a grid from a function
            pub fn from_fn<F>(size : Vector::<Idx,N>, mut f : F) -> Self where F : FnMut(Vector::<Idx,N>) -> T 
            { Self::from_grid(hexga_math::geometry::grid::GridBase::from_fn(size, f)) }

            /// Fill the grid with the [Default] value
            pub fn new(size : Vector::<Idx,N>) -> Self where T : Default 
            { Self::from_grid(hexga_math::geometry::grid::GridBase::new(size)) }

            /// Fill the grid by cloning the value
            pub fn new_uniform(size : Vector::<Idx,N>, value : T) -> Self where T : Clone 
            { Self::from_grid(hexga_math::geometry::grid::GridBase::new_uniform(size, value)) }

            /// Create a grid from a function in parallel
            pub fn from_fn_par<F>(size : Vector::<Idx,N>, f : F) -> Self where F : Fn(Vector::<Idx,N>) -> T + Sync, T : Send, Idx : Sync 
            { Self::from_grid(hexga_math::geometry::grid::GridBase::from_fn_par(size, f)) }

            /// Fill the grid with the [Default] value in parallel
            pub fn new_par(size : Vector::<Idx,N>) -> Self where T : Default + Send, Idx : Sync 
            { Self::from_grid(hexga_math::geometry::grid::GridBase::new_par(size)) }

            /// Fill the grid by cloning the value in parallel
            pub fn new_uniform_par(size : Vector::<Idx,N>, value : T) -> Self where T : Clone + Sync + Send, Idx : Sync 
             { Self::from_grid(hexga_math::geometry::grid::GridBase::new_uniform_par(size, value)) }
        }

        // Todo : Remove T: Clone constraint
        impl<T, Idx, const N : usize> Crop<Idx,N> for $name<T, Idx, N> where Idx : $crate:IntegerIndex, T : Clone
        {
            fn crop(self, subrect : Rectangle<Idx, N>) -> Option<Self>  { self.grid.crop(subrect).map(|g| Self::from_grid(g)) }
            unsafe fn crop_unchecked(self, subrect : Rectangle<Idx, N>) -> Self { Self::from_grid(unsafe { self.grid.crop_unchecked(subrect) }) }
        }

        impl<T, Idx, const N : usize> IGrid<T,(),Idx,N> for $name<T, Idx, N> where Idx : $crate:IntegerIndex,
        {
            fn values(&self) -> &[T] { self.grid.values() }
            fn values_mut(&mut self) -> &mut [T] { self.grid.values_mut() }

            fn into_values(self) -> Vec<T> { self.grid.into_values() }

            A
            fn transform<Dest, F>(self, f : F) -> <Self as IGridView<T,(),Idx,N>>::Map<Dest> where F : FnMut(T) -> Dest 
            { GridBase { size: self.size, value: self.value.into_iter().map(f).collect() } }
            fn transform_par<Dest, F>(self, f : F) -> <Self as IGridView<T,(),Idx,N>>::Map<Dest> where F : Fn(T) -> Dest + Sync + Send, T : Send + Sync, Dest : Send, Idx : Sync, () : Clone 
            { GridBase { size: self.size, value: self.value.into_par_iter().map(f).collect() } }
            
            //fn crop_margin(&self, margin_start : Vector<Idx,N>, margin_end : Vector<Idx,N>) -> Self where T : Clone { self.view().crop_margin(margin_start, margin_end).to_grid() }

            type View<'a> = grid::GridView<'a, T,Idx,N> where Self: 'a;
            fn view<'a>(&'a self) -> grid::GridView<'a, T,Idx,N> { grid::GridView::from_grid(self) }

            type ViewMut<'a> = grid::GridViewMut<'a, T,Idx,N> where Self: 'a;
            fn view_mut<'a>(&'a mut self) -> grid::GridViewMut<'a, T,Idx,N> { grid::GridViewMut::from_grid(self) }
        }

        impl<T, Idx, const N : usize> IGridView<T,(),Idx,N> for $name<T, Idx, N> where Idx : $crate:IntegerIndex 
        {

            type Map<Dest>=$name<Dest, Idx, N>;
            fn map<Dest, F>(&self, mut f : F) -> Self::Map<Dest> where F : FnMut(&T) -> Dest, () : Clone { GridBase::from_fn(self.size(), |p| f(&self[p])) }
            fn map_par<Dest, F>(&self, f : F) -> Self::Map<Dest> where F : Fn(&T) -> Dest + Sync, T : Send + Sync, Dest : Send, Idx : Sync, () : Clone  { GridBase::from_fn_par(self.size(), |p| f(&self[p])) }
        }

        impl<T, Idx, const N : usize> IRectangle<Idx, N> for $name<T, Idx, N> where Idx : $crate:IntegerIndex,
        {
            #[inline(always)]
            fn size(&self) -> Vector<Idx, N> { self.size }
            #[inline(always)]
            fn begin(&self) -> Vector<Idx,N> { zero() }

            fn iter_x(&self) -> Range<Idx> where Vector<Idx,N> : HaveX<Idx>, Range<Idx> : IntoIterator { Idx::ZERO..self.size_x() }
            fn iter_y(&self) -> Range<Idx> where Vector<Idx,N> : HaveY<Idx>, Range<Idx> : IntoIterator { Idx::ZERO..self.size_y() }
            fn iter_z(&self) -> Range<Idx> where Vector<Idx,N> : HaveZ<Idx>, Range<Idx> : IntoIterator { Idx::ZERO..self.size_z() }
            fn iter_w(&self) -> Range<Idx> where Vector<Idx,N> : HaveW<Idx>, Range<Idx> : IntoIterator { Idx::ZERO..self.size_w() }

            #[inline(always)] fn is_inside_x(&self, x : Idx) -> bool where Vector<Idx,N> : HaveX<Idx> { x >= Idx::ZERO && x < self.size_x() }
            #[inline(always)] fn is_inside_y(&self, y : Idx) -> bool where Vector<Idx,N> : HaveY<Idx> { y >= Idx::ZERO && y < self.size_y() }
            #[inline(always)] fn is_inside_z(&self, z : Idx) -> bool where Vector<Idx,N> : HaveZ<Idx> { z >= Idx::ZERO && z < self.size_z() }
            #[inline(always)] fn is_inside_w(&self, w : Idx) -> bool where Vector<Idx,N> : HaveW<Idx> { w >= Idx::ZERO && w < self.size_w() }
        }

        impl<T, Idx, const N : usize> IGridViewMut<T,(),Idx,N> for $name<T, Idx, N> 
            where Idx : $crate:IntegerIndex 
        {
            type SubViewMut<'b> = GridViewMut<'b,T,Idx,N> where Self: 'b;
            fn subview_mut<'a>(&'a mut self, rect : Rectangle<Idx, N>) -> Self::SubViewMut<'a> { GridViewMut::new_intersect(self, rect) }
        }

        impl<T, Idx, const N : usize> Get<Vector<Idx,N>> for $name<T, Idx,N>  where Idx : $crate:IntegerIndex 
        {
            type Output = <Self as Index<Vector<Idx,N>>>::Output;
            #[inline(always)]
            fn try_get(&self, pos : Vector<Idx,N>) -> Result<&Self::Output, ()> { self.get(pos).ok_or_void() }
            #[inline(always)]
            fn get(&self, pos : Vector<Idx,N>) -> Option<&Self::Output> { self.position_to_index(pos).and_then(|idx| self.get(idx)) }
            #[inline(always)]
            unsafe fn get_unchecked(&self, pos : Vector<Idx,N>) -> &Self::Output { unsafe { let idx = self.position_to_index_unchecked(pos); self.get_unchecked(idx) } }
        }

        impl<T, Idx, const N : usize> GetMut<Vector<Idx,N>> for $name<T, Idx,N> where Idx : $crate:IntegerIndex 
        {
            #[inline(always)]
            fn try_get_mut(&mut self, pos : Vector<Idx,N>) -> Result<&mut Self::Output, ()> { self.get_mut(pos).ok_or_void() }
            #[inline(always)]
            fn get_mut(&mut self, pos : Vector<Idx,N>) -> Option<&mut Self::Output> { self.position_to_index(pos).and_then(|i| self.get_mut(i)) }
            #[inline(always)]
            unsafe fn get_unchecked_mut(&mut self, pos : Vector<Idx,N>) -> &mut Self::Output{ unsafe { let idx = self.position_to_index_unchecked(pos); self.values_mut().get_unchecked_mut(idx)} }
        }

        impl<T, Idx, const N : usize> GetManyMut<Vector<Idx,N>> for $name<T, Idx,N> where Idx : $crate:IntegerIndex 
        {
            #[inline(always)]
            fn try_get_many_mut<const N2: usize>(&mut self, indices: [Vector<Idx,N>; N2]) -> Result<[&mut Self::Output;N2], ()> {
                // Use try_map https://doc.rust-lang.org/std/primitive.array.html#method.try_map when #stabilized
                let indices = indices.map(|pos| self.position_to_index(pos));
                if indices.any(|x| x.is_none()) 
                {
                    Err(())
                } else 
                {
                    self.try_get_many_mut(indices.map(|idx| idx.unwrap()))
                }
            }
        }

        impl<T, Idx, const N : usize> Get<usize> for $name<T, Idx,N> 
            where Idx : $crate:IntegerIndex 
        {
            type Output = <Self as Index<usize>>::Output;
            #[inline(always)]
            fn try_get(&self, index : usize) -> Result<&T, ()> { self.get(index).ok_or_void() }
            #[inline(always)]
            fn get(&self, index : usize) -> Option<&T> { self.values().get(index) }
            #[inline(always)]
            #[track_caller]
            unsafe fn get_unchecked(&self, index : usize) -> &T { unsafe { self.values().get_unchecked(index) } }
        }

        impl<T, Idx, const N : usize> GetMut<usize> for $name<T, Idx,N> where Idx : $crate:IntegerIndex 
        {
            #[inline(always)]
            fn try_get_mut(&mut self, index : usize) -> Result<&mut T, ()> { self.get_mut(index).ok_or_void() }
            #[inline(always)]
            fn get_mut(&mut self, index : usize) -> Option<&mut T> { self.values_mut().get_mut(index) }
            #[inline(always)]
            #[track_caller]
            unsafe fn get_unchecked_mut(&mut self, index : usize) -> &mut T{ unsafe { self.values_mut().get_unchecked_mut(index)} }
        }

        impl<T, Idx, const N : usize> GetManyMut<usize> for $name<T, Idx,N> where Idx : $crate:IntegerIndex 
        {
            #[inline(always)]
            fn try_get_many_mut<const N2: usize>(&mut self, indices: [usize; N2]) -> Result<[&mut Self::Output;N2], ()> {
                self.values_mut().try_get_many_mut(indices)
            }
        }

        impl<T, Idx, const N : usize> Index<usize> for $name<T, Idx, N> where Idx : $crate:IntegerIndex
        {
            type Output=T;
            #[inline(always)]
            #[track_caller]
            fn index(&self, index: usize) -> &Self::Output { self.get_or_panic(index) }
        }
        impl<T, Idx, const N : usize> IndexMut<usize> for $name<T, Idx, N> where Idx : $crate:IntegerIndex
        {
            #[inline(always)]
            #[track_caller]
            fn index_mut(&mut self, index: usize) -> &mut Self::Output { self.get_mut_or_panic(index) }
        }

        impl<T, Idx, const N : usize> Index<Vector<Idx,N>> for $name<T, Idx, N> where Idx : $crate:IntegerIndex
        {
            type Output=T;
            #[inline(always)]
            fn index(&self, index: Vector<Idx,N>) -> &Self::Output { self.get_or_panic(index) }
        }
        impl<T, Idx, const N : usize> IndexMut<Vector<Idx,N>> for $name<T, Idx, N> where Idx : $crate:IntegerIndex
        {
            #[inline(always)]
            fn index_mut(&mut self, index: Vector<Idx,N>) -> &mut Self::Output { self.get_mut_or_panic(index) }
        }

        impl<T, Idx, const N : usize> $name<T, Idx, N> where Idx : $crate:IntegerIndex
        {
            pub fn iter(&self) -> Iter<'_,T,Idx,N> { self.into_iter() }
            pub fn iter_mut(&mut self) -> IterMut<'_,T,Idx,N> { self.into_iter() }
        }

        impl<T, Idx, const N : usize> Length for $name<T, Idx, N> 
            where Idx : $crate:IntegerIndex
        {
            #[inline(always)]
            fn len(&self) -> usize { self.values().len() }
        }
    };
}
*/