use super::*;

// ToOwned, that way SwapIndex will also work on HashMap<String, Foo> indexed by &str (index stored as String), then it can be acceded using the borrow version of the String : &str
pub struct ReplaceIndex<C,B,Idx,P=policy::Normal> where for<'a> C: 'a + GetMut<Idx>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Borrow<Idx>, Idx::Owned : Clone, P : Policy
{   
    pub index : Idx::Owned,
    pub value : C::Output,
    phantom : PhantomData<(C,B,P)>
} 

impl<C,B,Idx,P> ReplaceIndex<C,B,Idx,P> where for<'a> C: 'a + GetMut<Idx>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Borrow<Idx>, Idx::Owned : Clone, P : Policy
{
    pub fn new(index : &Idx, value : C::Output) -> Self { Self { index : index.to_owned(), value, phantom : PhantomData } }
    pub fn into_idx_value(self) -> (Idx::Owned, C::Output) { let Self { index, value, phantom : _ } = self; (index, value) }
}


impl<C,B,Idx,P> Clone      for ReplaceIndex<C,B,Idx,P> where for<'a> C: 'a + GetMut<Idx>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Borrow<Idx>, Idx::Owned : Clone, P : Policy                                                  { fn clone(&self) -> Self { Self { index: self.index.clone(), value: self.value.clone(), phantom: PhantomData } } }
impl<C,B,Idx,P> Copy       for ReplaceIndex<C,B,Idx,P> where for<'a> C: 'a + GetMut<Idx>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Borrow<Idx>, Idx::Owned : Clone, P : Policy, Idx::Owned : Copy, C::Output : Copy             {}
impl<C,B,Idx,P> PartialEq  for ReplaceIndex<C,B,Idx,P> where for<'a> C: 'a + GetMut<Idx>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Borrow<Idx>, Idx::Owned : Clone, P : Policy, Idx::Owned : PartialEq, C::Output : PartialEq   { fn eq(&self, other: &Self) -> bool { self.value == other.value && self.value == other.value } }
impl<C,B,Idx,P> Eq         for ReplaceIndex<C,B,Idx,P> where for<'a> C: 'a + GetMut<Idx>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Borrow<Idx>, Idx::Owned : Clone, P : Policy, Idx::Owned : Eq, C::Output : Eq                 {}
impl<C,B,Idx,P> PartialOrd for ReplaceIndex<C,B,Idx,P> where for<'a> C: 'a + GetMut<Idx>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Borrow<Idx>, Idx::Owned : Clone, P : Policy, Idx::Owned : PartialOrd, C::Output : PartialOrd { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { (&self.value, &self.value).partial_cmp(&(&other.value, &other.value)) } }
impl<C,B,Idx,P> Ord        for ReplaceIndex<C,B,Idx,P> where for<'a> C: 'a + GetMut<Idx>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Borrow<Idx>, Idx::Owned : Clone, P : Policy, Idx::Owned : Ord, C::Output : Ord               { fn cmp(&self, other: &Self) -> Ordering { (&self.index, &self.value).cmp(&(&other.index, &other.value)) } }
impl<C,B,Idx,P> Hash       for ReplaceIndex<C,B,Idx,P> where for<'a> C: 'a + GetMut<Idx>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Borrow<Idx>, Idx::Owned : Clone, P : Policy, Idx::Owned : Hash, C::Output : Hash             { fn hash<H: Hasher>(&self, state: &mut H) { self.value.hash(state); self.value.hash(state); } }
impl<C,B,Idx,P> Debug      for ReplaceIndex<C,B,Idx,P> where for<'a> C: 'a + GetMut<Idx>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Borrow<Idx>, Idx::Owned : Clone, P : Policy, Idx::Owned : Debug, C::Output : Debug           { fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}ReplaceIndex{:?}[{:?}] => {:?}", P::DEBUG_PREFIX, P::DEBUG_SUFFIX, self.index, self.value) } }


impl<C,B,Idx> UndoableAction for ReplaceIndex<C,B,Idx,policy::Try> where for<'a> C: 'a + GetMut<Idx>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Borrow<Idx>, Idx::Owned : Clone
{
    type Undo = Self;   type Context<'a>= C;  type Output<'a> = Result<C::Output, ()>;
    fn execute_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        stack.push_undo_action(|| self.clone());
        context.try_replace(self.index.borrow(), self.value)
    }
}
impl<C,B,Idx> UndoableAction for ReplaceIndex<C,B,Idx,policy::Normal> where for<'a> C: 'a + GetMut<Idx>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Borrow<Idx>, Idx::Owned : Clone
{
    type Undo = Self;   type Context<'a>= C;  type Output<'a> = Option<C::Output>;
    fn execute_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        stack.push_undo_action(|| self.clone());
        context.replace(self.index, self.value)
    }
}
impl<C,B,Idx> UndoableAction for ReplaceIndex<C,B,Idx,policy::Panic> where for<'a> C: 'a + GetMut<Idx>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Borrow<Idx>, Idx::Owned : Clone
{
    type Undo = Self;   type Context<'a>= C;  type Output<'a> = C::Output;
    fn execute_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        stack.push_undo_action(|| self.clone());
        context.replace_or_panic(self.index, self.value)
    }
}
impl<C,B,Idx> UndoableAction for ReplaceIndex<C,B,Idx,policy::Unchecked> where for<'a> C: 'a + GetMut<Idx>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Borrow<Idx>, Idx::Owned : Clone
{
    type Undo = Self;   type Context<'a>= C;  type Output<'a> = C::Output;
    fn execute_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        stack.push_undo_action(|| self.clone());
        unsafe { context.replace_unchecked(self.index, self.value) }
    }
}