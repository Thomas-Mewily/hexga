use super::*;


// ToOwned, that way SwapIndex will also work on HashMap<String, Foo> indexed by &str (index stored as String), then it can be acceded using the borrow version of the String : &str
pub struct ReplaceIndex<C,Idx,P=policy::Normal> where C: GetMut<Idx>, C::Output : Sized + Clone, P : Policy
    /* 
    Idx : ToOwned, Idx::Owned : Clone,
    C: GetMut<Idx> + GetMut<Idx::Owned,Output=<C as Get<Idx>>::Output>, 
    <C as Get<Idx>>::Output : Sized + Clone, 
    P : Policy
    */
{   
    pub index : Idx,
    pub value : C::Output,
    phantom : PhantomData<(C,P)>
} 

impl<C,Idx,P> ReplaceIndex<C,Idx,P> where C: GetMut<Idx>, C::Output : Sized + Clone, P : Policy
{
    pub fn new(index : Idx, value : C::Output) -> Self { Self { index : index, value, phantom : PhantomData } }
    pub fn into_idx_value(self) -> (Idx, C::Output) { let Self { index, value, phantom : _ } = self; (index, value) }
}

/* 
impl<'a,C,Idx> UndoableAction for ReplaceIndex<C,Idx,policy::Try> 
    where C: GetMut<Idx> + GetMut<Idx::Owned,Output=<C as Get<Idx>>::Output>, <C as Get<Idx>>::Output : Sized + Clone,
    Idx : ToOwned, Idx::Owned : Clone,
    ReplaceIndex<C,Idx::Owned,policy::Try> : UndoableAction<Undo = ReplaceIndex<C,Idx::Owned,policy::Try>, Context<'a> = Self::Context<'a>, Output<'a> = Self::Output<'a>>
{
    type Undo = ReplaceIndex<C,Idx::Owned,policy::Try>; 
    type Context<'b>= C;  
    type Output<'b> = Result<<C as Get<Idx>>::Output, ()>;
    fn execute_in<'b, S>(self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        stack.push_undo_action(|| ReplaceIndex::new(self.index.to_owned(), self.value));
        context.try_replace(self.index.borrow(), self.value)
    }
}
    */

/* 


impl<C,B,Idx,P> Clone      for ReplaceIndex<C,B,Idx,P> where for<'a> C: 'a + GetMut<Idx> + GetMut<Idx::Owned,Output=C>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Clone, P : Policy                                                  { fn clone(&self) -> Self { Self { index: self.index.clone(), value: self.value.clone(), phantom: PhantomData } } }
impl<C,B,Idx,P> Copy       for ReplaceIndex<C,B,Idx,P> where for<'a> C: 'a + GetMut<Idx> + GetMut<Idx::Owned,Output=C>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Clone, P : Policy, Idx::Owned : Copy, C::Output : Copy             {}
impl<C,B,Idx,P> PartialEq  for ReplaceIndex<C,B,Idx,P> where for<'a> C: 'a + GetMut<Idx> + GetMut<Idx::Owned,Output=C>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Clone, P : Policy, Idx::Owned : PartialEq, C::Output : PartialEq   { fn eq(&self, other: &Self) -> bool { self.value == other.value && self.value == other.value } }
impl<C,B,Idx,P> Eq         for ReplaceIndex<C,B,Idx,P> where for<'a> C: 'a + GetMut<Idx> + GetMut<Idx::Owned,Output=C>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Clone, P : Policy, Idx::Owned : Eq, C::Output : Eq                 {}
impl<C,B,Idx,P> PartialOrd for ReplaceIndex<C,B,Idx,P> where for<'a> C: 'a + GetMut<Idx> + GetMut<Idx::Owned,Output=C>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Clone, P : Policy, Idx::Owned : PartialOrd, C::Output : PartialOrd { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { (&self.value, &self.value).partial_cmp(&(&other.value, &other.value)) } }
impl<C,B,Idx,P> Ord        for ReplaceIndex<C,B,Idx,P> where for<'a> C: 'a + GetMut<Idx> + GetMut<Idx::Owned,Output=C>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Clone, P : Policy, Idx::Owned : Ord, C::Output : Ord               { fn cmp(&self, other: &Self) -> Ordering { (&self.index, &self.value).cmp(&(&other.index, &other.value)) } }
impl<C,B,Idx,P> Hash       for ReplaceIndex<C,B,Idx,P> where for<'a> C: 'a + GetMut<Idx> + GetMut<Idx::Owned,Output=C>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Clone, P : Policy, Idx::Owned : Hash, C::Output : Hash             { fn hash<H: Hasher>(&self, state: &mut H) { self.value.hash(state); self.value.hash(state); } }
impl<C,B,Idx,P> Debug      for ReplaceIndex<C,B,Idx,P> where for<'a> C: 'a + GetMut<Idx> + GetMut<Idx::Owned,Output=C>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Clone, P : Policy, Idx::Owned : Debug, C::Output : Debug           { fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}ReplaceIndex{:?}[{:?}] => {:?}", P::DEBUG_PREFIX, P::DEBUG_SUFFIX, self.index, self.value) } }


impl<C,B,Idx> UndoableAction for ReplaceIndex<C,B,Idx,policy::Try> where for<'a> C: 'a + GetMut<Idx> + GetMut<Idx::Owned,Output=C>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Clone
{
    type Undo = Self;   type Context<'a>= C;  type Output<'a> = Result<C::Output, ()>;
    fn execute_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        stack.push_undo_action(|| self.clone());
        context.try_replace(self.index.borrow(), self.value)
    }
}
impl<C,B,Idx> UndoableAction for ReplaceIndex<C,B,Idx,policy::Normal> where for<'a> C: 'a + GetMut<Idx> + GetMut<Idx::Owned,Output=C>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Clone
{
    type Undo = Self;   type Context<'a>= C;  type Output<'a> = Option<C::Output>;
    fn execute_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        stack.push_undo_action(|| self.clone());
        context.replace(self.index, self.value)
    }
}
impl<C,B,Idx> UndoableAction for ReplaceIndex<C,B,Idx,policy::Panic> where for<'a> C: 'a + GetMut<Idx> + GetMut<Idx::Owned,Output=C>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Clone
{
    type Undo = Self;   type Context<'a>= C;  type Output<'a> = C::Output;
    fn execute_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        stack.push_undo_action(|| self.clone());
        context.replace_or_panic(self.index, self.value)
    }
}
impl<C,B,Idx> UndoableAction for ReplaceIndex<C,B,Idx,policy::Unchecked> where for<'a> C: 'a + GetMut<Idx> + GetMut<Idx::Owned,Output=C>, C::Output : Sized + Clone, Idx : ToOwned, Idx::Owned : Clone
{
    type Undo = Self;   type Context<'a>= C;  type Output<'a> = C::Output;
    fn execute_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        stack.push_undo_action(|| self.clone());
        unsafe { context.replace_unchecked(self.index, self.value) }
    }
}
*/