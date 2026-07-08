use super::*;

pub trait RetainMut<T>
{
    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all elements `e` for which `f(&e)` returns `false`.
    /// This method operates in place, visiting each element exactly once in the
    /// original order, and preserves the order of the retained elements.
    fn retain_mut<F>(&mut self, f: F)
    where
        F: FnMut(&mut T) -> bool;
}

impl<T> RetainMut<T> for Vec<T>
{
    fn retain_mut<F>(&mut self, f: F)
    where
        F: FnMut(&mut T) -> bool,
    {
        self.retain_mut(f);
    }
}

impl<T> RetainMut<T> for VecDeque<T>
{
    fn retain_mut<F>(&mut self, f: F)
    where
        F: FnMut(&mut T) -> bool,
    {
        self.retain_mut(f);
    }
}

/*

Retain for Vec: Fn(&T)
Retain for String: Fn(char) (can make a wrapper to use Fn(&char))
Retain for HashMap: Fn(&K, &mut V) (idk how to wrap it properly)

???

pub trait Retain
{
    type Item<'a> : where Self: 'a;
    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all elements `e` for which `f(&e)` returns `false`.
    /// This method operates in place, visiting each element exactly once in the
    /// original order, and preserves the order of the retained elements.
    fn retain<F>(&mut self, f: F)
    where
        F: FnMut(Self::Item<'a>) -> bool;
}


*/

/*



pub trait Retain<T>
{
    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all elements `e` for which `f(&e)` returns `false`.
    /// This method operates in place, visiting each element exactly once in the
    /// original order, and preserves the order of the retained elements.
    fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&T) -> bool;
}



impl<T> Retain<T> for Vec<T>
{
    fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&T) -> bool {
        self.retain(f);
    }
}


impl<T> Retain<T> for VecDeque<T>
{
    fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&T) -> bool {
        self.retain(f);
    }
}


impl Retain<char> for String
{
    fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&char) -> bool {
        self.retain(|c| f(&c));
    }
}

/*
impl<K,V> Retain<(K,V)> for HashMap<K,V>
{
    fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&(K,V)) -> bool {
        self.retain(|k,v| f(&(k,v)));
    }
}

#[cfg(feature = "std")]
impl<T> RetainMut<T> for VecDeque<T>
{
    fn retain_mut<F>(&mut self, f: F)
    where
        F: FnMut(&mut T) -> bool {
        self.retain_mut(f);
    }
}
*/
*/
