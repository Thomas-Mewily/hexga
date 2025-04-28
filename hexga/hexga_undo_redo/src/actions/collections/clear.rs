use super::*;


pub struct Clear<C> where C : Clearable + Capacity + Length, <C as Capacity>::Param : Default
{ 
    phantom : PhantomData<C> 
}
impl<C>    Clear<C> where C : Clearable + Capacity + Length, <C as Capacity>::Param : Default { pub const fn new() -> Self { Self { phantom: PhantomData } }}

impl<C>    Clone      for Clear<C> where C : Clearable + Capacity + Length, <C as Capacity>::Param : Default { fn clone(&self) -> Self { Self { phantom: PhantomData } } }
impl<C>    Copy       for Clear<C> where C : Clearable + Capacity + Length, <C as Capacity>::Param : Default {}
impl<C>    PartialEq  for Clear<C> where C : Clearable + Capacity + Length, <C as Capacity>::Param : Default { fn eq(&self, _: &Self) -> bool { true } }
impl<C>    Eq         for Clear<C> where C : Clearable + Capacity + Length, <C as Capacity>::Param : Default {}
impl<C>    PartialOrd for Clear<C> where C : Clearable + Capacity + Length, <C as Capacity>::Param : Default { fn partial_cmp(&self, _: &Self) -> Option<Ordering> { Some(Ordering::Equal) } }
impl<C>    Ord        for Clear<C> where C : Clearable + Capacity + Length, <C as Capacity>::Param : Default { fn cmp(&self, _: &Self) -> Ordering { Ordering::Equal } }
impl<C>    Hash       for Clear<C> where C : Clearable + Capacity + Length, <C as Capacity>::Param : Default { fn hash<H: Hasher>(&self, state: &mut H) { } }
impl<C>    Default    for Clear<C> where C : Clearable + Capacity + Length, <C as Capacity>::Param : Default { fn default() -> Self { Self{ phantom: PhantomData } } }
impl<C>    Debug      for Clear<C> where C : Clearable + Capacity + Length, <C as Capacity>::Param : Default { fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "Clear") } }


impl<C> UndoableAction for Clear<C> where for<'a> C: 'a + Clearable + Capacity + Length, <C as Capacity>::Param : Default
{
    type Undo = Swap<C>;
    type Context<'a>= C;
    type Output<'a> = ();
    
    fn execute_in<'a, S>(self, context : &mut Self::Context<'a>, stack : &mut S) -> Self::Output<'a> where S : UndoStack<Self::Undo> 
    {
        Swap::new().execute_and_forget_in(&mut (&mut C::with_capacity(if S::LOG_UNDO { context.len() } else { 0 }), context), stack);
    }
}