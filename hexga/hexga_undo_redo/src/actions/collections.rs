use crate::*;
use super::mem::*;

pub struct SwapIndex<T,Idx,P=policy::Normal> where for<'a> T: 'a + GetManyMut<Idx>, T::Output : Sized, Idx : Clone, P : Policy
{   
    pub i: Idx,
    pub j: Idx, 
    phantom : PhantomData<(T,P)>
} 

impl<T,Idx,P> SwapIndex<T,Idx,P> where for<'a> T: 'a + GetManyMut<Idx>, T::Output : Sized, Idx : Clone, P : Policy
{
    pub const fn new(i : Idx, j : Idx) -> Self { Self{ i, j, phantom: PhantomData } }
    pub fn ij(self) -> (Idx, Idx) { (self.i, self.j) }
}


impl<T,Idx> UndoableAction for SwapIndex<T,Idx,policy::Try> where for<'a> T: 'a + GetManyMut<Idx>, T::Output : Sized, Idx : Clone
{
    type Undo = Self;   type Context<'a>= T;  type Output<'a> = Result<(), ()>;
    fn execute_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        stack.push_undo_action(|| self.clone());
        context.try_swap(self.i, self.j)
    }
}
impl<T,Idx> UndoableAction for SwapIndex<T,Idx,policy::Normal> where for<'a> T: 'a + GetManyMut<Idx>, T::Output : Sized, Idx : Clone
{
    type Undo = Self;  type Context<'a>= T;  type Output<'a> = bool;
    fn execute_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        stack.push_undo_action(|| self.clone());
        context.swap(self.i, self.j)
    }
}
impl<T,Idx> UndoableAction for SwapIndex<T,Idx,policy::Panic> where for<'a> T: 'a + GetManyMut<Idx>, T::Output : Sized, Idx : Clone
{
    type Undo = Self;  type Context<'a>= T;  type Output<'a> = ();
    fn execute_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        stack.push_undo_action(|| self.clone());
        context.swap_or_panic(self.i, self.j)
    }
}
impl<T,Idx> UndoableAction for SwapIndex<T,Idx,policy::Unchecked> where for<'a> T: 'a + GetManyMut<Idx>, T::Output : Sized, Idx : Clone
{
    type Undo = Self;  type Context<'a>= T;  type Output<'a> = ();
    fn execute_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        stack.push_undo_action(|| self.clone());
        unsafe { context.swap_unchecked(self.i, self.j) }
    }
}

impl<T,Idx,P> Copy       for SwapIndex<T,Idx,P> where for<'a> T: 'a + GetManyMut<Idx>, T::Output : Sized, Idx : Clone, P : Policy, Idx : Copy       {}
impl<T,Idx,P> Clone      for SwapIndex<T,Idx,P> where for<'a> T: 'a + GetManyMut<Idx>, T::Output : Sized, Idx : Clone, P : Policy                   { fn clone(&self) -> Self { Self::new(self.i.clone(), self.j.clone()) } }
impl<T,Idx,P> PartialEq  for SwapIndex<T,Idx,P> where for<'a> T: 'a + GetManyMut<Idx>, T::Output : Sized, Idx : Clone, P : Policy, Idx : PartialEq  { fn eq(&self, other: &Self) -> bool { self.i == other.i && self.j == other.j } }
impl<T,Idx,P> Eq         for SwapIndex<T,Idx,P> where for<'a> T: 'a + GetManyMut<Idx>, T::Output : Sized, Idx : Clone, P : Policy, Idx : Eq         {}
impl<T,Idx,P> PartialOrd for SwapIndex<T,Idx,P> where for<'a> T: 'a + GetManyMut<Idx>, T::Output : Sized, Idx : Clone, P : Policy, Idx : PartialOrd { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { (&self.i, &self.j).partial_cmp(&(&other.i, &other.j)) } }
impl<T,Idx,P> Ord        for SwapIndex<T,Idx,P> where for<'a> T: 'a + GetManyMut<Idx>, T::Output : Sized, Idx : Clone, P : Policy, Idx : Ord        { fn cmp(&self, other: &Self) -> Ordering { (&self.i, &self.j).cmp(&(&other.i, &other.j)) } }
impl<T,Idx,P> Hash       for SwapIndex<T,Idx,P> where for<'a> T: 'a + GetManyMut<Idx>, T::Output : Sized, Idx : Clone, P : Policy, Idx : Hash       { fn hash<H: Hasher>(&self, state: &mut H) { self.i.hash(state); self.j.hash(state); } }
impl<T,Idx,P> Debug      for SwapIndex<T,Idx,P> where for<'a> T: 'a + GetManyMut<Idx>, T::Output : Sized, Idx : Clone, P : Policy, Idx : Debug      { fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}SwapIndex{:?}({:?},{:?})", P::DEBUG_PREFIX, P::DEBUG_SUFFIX, self.i, self.j) } }



