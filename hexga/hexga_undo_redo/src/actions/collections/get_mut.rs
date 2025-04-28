use super::*;

/* 
pub struct GetMut<T,Idx,P=policy::Normal> where for<'a> T: 'a + GetManyMut<Idx>, Idx : Clone, P : Policy
{
    pub index : Idx,
    phantom : PhantomData<(T,P)>
} 

impl<T,Idx,P> GetMut<T,Idx,P> where for<'a> T: 'a + GetManyMut<Idx>, Idx : Clone, P : Policy
{
    pub const fn new(index : Idx) -> Self { Self { index, phantom : PhantomData } }
    pub fn into_idx(self) -> Idx { self.index }
}


impl<T,Idx,P> Clone      for GetMut<T,Idx,P> where for<'a> T: 'a + GetManyMut<Idx>, Idx : Clone, P : Policy                   { fn clone(&self) -> Self { Self::new(self.index.clone()) } }
impl<T,Idx,P> Copy       for GetMut<T,Idx,P> where for<'a> T: 'a + GetManyMut<Idx>, Idx : Clone, P : Policy, Idx : Copy       {}
impl<T,Idx,P> PartialEq  for GetMut<T,Idx,P> where for<'a> T: 'a + GetManyMut<Idx>, Idx : Clone, P : Policy, Idx : PartialEq  { fn eq(&self, other: &Self) -> bool { self.index == other.index } }
impl<T,Idx,P> Eq         for GetMut<T,Idx,P> where for<'a> T: 'a + GetManyMut<Idx>, Idx : Clone, P : Policy, Idx : Eq         {}
impl<T,Idx,P> PartialOrd for GetMut<T,Idx,P> where for<'a> T: 'a + GetManyMut<Idx>, Idx : Clone, P : Policy, Idx : PartialOrd { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { self.index.partial_cmp(&other.index) } }
impl<T,Idx,P> Ord        for GetMut<T,Idx,P> where for<'a> T: 'a + GetManyMut<Idx>, Idx : Clone, P : Policy, Idx : Ord        { fn cmp(&self, other: &Self) -> Ordering { self.index.cmp(&other.index) } }
impl<T,Idx,P> Hash       for GetMut<T,Idx,P> where for<'a> T: 'a + GetManyMut<Idx>, Idx : Clone, P : Policy, Idx : Hash       { fn hash<H: Hasher>(&self, state: &mut H) { self.index.hash(state); } }
impl<T,Idx,P> Debug      for GetMut<T,Idx,P> where for<'a> T: 'a + GetManyMut<Idx>, Idx : Clone, P : Policy, Idx : Debug      { fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}GetMut{:?}[{:?}]", P::DEBUG_PREFIX, P::DEBUG_SUFFIX, self.index) } }


impl<T,Idx> UndoableAction for GetMut<T,Idx,policy::Try> where for<'a> T: 'a + GetManyMut<Idx>, Idx : Clone
{
    type Undo = Self;   type Context<'a>= T;  type Output<'a> = Result<T::Output, ()>;
    fn execute_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        stack.push_undo_action(|| self.clone());
        context.try_replace(self.index, self.value)
    }
}
impl<T,Idx> UndoableAction for GetMut<T,Idx,policy::Normal> where for<'a> T: 'a + GetManyMut<Idx>, Idx : Clone
{
    type Undo = Self;   type Context<'a>= T;  type Output<'a> = Option<T::Output>;
    fn execute_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        stack.push_undo_action(|| self.clone());
        context.replace(self.index, self.value)
    }
}
impl<T,Idx> UndoableAction for GetMut<T,Idx,policy::Panic> where for<'a> T: 'a + GetManyMut<Idx>, Idx : Clone
{
    type Undo = Self;   type Context<'a>= T;  type Output<'a> = T::Output;
    fn execute_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        stack.push_undo_action(|| self.clone());
        context.replace_or_panic(self.index, self.value)
    }
}
impl<T,Idx> UndoableAction for GetMut<T,Idx,policy::Unchecked> where for<'a> T: 'a + GetManyMut<Idx>, Idx : Clone
{
    type Undo = Self;   type Context<'a>= T;  type Output<'a> = T::Output;
    fn execute_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        stack.push_undo_action(|| self.clone());
        unsafe { context.replace_unchecked(self.index, self.value) }
    }
}
*/