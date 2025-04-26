use crate::*;

use super::mem::Replace;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Clear<T>(PhantomData<T>) where T : Clone + Clearable + Capacity + Length, <T as Capacity>::Param : Default;
impl<T>    Default for Clear<T> where T : Clone + Clearable + Capacity + Length, <T as Capacity>::Param : Default { fn default() -> Self { Self(PhantomData) } }
impl<T>    Debug   for Clear<T> where T : Clone + Clearable + Capacity + Length, <T as Capacity>::Param : Default { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "Clear") } }

impl<T> UndoAction for Clear<T> where for<'a> T: 'a + Clone + Clearable + Capacity + Length, <T as Capacity>::Param : Default
{
    type Undo = Replace<T>;
    type Context<'a>= &'a mut T;
    type Output<'a> = ();
    
    fn execute<'a, U>(self, context : Self::Context<'a>, undo : &mut U) -> Self::Output<'a> where U : UndoStack<Self::Undo> 
    {
        Replace(T::with_capacity(context.len())).execute_and_forget(context, undo);
    }

    fn execute_without_undo<'a>(self, context : Self::Context<'a>) -> Self::Output<'a> {
        Replace(T::with_capacity(0)).execute_without_undo(context);
    }
}


pub struct SwapIndex<T,Idx>(Idx,Idx, PhantomData<T>) where for<'a> T: 'a + GetIndexMut<Idx>;

impl<T,Idx> SwapIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>
{
    pub fn new(i : Idx, j : Idx) -> Self { Self(i, j, PhantomData) }

    pub fn i_ref(&self) -> &Idx { &self.0 }
    pub fn j_ref(&self) -> &Idx { &self.1 }

    pub fn ij_ref(&self) -> (&Idx, &Idx) { (&self.0, &self.1) }

    pub fn i(self) -> Idx { self.0 }
    pub fn j(self) -> Idx { self.1 }
    pub fn ij(self) -> (Idx, Idx) { (self.0, self.1) }

    pub fn i_mut(&mut self) -> &mut Idx { &mut self.0 }
    pub fn j_mut(&mut self) -> &mut Idx { &mut self.1 }
    pub fn ij_mut(&mut self) -> (&mut Idx, &mut Idx) { (&mut self.0, &mut self.1) }
}

impl<T,Idx> Copy      for SwapIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, Idx : Copy       {}
impl<T,Idx> Clone     for SwapIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, Idx : Clone      { fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone(), PhantomData) } }
impl<T,Idx> Eq        for SwapIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, Idx : Eq         {}
impl<T,Idx> PartialEq for SwapIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, Idx : PartialEq  { fn eq(&self, other: &Self) -> bool { self.0 == other.0 && self.1 == other.1 } }
impl<T,Idx> Hash      for SwapIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, Idx : Hash       { fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.0.hash(state); self.1.hash(state); } }
impl<T,Idx> Debug     for SwapIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, Idx : Debug      { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.debug_tuple("Swap").field(&self.0).field(&&self.1).finish() } }


/// 2 distincts collection trade/swap one of their value
pub struct TradeIndex<T1,Idx1,T2,Idx2>(Idx1,Idx2, PhantomData<(T1,T2)>) 
    where T1 : GetIndexMut<Idx1>, T2 : GetIndexMut<Idx2,Output = T1::Output>, T1::Output : Sized, for<'a> T1 : 'a, for<'a> T2 : 'a;

impl<T1,Idx1,T2,Idx2> TradeIndex<T1,Idx1,T2,Idx2> where T1 : GetIndexMut<Idx1>, T2 : GetIndexMut<Idx2,Output = T1::Output>, T1::Output : Sized, for<'a> T1 : 'a, for<'a> T2 : 'a
{
    pub fn new(i : Idx1, j : Idx2) -> Self { Self(i, j, PhantomData) }

    pub fn i_ref(&self) -> &Idx1 { &self.0 }
    pub fn j_ref(&self) -> &Idx2 { &self.1 }

    pub fn ij_ref(&self) -> (&Idx1, &Idx2) { (&self.0, &self.1) }

    pub fn i(self) -> Idx1 { self.0 }
    pub fn j(self) -> Idx2 { self.1 }
    pub fn ij(self) -> (Idx1, Idx2) { (self.0, self.1) }