pub struct Clear<T> where T : Clearable + Capacity + Length, <T as Capacity>::Param : Default
{ 
    phantom : PhantomData<T> 
}
impl<T>    Clear<T> where T : Clearable + Capacity + Length, <T as Capacity>::Param : Default { pub const fn new() -> Self { Self { phantom: PhantomData } }}

impl<T>    Copy       for Clear<T> where T : Clearable + Capacity + Length, <T as Capacity>::Param : Default {}
impl<T>    Clone      for Clear<T> where T : Clearable + Capacity + Length, <T as Capacity>::Param : Default { fn clone(&self) -> Self { Self { phantom: PhantomData } } }
impl<T>    PartialEq  for Clear<T> where T : Clearable + Capacity + Length, <T as Capacity>::Param : Default { fn eq(&self, _: &Self) -> bool { true } }
impl<T>    Eq         for Clear<T> where T : Clearable + Capacity + Length, <T as Capacity>::Param : Default {}
impl<T>    PartialOrd for Clear<T> where T : Clearable + Capacity + Length, <T as Capacity>::Param : Default { fn partial_cmp(&self, _: &Self) -> Option<Ordering> { Some(Ordering::Equal) } }
impl<T>    Ord        for Clear<T> where T : Clearable + Capacity + Length, <T as Capacity>::Param : Default { fn cmp(&self, _: &Self) -> Ordering { Ordering::Equal } }
impl<T>    Default    for Clear<T> where T : Clearable + Capacity + Length, <T as Capacity>::Param : Default { fn default() -> Self { Self{ phantom: PhantomData } } }
impl<T>    Hash       for Clear<T> where T : Clearable + Capacity + Length, <T as Capacity>::Param : Default { fn hash<H: Hasher>(&self, state: &mut H) { } }
impl<T>    Debug      for Clear<T> where T : Clearable + Capacity + Length, <T as Capacity>::Param : Default { fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "Clear") } }


impl<T> UndoableAction for Clear<T> where for<'a> T: 'a + Clearable + Capacity + Length, <T as Capacity>::Param : Default
{
    type Undo = Swap<T>;
    type Context<'a>= T;
    type Output<'a> = ();
    
    fn execute_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        Swap::new().execute_and_forget_in(&mut (&mut T::with_capacity(if S::LOG_UNDO { context.len() } else { 0 }), context), stack);
    }
}

