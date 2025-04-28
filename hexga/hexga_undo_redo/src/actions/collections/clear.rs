use super::*;


pub struct Clear<T> where T : Clearable + Capacity + Length, <T as Capacity>::Param : Default
{ 
    phantom : PhantomData<T> 
}
impl<T>    Clear<T> where T : Clearable + Capacity + Length, <T as Capacity>::Param : Default { pub const fn new() -> Self { Self { phantom: PhantomData } }}

impl<T>    Clone      for Clear<T> where T : Clearable + Capacity + Length, <T as Capacity>::Param : Default { fn clone(&self) -> Self { Self { phantom: PhantomData } } }
impl<T>    Copy       for Clear<T> where T : Clearable + Capacity + Length, <T as Capacity>::Param : Default {}
impl<T>    PartialEq  for Clear<T> where T : Clearable + Capacity + Length, <T as Capacity>::Param : Default { fn eq(&self, _: &Self) -> bool { true } }
impl<T>    Eq         for Clear<T> where T : Clearable + Capacity + Length, <T as Capacity>::Param : Default {}
impl<T>    PartialOrd for Clear<T> where T : Clearable + Capacity + Length, <T as Capacity>::Param : Default { fn partial_cmp(&self, _: &Self) -> Option<Ordering> { Some(Ordering::Equal) } }
impl<T>    Ord        for Clear<T> where T : Clearable + Capacity + Length, <T as Capacity>::Param : Default { fn cmp(&self, _: &Self) -> Ordering { Ordering::Equal } }
impl<T>    Hash       for Clear<T> where T : Clearable + Capacity + Length, <T as Capacity>::Param : Default { fn hash<H: Hasher>(&self, state: &mut H) { } }
impl<T>    Default    for Clear<T> where T : Clearable + Capacity + Length, <T as Capacity>::Param : Default { fn default() -> Self { Self{ phantom: PhantomData } } }
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