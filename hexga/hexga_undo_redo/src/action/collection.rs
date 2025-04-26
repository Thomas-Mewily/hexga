use crate::*;

use super::mem::Replace;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Clear<T> where T : Clone + Clearable + Capacity + Length, <T as Capacity>::Param : Default
{ 
    phantom : PhantomData<T> 
}
impl<T>    Clear<T> where T : Clone + Clearable + Capacity + Length, <T as Capacity>::Param : Default { pub const fn new() -> Self { Self { phantom: PhantomData } }}
impl<T>    Default for Clear<T> where T : Clone + Clearable + Capacity + Length, <T as Capacity>::Param : Default { fn default() -> Self { Self{ phantom: PhantomData } } }
impl<T>    Debug   for Clear<T> where T : Clone + Clearable + Capacity + Length, <T as Capacity>::Param : Default { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "Clear") } }

impl<T> UndoAction for Clear<T> where for<'a> T: 'a + Clone + Clearable + Capacity + Length, <T as Capacity>::Param : Default
{
    type Undo = Replace<T>;
    type Context<'a>= T;
    type Output<'a> = ();
    
    fn execute<'a, U>(self, context : &mut Self::Context<'a>, undo : &mut U) -> Self::Output<'a> where U : ActionStack<Self::Undo> 
    {
        Replace::new(T::with_capacity(context.len())).execute_and_forget(context, undo);
    }

    fn execute_without_undo<'a>(self, context : &mut  Self::Context<'a>) -> Self::Output<'a> {
        Replace::new(T::with_capacity(0)).execute_without_undo(context);
    }
}


pub struct SwapIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>
{   
    pub i: Idx,
    pub j: Idx, 
    phantom : PhantomData<T>
} 

impl<T,Idx> SwapIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>
{
    pub const fn new(i : Idx, j : Idx) -> Self { Self{ i, j, phantom: PhantomData } }
    pub fn ij(self) -> (Idx, Idx) { (self.i, self.j) }
}

impl<T,Idx> Copy      for SwapIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, Idx : Copy       {}
impl<T,Idx> Clone     for SwapIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, Idx : Clone      { fn clone(&self) -> Self { Self::new(self.i.clone(), self.j.clone()) } }
impl<T,Idx> Eq        for SwapIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, Idx : Eq         {}
impl<T,Idx> PartialEq for SwapIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, Idx : PartialEq  { fn eq(&self, other: &Self) -> bool { self.i == other.i && self.j == other.j } }
impl<T,Idx> Hash      for SwapIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, Idx : Hash       { fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.i.hash(state); self.j.hash(state); } }
impl<T,Idx> Debug     for SwapIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, Idx : Debug      { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.debug_tuple("Swap").field(&self.i).field(&&self.j).finish() } }


/// 2 distincts collection trade/swap one of their value
pub struct TradeIndex<T1,Idx1,T2,Idx2>
    where T1 : GetIndexMut<Idx1>, T2 : GetIndexMut<Idx2,Output = T1::Output>, T1::Output : Sized, for<'a> T1 : 'a, for<'a> T2 : 'a
{
    pub i : Idx1,
    pub j : Idx2, 
    phantom : PhantomData<(T1,T2)>
} 

impl<T1,Idx1,T2,Idx2> TradeIndex<T1,Idx1,T2,Idx2> where T1 : GetIndexMut<Idx1>, T2 : GetIndexMut<Idx2,Output = T1::Output>, T1::Output : Sized, for<'a> T1 : 'a, for<'a> T2 : 'a
{
    pub const fn new(i : Idx1, j : Idx2) -> Self { Self {i, j, phantom : PhantomData} }
    pub fn ij(self) -> (Idx1, Idx2) { (self.i, self.j) }
}