    pub fn i_mut(&mut self) -> &mut Idx1 { &mut self.0 }
    pub fn j_mut(&mut self) -> &mut Idx2 { &mut self.1 }
    pub fn ij_mut(&mut self) -> (&mut Idx1, &mut Idx2) { (&mut self.0, &mut self.1) }
}

impl<T1,Idx1,T2,Idx2> Copy      for TradeIndex<T1,Idx1,T2,Idx2> where T1 : GetIndexMut<Idx1>, T2 : GetIndexMut<Idx2,Output = T1::Output>, T1::Output : Sized, for<'a> T1 : 'a, for<'a> T2 : 'a, Idx1 : Copy,      Idx2 : Copy       {}
impl<T1,Idx1,T2,Idx2> Clone     for TradeIndex<T1,Idx1,T2,Idx2> where T1 : GetIndexMut<Idx1>, T2 : GetIndexMut<Idx2,Output = T1::Output>, T1::Output : Sized, for<'a> T1 : 'a, for<'a> T2 : 'a, Idx1 : Clone,     Idx2 : Clone      { fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone(), PhantomData) } }
impl<T1,Idx1,T2,Idx2> Eq        for TradeIndex<T1,Idx1,T2,Idx2> where T1 : GetIndexMut<Idx1>, T2 : GetIndexMut<Idx2,Output = T1::Output>, T1::Output : Sized, for<'a> T1 : 'a, for<'a> T2 : 'a, Idx1 : Eq,        Idx2 : Eq         {}
impl<T1,Idx1,T2,Idx2> PartialEq for TradeIndex<T1,Idx1,T2,Idx2> where T1 : GetIndexMut<Idx1>, T2 : GetIndexMut<Idx2,Output = T1::Output>, T1::Output : Sized, for<'a> T1 : 'a, for<'a> T2 : 'a, Idx1 : PartialEq, Idx2 : PartialEq  { fn eq(&self, other: &Self) -> bool { self.0 == other.0 && self.1 == other.1 } }
impl<T1,Idx1,T2,Idx2> Hash      for TradeIndex<T1,Idx1,T2,Idx2> where T1 : GetIndexMut<Idx1>, T2 : GetIndexMut<Idx2,Output = T1::Output>, T1::Output : Sized, for<'a> T1 : 'a, for<'a> T2 : 'a, Idx1 : Hash,      Idx2 : Hash       { fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.0.hash(state); self.1.hash(state); } }
impl<T1,Idx1,T2,Idx2> Debug     for TradeIndex<T1,Idx1,T2,Idx2> where T1 : GetIndexMut<Idx1>, T2 : GetIndexMut<Idx2,Output = T1::Output>, T1::Output : Sized, for<'a> T1 : 'a, for<'a> T2 : 'a, Idx1 : Debug,     Idx2 : Debug      { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.debug_tuple("Trade").field(&self.0).field(&&self.1).finish() } }

impl<T1,Idx1,T2,Idx2> UndoAction for TradeIndex<T1,Idx1,T2,Idx2> where T1 : GetIndexMut<Idx1>, T2 : GetIndexMut<Idx2,Output = T1::Output>, T1::Output : Sized, for<'a> T1 : 'a, for<'a> T2 : 'a, Idx1 : Clone, Idx2 : Clone
{
    type Undo = Self;
    type Context<'a>= (&'a mut T1, &'a mut T2);
    type Output<'a> = Result<(), ()>; // Todo : put a proper error type
    
    fn execute<'a, U>(self, context : Self::Context<'a>, undo : &mut U) -> Self::Output<'a> where U : UndoStack<Self::Undo> 
    {
        let (a,b) = context;
        // not a fan of the clone
        match (a.get_mut(self.i_ref().clone()), b.get_mut(self.j_ref().clone()))
        {
            (Some(a), Some(b)) => 
            {
                std::mem::swap(a, b);
                undo.push(|| self);
                Ok(())
            },
            _ => Err(())
        }
    }
}
/* 
    some get_disjoint_mut related trait aren't stable yet...
    T : GetIndexMut
impl<T,Idx> UndoAction for SwapAt<T,Idx> where T: GetIndexMut<Idx>, for<'a> Idx : 'a + Clone, for<'a> T::Output : 'a + Clone, T::Output : Sized 
{
    type Undo = Self;
    type Context=T;
    type Output<'ctx> = Result<(), ()>;
    
    fn execute<'ctx, U>(mut self, context : &'ctx mut Self::Context, undo : &'ctx mut U) -> Self::Output<'ctx> where U : UndoStack<Self::Undo> 
    {
        let SwapAt(idx, value) = &mut self;

        // I don't like this clone. Imagine it on a HashMap<String, ...>...
        // Maybe change the api to look like the HashMap::get fn
        match context.get_mut(idx.clone()) 
        {
            Some(v) => 
            {
                std::mem::swap(value, v);
                undo.push(|| self);
                Ok(())
            },
            None => Err(()),
        }
    }
}
*/



