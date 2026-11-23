use super::*;

pub struct GridViewIterMut<'a, G, T, Idx, const N: usize>
where
    G: IGrid<T, Idx, N>,
    Idx: Integer,
    T: 'a,
{
    grid: &'a mut G,
    rect: RectangleIter<Idx, N>,
    phantom: std::marker::PhantomData<T>,
}

impl<'a, G, T, Idx, const N: usize> GridViewIterMut<'a, G, T, Idx, N>
where
    G: IGrid<T, Idx, N>,
    Idx: Integer,
    T: 'a,
{
    pub fn new(grid: &'a mut G) -> Self
    {
        let rect = grid.rect().iter_index();
        Self {
            grid,
            rect,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn from_rect(grid: &'a mut G, rect: Rectangle<Idx, N>) -> Option<Self>
    {
        if !grid.rect().is_rect_inside(rect)
        {
            None
        }
        else
        {
            Some(unsafe { Self::from_rect_unchecked(grid, rect) })
        }
    }

    pub unsafe fn from_rect_unchecked(grid: &'a mut G, rect: Rectangle<Idx, N>) -> Self
    {
        Self {
            grid,
            rect: rect.iter_index(),
            phantom: std::marker::PhantomData,
        }
    }

    pub fn from_rect_intersect(grid: &'a mut G, rect: Rectangle<Idx, N>) -> Self
    {
        let rect = grid.rect().intersect_or_empty(rect).iter_index();
        Self {
            grid,
            rect,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, G, T, Idx, const N: usize> Iterator for GridViewIterMut<'a, G, T, Idx, N>
where
    G: IGrid<T, Idx, N>,
    Idx: Integer,
{
    type Item = (Vector<Idx, N>, &'a mut T);

    fn next(&mut self) -> Option<Self::Item>
    {
        let idx = self.rect.next()?;
        // # Safety
        // Each index are different
        Some((idx, unsafe { (&mut *(&mut *self.grid as *mut G)).index_mut(idx) }))
    }

    fn size_hint(&self) -> (usize, Option<usize>) { self.rect.size_hint() }
}
impl<'a, G, T, Idx, const N: usize> std::iter::FusedIterator for GridViewIterMut<'a, G, T, Idx, N>
where
    G: IGrid<T, Idx, N>,
    Idx: Integer,
    RectangleIter<Idx, N>: std::iter::FusedIterator,
{
}
impl<'a, G, T, Idx, const N: usize> std::iter::ExactSizeIterator for GridViewIterMut<'a, G, T, Idx, N>
where
    G: IGrid<T, Idx, N>,
    Idx: Integer,
    RectangleIter<Idx, N>: std::iter::ExactSizeIterator,
{
}


/*
impl<'a, G, T, Idx, const N: usize> Map for GridViewMut<'a, G, T, Idx, N>
where
    G: IGrid<T, Idx, N>,
    Idx: Integer,
{
    type Item = T;

    fn map<F>(mut self, mut f: F) -> Self
    where
        F: FnMut(T) -> T,
    {
        self.for_each_mut(|(_, value)| {
            // RefMutMap requires the callback not to panic; see its panic contract.
            unsafe { <&mut T as RefMutMap>::map(value, &mut f) };
        });
        self
    }
}
*/

/*
impl<'a, G, T, Idx, const N: usize> MapZip for GridViewMut<'a, G, T, Idx, N>
where
    G: IGrid<T, Idx, N>,
    Idx: Integer,
    T: Clone,
{
    fn map_zip<F, const M: usize>(this: [Self; M], mut f: F) -> Self
    where
        F: FnMut([T; M]) -> T,
    {
        const { assert!(M > 0); }

        let mut views = this.into_iter();
        let mut result = views.next().expect("at least one grid view is required");
        let size = result.size();
        let sources: Vec<_> = views.collect();
        assert!(sources.iter().all(|view| view.size() == size), "size mismatch");

        let mut source_iters: Vec<_> = sources.iter().map(|view| view.iter()).collect();
        result.for_each_mut(|(_, target)| {
            let mut values = Vec::with_capacity(M);
            values.push(target.clone());
            for iter in &mut source_iters {
                values.push(iter.next().expect("grid views have matching sizes").1.clone());
            }
            let values: [T; M] = values.try_into().ok().expect("one value per grid view");
            *target = f(values);
        });

        result
    }
}
*/