impl<T1,Idx1,T2,Idx2> Copy      for TradeIndex<T1,Idx1,T2,Idx2> where T1 : GetIndexMut<Idx1>, T2 : GetIndexMut<Idx2,Output = T1::Output>, T1::Output : Sized, for<'a> T1 : 'a, for<'a> T2 : 'a, Idx1 : Copy,      Idx2 : Copy       {}
impl<T1,Idx1,T2,Idx2> Clone     for TradeIndex<T1,Idx1,T2,Idx2> where T1 : GetIndexMut<Idx1>, T2 : GetIndexMut<Idx2,Output = T1::Output>, T1::Output : Sized, for<'a> T1 : 'a, for<'a> T2 : 'a, Idx1 : Clone,     Idx2 : Clone      { fn clone(&self) -> Self { Self::new(self.i.clone(), self.j.clone()) } }
impl<T1,Idx1,T2,Idx2> Eq        for TradeIndex<T1,Idx1,T2,Idx2> where T1 : GetIndexMut<Idx1>, T2 : GetIndexMut<Idx2,Output = T1::Output>, T1::Output : Sized, for<'a> T1 : 'a, for<'a> T2 : 'a, Idx1 : Eq,        Idx2 : Eq         {}
impl<T1,Idx1,T2,Idx2> PartialEq for TradeIndex<T1,Idx1,T2,Idx2> where T1 : GetIndexMut<Idx1>, T2 : GetIndexMut<Idx2,Output = T1::Output>, T1::Output : Sized, for<'a> T1 : 'a, for<'a> T2 : 'a, Idx1 : PartialEq, Idx2 : PartialEq  { fn eq(&self, other: &Self) -> bool { self.i == other.i && self.j == other.j } }
impl<T1,Idx1,T2,Idx2> Hash      for TradeIndex<T1,Idx1,T2,Idx2> where T1 : GetIndexMut<Idx1>, T2 : GetIndexMut<Idx2,Output = T1::Output>, T1::Output : Sized, for<'a> T1 : 'a, for<'a> T2 : 'a, Idx1 : Hash,      Idx2 : Hash       { fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.i.hash(state); self.j.hash(state); } }
impl<T1,Idx1,T2,Idx2> Debug     for TradeIndex<T1,Idx1,T2,Idx2> where T1 : GetIndexMut<Idx1>, T2 : GetIndexMut<Idx2,Output = T1::Output>, T1::Output : Sized, for<'a> T1 : 'a, for<'a> T2 : 'a, Idx1 : Debug,     Idx2 : Debug      { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.debug_tuple("Trade").field(&self.i).field(&&self.j).finish() } }

impl<T1,Idx1,T2,Idx2> UndoAction for TradeIndex<T1,Idx1,T2,Idx2> where T1 : GetIndexMut<Idx1>, T2 : GetIndexMut<Idx2,Output = T1::Output>, T1::Output : Sized, for<'a> T1 : 'a, for<'a> T2 : 'a, Idx1 : Clone, Idx2 : Clone
{
    type Undo = Self;
    type Context<'a>= (T1, T2);
    type Output<'a> = Result<(), ()>; // Todo : put a proper error type
    
    fn execute<'a, U>(self, context : &mut Self::Context<'a>, undo : &mut U) -> Self::Output<'a> where U : ActionStack<Self::Undo> 
    {
        let (a,b) = context;
        // not a fan of the clone
        match (a.get_mut(self.i.clone()), b.get_mut(self.j.clone()))
        {
            (Some(a), Some(b)) => 
            {
                std::mem::swap(a, b);
                undo.push_undo_action(|| self);
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


/// Like a ReplaceIndex that don't return anythings / always forgot
pub struct SetIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, T::Output : Sized + Clone, Idx : Clone
{
    pub idx   : Idx, 
    pub value : T::Output
}

impl<T,Idx> SetIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, T::Output : Sized + Clone, Idx : Clone
{
    pub const fn new(idx : Idx, value : T::Output) -> Self { Self { idx, value } }
}

impl<T,Idx> Copy      for SetIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, T::Output : Sized + Clone, Idx : Clone, Idx : Copy, T::Output : Copy {}
impl<T,Idx> Clone     for SetIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, T::Output : Sized + Clone, Idx : Clone, Idx : Clone, T::Output : Clone { fn clone(&self) -> Self { Self::new(self.idx.clone(), self.value.clone()) } }
impl<T,Idx> Eq        for SetIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, T::Output : Sized + Clone, Idx : Clone, Idx : Eq, T::Output : Eq {}
impl<T,Idx> PartialEq for SetIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, T::Output : Sized + Clone, Idx : Clone, Idx : PartialEq, T::Output : PartialEq { fn eq(&self, other: &Self) -> bool { self.idx == other.idx && self.value == other.value } }
impl<T,Idx> Hash      for SetIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, T::Output : Sized + Clone, Idx : Clone, Idx : Hash, T::Output : Hash { fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.idx.hash(state); self.value.hash(state); } }
impl<T,Idx> Debug     for SetIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, T::Output : Sized + Clone, Idx : Clone, Idx : Debug, T::Output : Debug { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.debug_tuple("Set").field(&self.idx).field(&&self.value).finish() } }

impl<T,Idx> UndoAction for SetIndex<T,Idx>  where for<'a> T: 'a + GetIndexMut<Idx>, T::Output : Sized + Clone, Idx : Clone
{
    type Undo = ReplaceIndex<T,Idx>;
    type Context<'a>= T;
    type Output<'ctx> = Result<(), ()>;
    
    fn execute<'a, U>(mut self, context : &mut Self::Context<'a>, undo : &mut U) -> Self::Output<'a>  where U : ActionStack<Self::Undo> 
    {
        // I don't like this clone. Imagine it on a HashMap<String, ...>...
        // Maybe change the api to look like the HashMap::get fn
        match context.get_mut(self.idx.clone())
        {
            Some(v) => 
            {
                std::mem::swap(&mut self.value, v);
                undo.push_undo_action(|| ReplaceIndex::new(self.idx.clone(), self.value));
                Ok(())
            },
            None => Err(()),
        }
    }
}




