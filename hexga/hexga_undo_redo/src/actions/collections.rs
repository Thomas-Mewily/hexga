use crate::*;
use super::mem::Replace;

/* 
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Clear<T> where T : Clone + Clearable + Capacity + Length, <T as Capacity>::Param : Default
{ 
    phantom : PhantomData<T> 
}
impl<T>    Clear<T> where T : Clone + Clearable + Capacity + Length, <T as Capacity>::Param : Default { pub const fn new() -> Self { Self { phantom: PhantomData } }}
impl<T>    Default for Clear<T> where T : Clone + Clearable + Capacity + Length, <T as Capacity>::Param : Default { fn default() -> Self { Self{ phantom: PhantomData } } }
impl<T>    Debug   for Clear<T> where T : Clone + Clearable + Capacity + Length, <T as Capacity>::Param : Default { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "Clear") } }


impl<T> UndoableAction for Clear<T> where for<'a> T: 'a + Clone + Clearable + Capacity + Length, <T as Capacity>::Param : Default
{
    type Undo = Replace<T>;
    type Context<'a>= T;
    type Output<'a> = ();
    
    fn execute_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        Replace::new(T::with_capacity(if S::LOG_UNDO { context.len() } else { 0 })).execute_and_forget_in(context, stack);
    }
}


pub struct SwapIndex<T,Idx> where for<'a> T: 'a + CollectionGetMut<Idx>
{   
    pub i: Idx,
    pub j: Idx, 
    phantom : PhantomData<T>
} 

impl<T,Idx> SwapIndex<T,Idx> where for<'a> T: 'a + CollectionGetMut<Idx>
{
    pub const fn new(i : Idx, j : Idx) -> Self { Self{ i, j, phantom: PhantomData } }
    pub fn ij(self) -> (Idx, Idx) { (self.i, self.j) }
}

impl<T,Idx> UndoableAction for SwapIndex<T,Idx> where for<'a> T: 'a + Clone + CollectionGetMut<Idx>
{
    type Undo = Self;
    type Context<'a>= T;
    type Output<'a> = bool;
    
    fn execute_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        context.swap(self.i, self.j)
    }
}
/* 
    some get_disjoint_mut related trait aren't stable yet...
impl<T,Idx> UndoAction for SwapAt<T,Idx> where T: GetIndexMut<Idx>, for<'a> Idx : 'a + Clone, for<'a> T::Output : 'a + Clone, T::Output : Sized 
{
   
}
*/

impl<T,Idx> Copy      for SwapIndex<T,Idx> where for<'a> T: 'a + CollectionGetMut<Idx>, Idx : Copy  {}
impl<T,Idx> Clone     for SwapIndex<T,Idx> where for<'a> T: 'a + CollectionGetMut<Idx>, Idx : Clone { fn clone(&self) -> Self { Self::new(self.i.clone(), self.j.clone()) } }
impl<T,Idx> Eq        for SwapIndex<T,Idx> where for<'a> T: 'a + CollectionGetMut<Idx>, Idx : Eq         {}
impl<T,Idx> PartialEq for SwapIndex<T,Idx> where for<'a> T: 'a + CollectionGetMut<Idx>, Idx : PartialEq  { fn eq(&self, other: &Self) -> bool { self.i == other.i && self.j == other.j } }
impl<T,Idx> Hash      for SwapIndex<T,Idx> where for<'a> T: 'a + CollectionGetMut<Idx>, Idx : Hash       { fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.i.hash(state); self.j.hash(state); } }
impl<T,Idx> Debug     for SwapIndex<T,Idx> where for<'a> T: 'a + CollectionGetMut<Idx>, Idx : Debug      { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.debug_tuple("Swap").field(&self.i).field(&&self.j).finish() } }


/// 2 distincts collection trade/swap one of their value
pub struct TradeIndex<T1,Idx1,T2,Idx2>
    where T1 : CollectionGetMut<Idx1>, Idx1 : Copy, T2 : CollectionGetMut<Idx2,Output = T1::Output>, Idx2 : Copy, T1::Output : Sized, for<'a> T1 : 'a, for<'a> T2 : 'a
{
    pub i : Idx1,
    pub j : Idx2, 
    phantom : PhantomData<(T1,T2)>
} 

