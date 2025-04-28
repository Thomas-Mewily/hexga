use super::*;


pub struct SwapIndex<C,Idx,P=policy::Normal> where for<'a> C: 'a + GetManyMut<Idx>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Clone, P : Policy
{   
    pub i: Idx::Owned,
    pub j: Idx::Owned, 
    phantom : PhantomData<(C,P)>
}

impl<C,Idx,P> SwapIndex<C,Idx,P> where for<'a> C: 'a + GetManyMut<Idx>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Clone, P : Policy
{
    pub const fn new(i : &Idx, j : &Idx) -> Self { Self{ i : i.to_owned(), j : j.to_owned(), phantom: PhantomData } }
    pub fn ij(self) -> (Idx::Owned, Idx::Owned) { (self.i, self.j) }
}


impl<C,Idx,P> Clone      for SwapIndex<C,Idx,P> where for<'a> C: 'a + GetManyMut<Idx>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Clone, P : Policy                          { fn clone(&self) -> Self { Self { i: self.i.clone(), j: self.j.clone(), phantom: PhantomData } } }
impl<C,Idx,P> Copy       for SwapIndex<C,Idx,P> where for<'a> C: 'a + GetManyMut<Idx>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Clone, P : Policy, Idx::Owned : Copy       {}
impl<C,Idx,P> PartialEq  for SwapIndex<C,Idx,P> where for<'a> C: 'a + GetManyMut<Idx>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Clone, P : Policy, Idx::Owned : PartialEq  { fn eq(&self, other: &Self) -> bool { self.i == other.i && self.j == other.j } }
impl<C,Idx,P> Eq         for SwapIndex<C,Idx,P> where for<'a> C: 'a + GetManyMut<Idx>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Clone, P : Policy, Idx::Owned : Eq         {}
impl<C,Idx,P> PartialOrd for SwapIndex<C,Idx,P> where for<'a> C: 'a + GetManyMut<Idx>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Clone, P : Policy, Idx::Owned : PartialOrd { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { (&self.i, &self.j).partial_cmp(&(&other.i, &other.j)) } }
impl<C,Idx,P> Ord        for SwapIndex<C,Idx,P> where for<'a> C: 'a + GetManyMut<Idx>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Clone, P : Policy, Idx::Owned : Ord        { fn cmp(&self, other: &Self) -> Ordering { (&self.i, &self.j).cmp(&(&other.i, &other.j)) } }
impl<C,Idx,P> Hash       for SwapIndex<C,Idx,P> where for<'a> C: 'a + GetManyMut<Idx>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Clone, P : Policy, Idx::Owned : Hash       { fn hash<H: Hasher>(&self, state: &mut H) { self.i.hash(state); self.j.hash(state); } }
impl<C,Idx,P> Debug      for SwapIndex<C,Idx,P> where for<'a> C: 'a + GetManyMut<Idx>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Clone, P : Policy, Idx::Owned : Debug      { fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}SwapIndex{:?}({:?},{:?})", P::DEBUG_PREFIX, P::DEBUG_SUFFIX, self.i, self.j) } }


impl<C,Idx> UndoableAction for SwapIndex<C,Idx,policy::Try> where for<'a> C: 'a + GetManyMut<Idx>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Clone, P : Policy
{
    type Undo = Self;   type Context<'a>= C;  type Output<'a> = Result<(), ()>;
    fn execute_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        stack.push_undo_action(|| self.clone());
        context.try_swap(self.i, self.j)
    }
}
impl<C,Idx> UndoableAction for SwapIndex<C,Idx,policy::Normal> where for<'a> C: 'a + GetManyMut<Idx>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Clone, P : Policy
{
    type Undo = Self;  type Context<'a>= C;  type Output<'a> = bool;
    fn execute_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        stack.push_undo_action(|| self.clone());
        context.swap(self.i, self.j)
    }
}
impl<C,Idx> UndoableAction for SwapIndex<C,Idx,policy::Panic> where for<'a> C: 'a + GetManyMut<Idx>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Clone, P : Policy
{
    type Undo = Self;  type Context<'a>= C;  type Output<'a> = ();
    fn execute_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        stack.push_undo_action(|| self.clone());
        context.swap_or_panic(self.i, self.j)
    }
}
impl<C,Idx> UndoableAction for SwapIndex<C,Idx,policy::Unchecked> where for<'a> C: 'a + GetManyMut<Idx>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Clone, P : Policy
{
    type Undo = Self;  type Context<'a>= C;  type Output<'a> = ();
    fn execute_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        stack.push_undo_action(|| self.clone());
        unsafe { context.swap_unchecked(self.i, self.j) }
    }
}
