use super::*;


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


impl<T,Idx,P> Clone      for SwapIndex<T,Idx,P> where for<'a> T: 'a + GetManyMut<Idx>, T::Output : Sized, Idx : Clone, P : Policy                   { fn clone(&self) -> Self { Self::new(self.i.clone(), self.j.clone()) } }
impl<T,Idx,P> Copy       for SwapIndex<T,Idx,P> where for<'a> T: 'a + GetManyMut<Idx>, T::Output : Sized, Idx : Clone, P : Policy, Idx : Copy       {}
impl<T,Idx,P> PartialEq  for SwapIndex<T,Idx,P> where for<'a> T: 'a + GetManyMut<Idx>, T::Output : Sized, Idx : Clone, P : Policy, Idx : PartialEq  { fn eq(&self, other: &Self) -> bool { self.i == other.i && self.j == other.j } }
impl<T,Idx,P> Eq         for SwapIndex<T,Idx,P> where for<'a> T: 'a + GetManyMut<Idx>, T::Output : Sized, Idx : Clone, P : Policy, Idx : Eq         {}
impl<T,Idx,P> PartialOrd for SwapIndex<T,Idx,P> where for<'a> T: 'a + GetManyMut<Idx>, T::Output : Sized, Idx : Clone, P : Policy, Idx : PartialOrd { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { (&self.i, &self.j).partial_cmp(&(&other.i, &other.j)) } }
impl<T,Idx,P> Ord        for SwapIndex<T,Idx,P> where for<'a> T: 'a + GetManyMut<Idx>, T::Output : Sized, Idx : Clone, P : Policy, Idx : Ord        { fn cmp(&self, other: &Self) -> Ordering { (&self.i, &self.j).cmp(&(&other.i, &other.j)) } }
impl<T,Idx,P> Hash       for SwapIndex<T,Idx,P> where for<'a> T: 'a + GetManyMut<Idx>, T::Output : Sized, Idx : Clone, P : Policy, Idx : Hash       { fn hash<H: Hasher>(&self, state: &mut H) { self.i.hash(state); self.j.hash(state); } }
impl<T,Idx,P> Debug      for SwapIndex<T,Idx,P> where for<'a> T: 'a + GetManyMut<Idx>, T::Output : Sized, Idx : Clone, P : Policy, Idx : Debug      { fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}SwapIndex{:?}({:?},{:?})", P::DEBUG_PREFIX, P::DEBUG_SUFFIX, self.i, self.j) } }


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