impl<T1,Idx1,T2,Idx2> TradeIndex<T1,Idx1,T2,Idx2> where T1 : CollectionGetMut<Idx1>, Idx1 : Copy, T2 : CollectionGetMut<Idx2,Output = T1::Output>, Idx2 : Copy, T1::Output : Sized, for<'a> T1 : 'a, for<'a> T2 : 'a
{
    pub const fn new(i : Idx1, j : Idx2) -> Self { Self {i, j, phantom : PhantomData} }
    pub fn ij(self) -> (Idx1, Idx2) { (self.i, self.j) }
}

impl<T1,Idx1,T2,Idx2> Copy      for TradeIndex<T1,Idx1,T2,Idx2> where T1 : CollectionGetMut<Idx1>, Idx1 : Copy, T2 : CollectionGetMut<Idx2,Output = T1::Output>, Idx2 : Copy, T1::Output : Sized, for<'a> T1 : 'a, for<'a> T2 : 'a, Idx1 : Copy,      Idx2 : Copy       {}
impl<T1,Idx1,T2,Idx2> Clone     for TradeIndex<T1,Idx1,T2,Idx2> where T1 : CollectionGetMut<Idx1>, Idx1 : Copy, T2 : CollectionGetMut<Idx2,Output = T1::Output>, Idx2 : Copy, T1::Output : Sized, for<'a> T1 : 'a, for<'a> T2 : 'a, Idx1 : Clone,     Idx2 : Clone      { fn clone(&self) -> Self { Self::new(self.i.clone(), self.j.clone()) } }
impl<T1,Idx1,T2,Idx2> Eq        for TradeIndex<T1,Idx1,T2,Idx2> where T1 : CollectionGetMut<Idx1>, Idx1 : Copy, T2 : CollectionGetMut<Idx2,Output = T1::Output>, Idx2 : Copy, T1::Output : Sized, for<'a> T1 : 'a, for<'a> T2 : 'a, Idx1 : Eq,        Idx2 : Eq         {}
impl<T1,Idx1,T2,Idx2> PartialEq for TradeIndex<T1,Idx1,T2,Idx2> where T1 : CollectionGetMut<Idx1>, Idx1 : Copy, T2 : CollectionGetMut<Idx2,Output = T1::Output>, Idx2 : Copy, T1::Output : Sized, for<'a> T1 : 'a, for<'a> T2 : 'a, Idx1 : PartialEq, Idx2 : PartialEq  { fn eq(&self, other: &Self) -> bool { self.i == other.i && self.j == other.j } }
impl<T1,Idx1,T2,Idx2> Hash      for TradeIndex<T1,Idx1,T2,Idx2> where T1 : CollectionGetMut<Idx1>, Idx1 : Copy, T2 : CollectionGetMut<Idx2,Output = T1::Output>, Idx2 : Copy, T1::Output : Sized, for<'a> T1 : 'a, for<'a> T2 : 'a, Idx1 : Hash,      Idx2 : Hash       { fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.i.hash(state); self.j.hash(state); } }
impl<T1,Idx1,T2,Idx2> Debug     for TradeIndex<T1,Idx1,T2,Idx2> where T1 : CollectionGetMut<Idx1>, Idx1 : Copy, T2 : CollectionGetMut<Idx2,Output = T1::Output>, Idx2 : Copy, T1::Output : Sized, for<'a> T1 : 'a, for<'a> T2 : 'a, Idx1 : Debug,     Idx2 : Debug      { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.debug_tuple("Trade").field(&self.i).field(&&self.j).finish() } }