pub struct SetIndex<T,Idx>(pub Idx, pub T::Output) where for<'a> T: 'a + GetIndexMut<Idx>;

impl<T,Idx> Copy      for SetIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, Idx : Copy, T::Output : Copy {}
impl<T,Idx> Clone     for SetIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, Idx : Clone, T::Output : Clone { fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) } }
impl<T,Idx> Eq        for SetIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, Idx : Eq, T::Output : Eq {}
impl<T,Idx> PartialEq for SetIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, Idx : PartialEq, T::Output : PartialEq { fn eq(&self, other: &Self) -> bool { self.0 == other.0 && self.1 == other.1 } }
impl<T,Idx> Hash      for SetIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, Idx : Hash, T::Output : Hash { fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.0.hash(state); self.1.hash(state); } }
impl<T,Idx> Debug     for SetIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, Idx : Debug, T::Output : Debug { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.debug_tuple("Set").field(&self.0).field(&&self.1).finish() } }

impl<T,Idx> UndoAction for SetIndex<T,Idx>  where for<'a> T: 'a + GetIndexMut<Idx>, T::Output : Sized + Clone, Idx : Clone
{
    type Undo = ReplaceIndex<T,Idx>;
    type Context<'a>= &'a mut T;
    type Output<'ctx> = ();
    
    fn execute<'a, U>(self, context : Self::Context<'a>, undo : &mut U) -> Self::Output<'a>  where U : UndoStack<Self::Undo> 
    {
        ReplaceIndex(self.0, self.1).execute_and_forget(context, undo);
    }
}



pub struct ReplaceIndex<T,Idx>(pub Idx, pub T::Output) where for<'a> T: 'a + GetIndexMut<Idx>;

impl<T,Idx> Copy      for ReplaceIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, Idx : Copy, T::Output : Copy {}
impl<T,Idx> Clone     for ReplaceIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, Idx : Clone, T::Output : Clone { fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) } }
impl<T,Idx> Eq        for ReplaceIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, Idx : Eq, T::Output : Eq {}
impl<T,Idx> PartialEq for ReplaceIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, Idx : PartialEq, T::Output : PartialEq { fn eq(&self, other: &Self) -> bool { self.0 == other.0 && self.1 == other.1 } }
impl<T,Idx> Hash      for ReplaceIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, Idx : Hash, T::Output : Hash { fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.0.hash(state); self.1.hash(state); } }
impl<T,Idx> Debug     for ReplaceIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, Idx : Debug, T::Output : Debug { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.debug_tuple("Replace").field(&self.0).field(&&self.1).finish() } }

impl<T,Idx> UndoAction for ReplaceIndex<T,Idx>  where for<'a> T: 'a + GetIndexMut<Idx>, T::Output : Sized + Clone, Idx : Clone
{
    type Undo = Self;
    type Context<'a>= &'a mut T;
    type Output<'ctx> = Option<T::Output>;
    
    fn execute<'a, U>(mut self, context : Self::Context<'a>, undo : &mut U) -> Self::Output<'a> where U : UndoStack<Self::Undo> 
    {
        // I don't like this clone. Imagine it on a HashMap<String, ...>...
        // Maybe change the api to look like the HashMap::get fn
        match context.get_mut(self.0.clone())
        {
            Some(v) => 
            {
                std::mem::swap(&mut self.1, v);
                undo.push(|| ReplaceIndex(self.0.clone(), self.1.clone()));
                Some(self.1)
            },
            None => None,
        }
    }

    fn execute_and_forget<'a, U>(mut self, context : Self::Context<'a>, undo : &mut U) where U : UndoStack<Self::Undo> {
        // I don't like this clone. Imagine it on a HashMap<String, ...>...
        // Maybe change the api to look like the HashMap::get fn
        match context.get_mut(self.0.clone())
        {
            Some(v) => 
            {
                std::mem::swap(&mut self.1, v);
                undo.push(|| ReplaceIndex(self.0.clone(), self.1));
            },
            None => {},
        };
    }
}