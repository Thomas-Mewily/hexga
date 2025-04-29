use crate::*;
use crate::actions::mem::*;
use crate::actions::collections::*;

/// What is the equivalent relative action when the action take place a collection.
/// ex : `x.replace_action(42, &mut stack)`
/// If `x` was a mutable reference to a collection, we need to track where do `x` come from :
/// 
/// ```rust
/// let mut stack = Vec::new();
/// let s = &mut stack;
/// 
/// let array = [1,2,3];
/// let x : &mut i32 = array.get_mut_or_panic(1);
/// 
/// x.replace_action(42, s); // <- We don't know where to x come from
/// ```
/// 
/// But the `replace_action` itself don't know the borrow value was relative to array
/// 
/// ```rust
/// let mut stack = Vec::new();
/// let s = &mut stack;
/// 
/// let array = [1,2,3];
/// array.get_mut_or_panic_action(1,s, |value, tmp_stack| value.replace_action(42, tmp_stack)); // Ok
/// ```
/// 
/// Here, the `actions::mem::Replace` will be related to the borrowed value, thus becoming :
/// `actions::mem::Replace` -> `actions::mem::ReplaceIndex`

/* 
pub trait Relative<C,B,Idx,P> : Action where C: GetMut<Idx>, Idx : ToOwned + Borrow<B>, Idx::Owned : Clone
{
    type Relative<'a> : Action; //<Context<'a> = Self::Context<'a>>;
    fn relative<'a>(self, index : &Idx) -> Self::Relative<'a>; 
}
*/

/* 
impl<C,B,Idx> Relative<C,B,Idx,policy::Normal> for Replace<Idx> where for<'a> C: 'a + GetMut<Idx::Owned>, Idx : ToOwned + Borrow<B>, Idx::Owned : Clone
{
    type Relative<'a> = ReplaceIndex<C,Idx,policy::Normal>;
    fn relative<'a>(self, index : &Idx) -> Self::Relative<'a> {
        //SwapIndex::new(i, j)
    }
}
*/


//impl<Idx> Relative<Idx> for : Action where for<'a> Self::Context<'a> : GetMut<Idx>

/// Memorize where do the context/value come from using it's index
#[derive(Debug)]
pub struct ActionStackRelative<'a, S, A, C, Idx, P> where S : UndoStack<A>, A: UndoableAction, C: GetMut<Idx>, Idx: Clone, P: Policy
{
    stack : &'a mut S,
    collection : &'a mut C,
    index : Idx,
    phantom : PhantomData<(A,P)>,
}

impl<'a, S, A, C, Idx, P> ActionStackRelative<'a, S, A, C, Idx, P> where S : UndoStack<A>, A: UndoableAction, C: GetMut<Idx>, Idx: Clone, P: Policy
{
    pub fn new(stack : &'a mut S, collection : &'a mut C, index : Idx) -> Self { Self { stack, collection, index, phantom : PhantomData }}
}

/*
impl<'a, U, A, T, Idx> UndoStack<T> for ActionStackRelative<'a, U, A, T, Idx> where U : UndoStack<A>, A : UndoableAction, T : GetMut<Idx>, Idx : Clone, P: Policy
{
    const LOG_UNDO : bool = U::LOG_UNDO;

    fn push_undo_action<F>(&mut self, f : F) where F : FnOnce() -> T
    {
        self.undo.push_undo_action(|| (self.f)(f()));
    }
    
    fn stack_undo_in<Dest>(&mut self, _ : &mut <T as UndoableAction>::Context<'_>, _ : &mut Dest) -> bool where Dest : UndoStack<T::Undo> { false }
    
    fn prepare(&mut self) { self.undo.prepare(); }
}

/// Ignore the action
impl<A> UndoStack<A> for () where A : UndoableAction 
{
    const LOG_UNDO : bool = false;
    fn push_undo_action<F>(&mut self, _ : F) where F : FnOnce() -> A {}
    fn stack_undo_in<Dest>(&mut self, _ : &mut <A as UndoableAction>::Context<'_>, _ : &mut Dest) -> bool where Dest : UndoStack<A::Undo> { false }
    fn prepare(&mut self) {}
}

// Todo impl it for sequence that support push ?
impl<A> UndoStack<A> for Vec<A> where A : UndoableAction
{
    const LOG_UNDO : bool = true;
    fn push_undo_action<F>(&mut self, f : F) where F : FnOnce() -> A {
        self.push(f());
    }
    
    fn stack_undo_in<Dest>(&mut self, ctx : &mut <A as UndoableAction>::Context<'_>, dest : &mut Dest) -> bool where Dest : UndoStack<A::Undo> {
        dest.prepare();
        self.pop().map(|v| v.execute_and_forget_in(ctx, dest)).is_some()
    }

    fn prepare(&mut self) {}
}
*/