impl<T1,Idx1,T2,Idx2> UndoableAction for TradeIndex<T1,Idx1,T2,Idx2> where T1 : CollectionGetMut<Idx1>, Idx1 : Copy, T2 : CollectionGetMut<Idx2,Output = T1::Output>, Idx2 : Copy, T1::Output : Sized, for<'a> T1 : 'a, for<'a> T2 : 'a, Idx1 : Clone, Idx2 : Clone
{
    type Undo = Self;
    type Context<'a>= (T1, T2);
    type Output<'a> = Result<(), ()>; // Todo : put a proper error type
    
    fn execute_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        let (a,b) = context;
        // not a fan of the clone
        match (a.get_mut(self.i.clone()), b.get_mut(self.j.clone()))
        {
            (Some(a), Some(b)) => 
            {
                std::mem::swap(a, b);
                stack.push_undo_action(|| self);
                Ok(())
            },
            _ => Err(())
        }
    }
}

/// Like a ReplaceIndex that don't return anythings / always forgot
pub struct SetIndex<T,Idx> where for<'a> T: 'a + CollectionGetMut<Idx>, T::Output : Sized + Clone
{
    pub idx   : Idx, 
    pub value : T::Output
}

pub trait SetIndexExtension<Idx> : Sized where for<'a> Self: 'a + CollectionGetMut<Idx>, Self::Output : Sized + Clone
{
    fn set_action<'a, S>(&'a mut self, value : Self::Output, idx : Idx, stack : &mut S) -> <SetIndex::<Self, Idx> as UndoableAction>::Output<'a> where S : UndoStack<<SetIndex<Self,Idx> as UndoableAction>::Undo>;
}
impl<T,Idx> SetIndexExtension<Idx> for T where for<'a> Self: 'a + CollectionGetMut<Idx>, Self::Output : Sized + Clone
{
    fn set_action<'a, S>(&'a mut self, value : Self::Output, idx : Idx, stack : &mut S) -> <SetIndex::<Self, Idx> as UndoableAction>::Output<'a> where S : UndoStack<<SetIndex<Self,Idx> as UndoableAction>::Undo> 
    { SetIndex::new(idx, value).execute_in(self, stack) }
}

impl<T,Idx> SetIndex<T,Idx> where for<'a> T: 'a + CollectionGetMut<Idx>, T::Output : Sized + Clone
{
    pub const fn new(idx : Idx, value : T::Output) -> Self { Self { idx, value } }
}

impl<T,Idx> Copy      for SetIndex<T,Idx> where for<'a> T: 'a + CollectionGetMut<Idx>, T::Output : Sized + Clone, Idx : Copy, T::Output : Copy {}
impl<T,Idx> Clone     for SetIndex<T,Idx> where for<'a> T: 'a + CollectionGetMut<Idx>, T::Output : Sized + Clone, Idx : Clone, T::Output : Clone { fn clone(&self) -> Self { Self::new(self.idx.clone(), self.value.clone()) } }
impl<T,Idx> Eq        for SetIndex<T,Idx> where for<'a> T: 'a + CollectionGetMut<Idx>, T::Output : Sized + Clone, Idx : Eq, T::Output : Eq {}
impl<T,Idx> PartialEq for SetIndex<T,Idx> where for<'a> T: 'a + CollectionGetMut<Idx>, T::Output : Sized + Clone, Idx : PartialEq, T::Output : PartialEq { fn eq(&self, other: &Self) -> bool { self.idx == other.idx && self.value == other.value } }
impl<T,Idx> Hash      for SetIndex<T,Idx> where for<'a> T: 'a + CollectionGetMut<Idx>, T::Output : Sized + Clone, Idx : Hash, T::Output : Hash { fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.idx.hash(state); self.value.hash(state); } }
impl<T,Idx> Debug     for SetIndex<T,Idx> where for<'a> T: 'a + CollectionGetMut<Idx>, T::Output : Sized + Clone, Idx : Debug, T::Output : Debug { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.debug_tuple("Set").field(&self.idx).field(&&self.value).finish() } }

impl<T,Idx> UndoableAction for SetIndex<T,Idx>  where for<'a> T: 'a + CollectionGetMut<Idx>, T::Output : Sized, Idx : Clone
{
    type Undo = ReplaceIndex<T,Idx>;
    type Context<'a>= T;
    type Output<'a> = bool;
    