/*
/// 2 distincts collection trade/swap one of their value
pub struct TradeIndex<T1,Idx1,T2,Idx2>
    where T1 : GetMut<Idx1>, Idx1 : Copy, T2 : GetMut<Idx2,Output = T1::Output>, Idx2 : Copy, T1::Output : Sized, for<'a> T1 : 'a, for<'a> T2 : 'a
{
    pub i : Idx1,
    pub j : Idx2, 
    phantom : PhantomData<(T1,T2)>
} 

impl<T1,Idx1,T2,Idx2> TradeIndex<T1,Idx1,T2,Idx2> where T1 : GetMut<Idx1>, Idx1 : Copy, T2 : GetMut<Idx2,Output = T1::Output>, Idx2 : Copy, T1::Output : Sized, for<'a> T1 : 'a, for<'a> T2 : 'a
{
    pub const fn new(i : Idx1, j : Idx2) -> Self { Self {i, j, phantom : PhantomData} }
    pub fn ij(self) -> (Idx1, Idx2) { (self.i, self.j) }
}

impl<T1,Idx1,T2,Idx2> Copy      for TradeIndex<T1,Idx1,T2,Idx2> where T1 : GetMut<Idx1>, Idx1 : Copy, T2 : GetMut<Idx2,Output = T1::Output>, Idx2 : Copy, T1::Output : Sized, for<'a> T1 : 'a, for<'a> T2 : 'a, Idx1 : Copy,      Idx2 : Copy       {}
impl<T1,Idx1,T2,Idx2> Clone     for TradeIndex<T1,Idx1,T2,Idx2> where T1 : GetMut<Idx1>, Idx1 : Copy, T2 : GetMut<Idx2,Output = T1::Output>, Idx2 : Copy, T1::Output : Sized, for<'a> T1 : 'a, for<'a> T2 : 'a, Idx1 : Clone,     Idx2 : Clone      { fn clone(&self) -> Self { Self::new(self.i.clone(), self.j.clone()) } }
impl<T1,Idx1,T2,Idx2> Eq        for TradeIndex<T1,Idx1,T2,Idx2> where T1 : GetMut<Idx1>, Idx1 : Copy, T2 : GetMut<Idx2,Output = T1::Output>, Idx2 : Copy, T1::Output : Sized, for<'a> T1 : 'a, for<'a> T2 : 'a, Idx1 : Eq,        Idx2 : Eq         {}
impl<T1,Idx1,T2,Idx2> PartialEq for TradeIndex<T1,Idx1,T2,Idx2> where T1 : GetMut<Idx1>, Idx1 : Copy, T2 : GetMut<Idx2,Output = T1::Output>, Idx2 : Copy, T1::Output : Sized, for<'a> T1 : 'a, for<'a> T2 : 'a, Idx1 : PartialEq, Idx2 : PartialEq  { fn eq(&self, other: &Self) -> bool { self.i == other.i && self.j == other.j } }
impl<T1,Idx1,T2,Idx2> Hash      for TradeIndex<T1,Idx1,T2,Idx2> where T1 : GetMut<Idx1>, Idx1 : Copy, T2 : GetMut<Idx2,Output = T1::Output>, Idx2 : Copy, T1::Output : Sized, for<'a> T1 : 'a, for<'a> T2 : 'a, Idx1 : Hash,      Idx2 : Hash       { fn hash<H: Hasher>(&self, state: &mut H) { self.i.hash(state); self.j.hash(state); } }
impl<T1,Idx1,T2,Idx2> Debug     for TradeIndex<T1,Idx1,T2,Idx2> where T1 : GetMut<Idx1>, Idx1 : Copy, T2 : GetMut<Idx2,Output = T1::Output>, Idx2 : Copy, T1::Output : Sized, for<'a> T1 : 'a, for<'a> T2 : 'a, Idx1 : Debug,     Idx2 : Debug      { fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { f.debug_tuple("Trade").field(&self.i).field(&&self.j).finish() } }

impl<T1,Idx1,T2,Idx2> UndoableAction for TradeIndex<T1,Idx1,T2,Idx2> where T1 : GetMut<Idx1>, Idx1 : Copy, T2 : GetMut<Idx2,Output = T1::Output>, Idx2 : Copy, T1::Output : Sized, for<'a> T1 : 'a, for<'a> T2 : 'a, Idx1 : Clone, Idx2 : Clone
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
pub struct SetIndex<T,Idx> where for<'a> T: 'a + GetMut<Idx>, T::Output : Sized + Clone
{
    pub idx   : Idx, 
    pub value : T::Output
}

pub trait SetIndexExtension<Idx> : Sized where for<'a> Self: 'a + GetMut<Idx>, Self::Output : Sized + Clone
{
    fn set_action<'a, S>(&'a mut self, value : Self::Output, idx : Idx, stack : &mut S) -> <SetIndex::<Self, Idx> as UndoableAction>::Output<'a> where S : UndoStack<<SetIndex<Self,Idx> as UndoableAction>::Undo>;
}
impl<T,Idx> SetIndexExtension<Idx> for T where for<'a> Self: 'a + GetMut<Idx>, Self::Output : Sized + Clone
{
    fn set_action<'a, S>(&'a mut self, value : Self::Output, idx : Idx, stack : &mut S) -> <SetIndex::<Self, Idx> as UndoableAction>::Output<'a> where S : UndoStack<<SetIndex<Self,Idx> as UndoableAction>::Undo> 
    { SetIndex::new(idx, value).execute_in(self, stack) }
}

impl<T,Idx> SetIndex<T,Idx> where for<'a> T: 'a + GetMut<Idx>, T::Output : Sized + Clone
{
    pub const fn new(idx : Idx, value : T::Output) -> Self { Self { idx, value } }
}

impl<T,Idx> Copy      for SetIndex<T,Idx> where for<'a> T: 'a + GetMut<Idx>, T::Output : Sized + Clone, Idx : Copy, T::Output : Copy {}
impl<T,Idx> Clone     for SetIndex<T,Idx> where for<'a> T: 'a + GetMut<Idx>, T::Output : Sized + Clone, Idx : Clone, T::Output : Clone { fn clone(&self) -> Self { Self::new(self.idx.clone(), self.value.clone()) } }
impl<T,Idx> Eq        for SetIndex<T,Idx> where for<'a> T: 'a + GetMut<Idx>, T::Output : Sized + Clone, Idx : Eq, T::Output : Eq {}
impl<T,Idx> PartialEq for SetIndex<T,Idx> where for<'a> T: 'a + GetMut<Idx>, T::Output : Sized + Clone, Idx : PartialEq, T::Output : PartialEq { fn eq(&self, other: &Self) -> bool { self.idx == other.idx && self.value == other.value } }
impl<T,Idx> Hash      for SetIndex<T,Idx> where for<'a> T: 'a + GetMut<Idx>, T::Output : Sized + Clone, Idx : Hash, T::Output : Hash { fn hash<H: Hasher>(&self, state: &mut H) { self.idx.hash(state); self.value.hash(state); } }
impl<T,Idx> Debug     for SetIndex<T,Idx> where for<'a> T: 'a + GetMut<Idx>, T::Output : Sized + Clone, Idx : Debug, T::Output : Debug { fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { f.debug_tuple("Set").field(&self.idx).field(&&self.value).finish() } }

impl<T,Idx> UndoableAction for SetIndex<T,Idx>  where for<'a> T: 'a + GetMut<Idx>, T::Output : Sized, Idx : Clone
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




pub struct ReplaceIndex<T,Idx> where for<'a> T: 'a + GetMut<Idx>, T::Output : Sized + Clone
{
    pub index   : Idx, 
    pub value : T::Output
}

impl<T,Idx> ReplaceIndex<T,Idx> where for<'a> T: 'a + GetMut<Idx>, T::Output : Sized + Clone
{
    pub const fn new(index : Idx, value : T::Output) -> Self { Self { index, value } }
}

impl<T,Idx> Copy      for ReplaceIndex<T,Idx> where for<'a> T: 'a + GetMut<Idx>, T::Output : Sized + Clone, T::Output : Copy {}
impl<T,Idx> Clone     for ReplaceIndex<T,Idx> where for<'a> T: 'a + GetMut<Idx>, T::Output : Sized + Clone, T::Output : Clone { fn clone(&self) -> Self { Self::new(self.index.clone(), self.value.clone()) } }
impl<T,Idx> Eq        for ReplaceIndex<T,Idx> where for<'a> T: 'a + GetMut<Idx>, T::Output : Sized + Clone, Idx : Eq, T::Output : Eq {}
impl<T,Idx> PartialEq for ReplaceIndex<T,Idx> where for<'a> T: 'a + GetMut<Idx>, T::Output : Sized + Clone, Idx : PartialEq, T::Output : PartialEq { fn eq(&self, other: &Self) -> bool { self.index == other.index && self.value == other.value } }
impl<T,Idx> Hash      for ReplaceIndex<T,Idx> where for<'a> T: 'a + GetMut<Idx>, T::Output : Sized + Clone, Idx : Hash, T::Output : Hash { fn hash<H: Hasher>(&self, state: &mut H) { self.index.hash(state); self.value.hash(state); } }
impl<T,Idx> Debug     for ReplaceIndex<T,Idx> where for<'a> T: 'a + GetMut<Idx>, T::Output : Sized + Clone, Idx : Debug, T::Output : Debug { fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "Replace([{:?}]=>{:?})", self.index, self.value) } }

impl<T,Idx> UndoableAction for ReplaceIndex<T,Idx>  where for<'a> T: 'a + GetMut<Idx>, T::Output : Sized + Clone
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