pub struct ReplaceIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, T::Output : Sized + Clone, Idx : Clone
{
    pub idx   : Idx, 
    pub value : T::Output
}

impl<T,Idx> ReplaceIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, T::Output : Sized + Clone, Idx : Clone
{
    pub const fn new(idx : Idx, value : T::Output) -> Self { Self { idx, value } }
}

impl<T,Idx> Copy      for ReplaceIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, T::Output : Sized + Clone, Idx : Clone, Idx : Copy, T::Output : Copy {}
impl<T,Idx> Clone     for ReplaceIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, T::Output : Sized + Clone, Idx : Clone, Idx : Clone, T::Output : Clone { fn clone(&self) -> Self { Self::new(self.idx.clone(), self.value.clone()) } }
impl<T,Idx> Eq        for ReplaceIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, T::Output : Sized + Clone, Idx : Clone, Idx : Eq, T::Output : Eq {}
impl<T,Idx> PartialEq for ReplaceIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, T::Output : Sized + Clone, Idx : Clone, Idx : PartialEq, T::Output : PartialEq { fn eq(&self, other: &Self) -> bool { self.idx == other.idx && self.value == other.value } }
impl<T,Idx> Hash      for ReplaceIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, T::Output : Sized + Clone, Idx : Clone, Idx : Hash, T::Output : Hash { fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.idx.hash(state); self.value.hash(state); } }
impl<T,Idx> Debug     for ReplaceIndex<T,Idx> where for<'a> T: 'a + GetIndexMut<Idx>, T::Output : Sized + Clone, Idx : Clone, Idx : Debug, T::Output : Debug { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.debug_tuple("Replace").field(&self.idx).field(&&self.value).finish() } }

impl<T,Idx> UndoAction for ReplaceIndex<T,Idx>  where for<'a> T: 'a + GetIndexMut<Idx>, T::Output : Sized + Clone, Idx : Clone
{
    type Undo = Self;
    type Context<'a>= T;
    type Output<'ctx> = Result<T::Output, ()>;
    
    fn execute<'a, U>(mut self, context : &mut Self::Context<'a>, undo : &mut U) -> Self::Output<'a> where U : ActionStack<Self::Undo> 
    {
        // I don't like this clone. Imagine it on a HashMap<String, ...>...
        // Maybe change the api to look like the HashMap::get fn
        match context.get_mut(self.idx.clone())
        {
            Some(v) => 
            {
                std::mem::swap(&mut self.value, v);
                undo.push_undo_action(|| ReplaceIndex::new(self.idx.clone(), self.value.clone()));
                Ok(self.value)
            },
            None => Err(()),
        }
    }

    fn execute_and_forget<'a, U>(mut self, context : &mut Self::Context<'a>, undo : &mut U) where U : ActionStack<Self::Undo> {
        // I don't like this clone. Imagine it on a HashMap<String, ...>...
        // Maybe change the api to look like the HashMap::get fn
        match context.get_mut(self.idx.clone())
        {
            Some(v) => 
            {
                std::mem::swap(&mut self.value, v);
                undo.push_undo_action(|| ReplaceIndex::new(self.idx.clone(), self.value));
            },
            None => {},
        };
    }
}