    fn execute_in<'a, S>(mut self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a>  where S : UndoStack<Self::Undo> 
    {
        // I don't like this clone. Imagine it on a HashMap<String, ...>...
        // Maybe change the api to look like the HashMap::get fn
        match context.get_mut(self.idx.clone())
        {
            Some(v) => 
            {
                std::mem::swap(&mut self.value, v);
                stack.push_undo_action(|| ReplaceIndex::new(self.idx.clone(), self.value));
                true
            },
            None => false,
        }
    }
}




pub struct ReplaceIndex<T,Idx> where for<'a> T: 'a + CollectionGetMut<Idx>, T::Output : Sized + Clone
{
    pub index   : Idx, 
    pub value : T::Output
}

impl<T,Idx> ReplaceIndex<T,Idx> where for<'a> T: 'a + CollectionGetMut<Idx>, T::Output : Sized + Clone
{
    pub const fn new(index : Idx, value : T::Output) -> Self { Self { index, value } }
}

impl<T,Idx> Copy      for ReplaceIndex<T,Idx> where for<'a> T: 'a + CollectionGetMut<Idx>, T::Output : Sized + Clone, T::Output : Copy {}
impl<T,Idx> Clone     for ReplaceIndex<T,Idx> where for<'a> T: 'a + CollectionGetMut<Idx>, T::Output : Sized + Clone, T::Output : Clone { fn clone(&self) -> Self { Self::new(self.index.clone(), self.value.clone()) } }
impl<T,Idx> Eq        for ReplaceIndex<T,Idx> where for<'a> T: 'a + CollectionGetMut<Idx>, T::Output : Sized + Clone, Idx : Eq, T::Output : Eq {}
impl<T,Idx> PartialEq for ReplaceIndex<T,Idx> where for<'a> T: 'a + CollectionGetMut<Idx>, T::Output : Sized + Clone, Idx : PartialEq, T::Output : PartialEq { fn eq(&self, other: &Self) -> bool { self.index == other.index && self.value == other.value } }
impl<T,Idx> Hash      for ReplaceIndex<T,Idx> where for<'a> T: 'a + CollectionGetMut<Idx>, T::Output : Sized + Clone, Idx : Hash, T::Output : Hash { fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.index.hash(state); self.value.hash(state); } }
impl<T,Idx> Debug     for ReplaceIndex<T,Idx> where for<'a> T: 'a + CollectionGetMut<Idx>, T::Output : Sized + Clone, Idx : Debug, T::Output : Debug { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "Replace([{:?}]=>{:?})", self.index, self.value) } }

impl<T,Idx> UndoableAction for ReplaceIndex<T,Idx>  where for<'a> T: 'a + CollectionGetMut<Idx>, T::Output : Sized + Clone
{
    type Undo = Self;
    type Context<'a>= T;
    type Output<'ctx> = Result<T::Output, ()>;
    
    fn execute_in<'a, S>(mut self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        // I don't like this clone. Imagine it on a HashMap<String, ...>...
        // Maybe change the api to look like the HashMap::get fn
        match context.get_mut(self.index.clone())
        {
            Some(v) => 
            {
                std::mem::swap(&mut self.value, v);
                stack.push_undo_action(|| ReplaceIndex::new(self.index.clone(), self.value.clone()));
                Ok(self.value)
            },
            None => Err(()),
        }
    }

    fn execute_and_forget_in<'a, S>(mut self, context : &mut Self::Context<'a>, stack : &mut S) where S : UndoStack<Self::Undo> {
        // I don't like this clone. Imagine it on a HashMap<String, ...>...
        // Maybe change the api to look like the HashMap::get fn
        match context.get_mut(self.index.clone())
        {
            Some(v) => 
            {
                std::mem::swap(&mut self.value, v);
                stack.push_undo_action(|| ReplaceIndex::new(self.index.clone(), self.value));
            },
            None => {},
        };
    }
}
    */