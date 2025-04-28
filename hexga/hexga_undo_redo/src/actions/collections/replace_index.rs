use super::*;


pub struct ReplaceIndex<T,Idx,P=policy::Normal> where for<'a> T: 'a + GetManyMut<Idx>, T::Output : Sized + Clone, Idx : Clone, P : Policy
{   
    pub index : Idx,
    pub value : T::Output,
    phantom : PhantomData<(T,P)>
} 

impl<T,Idx,P> ReplaceIndex<T,Idx,P> where for<'a> T: 'a + GetManyMut<Idx>, T::Output : Sized + Clone, Idx : Clone, P : Policy
{
    pub const fn new(idx : Idx, value : T::Output) -> Self { Self { index: idx, value, phantom : PhantomData } }
    pub fn into_idx_value(self) -> (Idx, T::Output) { let Self { index: idx, value, phantom : _ } = self; (idx, value) }
}


impl<T,Idx,P> Clone      for ReplaceIndex<T,Idx,P> where for<'a> T: 'a + GetManyMut<Idx>, T::Output : Sized + Clone, Idx : Clone, P : Policy             { fn clone(&self) -> Self { Self::new(self.index.clone(), self.value.clone()) } }
impl<T,Idx,P> Copy       for ReplaceIndex<T,Idx,P> where for<'a> T: 'a + GetManyMut<Idx>, T::Output : Sized + Clone, Idx : Clone, P : Policy, Idx : Copy, T::Output : Copy  {}
impl<T,Idx,P> PartialEq  for ReplaceIndex<T,Idx,P> where for<'a> T: 'a + GetManyMut<Idx>, T::Output : Sized + Clone, Idx : Clone, P : Policy, Idx : PartialEq, T::Output : PartialEq               { fn eq(&self, other: &Self) -> bool { self.value == other.value && self.value == other.value } }
impl<T,Idx,P> Eq         for ReplaceIndex<T,Idx,P> where for<'a> T: 'a + GetManyMut<Idx>, T::Output : Sized + Clone, Idx : Clone, P : Policy, Idx : Eq, T::Output : Eq         {}
impl<T,Idx,P> PartialOrd for ReplaceIndex<T,Idx,P> where for<'a> T: 'a + GetManyMut<Idx>, T::Output : Sized + Clone, Idx : Clone, P : Policy, Idx : PartialOrd, T::Output : PartialOrd { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { (&self.value, &self.value).partial_cmp(&(&other.value, &other.value)) } }
impl<T,Idx,P> Ord        for ReplaceIndex<T,Idx,P> where for<'a> T: 'a + GetManyMut<Idx>, T::Output : Sized + Clone, Idx : Clone, P : Policy, Idx : Ord, T::Output : Ord        { fn cmp(&self, other: &Self) -> Ordering { (&self.index, &self.value).cmp(&(&other.index, &other.value)) } }
impl<T,Idx,P> Hash       for ReplaceIndex<T,Idx,P> where for<'a> T: 'a + GetManyMut<Idx>, T::Output : Sized + Clone, Idx : Clone, P : Policy, Idx : Hash, T::Output : Hash       { fn hash<H: Hasher>(&self, state: &mut H) { self.value.hash(state); self.value.hash(state); } }
impl<T,Idx,P> Debug      for ReplaceIndex<T,Idx,P> where for<'a> T: 'a + GetManyMut<Idx>, T::Output : Sized + Clone, Idx : Clone, P : Policy, Idx : Debug, T::Output : Debug      { fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}ReplaceIndex{:?}[{:?}] => {:?}", P::DEBUG_PREFIX, P::DEBUG_SUFFIX, self.index, self.value) } }


impl<T,Idx> UndoableAction for ReplaceIndex<T,Idx,policy::Try> where for<'a> T: 'a + GetManyMut<Idx>, T::Output : Sized + Clone, Idx : Clone
{
    type Undo = Self;   type Context<'a>= T;  type Output<'a> = Result<T::Output, ()>;
    fn execute_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        stack.push_undo_action(|| self.clone());
        context.try_replace(self.index, self.value)
    }
}
impl<T,Idx> UndoableAction for ReplaceIndex<T,Idx,policy::Normal> where for<'a> T: 'a + GetManyMut<Idx>, T::Output : Sized + Clone, Idx : Clone
{
    type Undo = Self;   type Context<'a>= T;  type Output<'a> = Option<T::Output>;
    fn execute_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        stack.push_undo_action(|| self.clone());
        context.replace(self.index, self.value)
    }
}
impl<T,Idx> UndoableAction for ReplaceIndex<T,Idx,policy::Panic> where for<'a> T: 'a + GetManyMut<Idx>, T::Output : Sized + Clone, Idx : Clone
{
    type Undo = Self;   type Context<'a>= T;  type Output<'a> = T::Output;
    fn execute_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        stack.push_undo_action(|| self.clone());
        context.replace_or_panic(self.index, self.value)
    }
}
impl<T,Idx> UndoableAction for ReplaceIndex<T,Idx,policy::Unchecked> where for<'a> T: 'a + GetManyMut<Idx>, T::Output : Sized + Clone, Idx : Clone
{
    type Undo = Self;   type Context<'a>= T;  type Output<'a> = T::Output;
    fn execute_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        stack.push_undo_action(|| self.clone());
        unsafe { context.replace_unchecked(self.index, self.value) }
    }
}