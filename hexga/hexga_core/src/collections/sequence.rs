use crate::*;

/* 
pub trait Collection : Length /* + Capacity */ {}  // slice and str don't impl capacity
impl<T : ?Sized> Collection for T where T : Length {}
*/

/// A special kind of collection where the order of insertion / deletion is conserved with index and pop()
pub trait Sequence {}

// IntoIterator<Item=T>

impl<T> Sequence for Vec<T> {}
impl<T> Sequence for [T] {}

impl<T> Sequence for VecDeque<T> {}
impl<T> Sequence for LinkedList<T> {}

impl Sequence for String {}
impl Sequence for str {}

impl Sequence for OsString {}
impl Sequence for OsStr {}

pub trait SequencePush<T> : Sequence
{
    type Output<'a> where Self: 'a;
    fn push<'a>(&'a mut self, value : T) -> Self::Output<'a>;

    type Error;
    fn try_push<'a>(&'a mut self, value : T) -> Result<Self::Output<'a>, Self::Error>;
}

impl<T> SequencePush<T> for Vec<T>
{
    type Output<'a> = () where Self: 'a;
    fn push<'a>(&'a mut self, value : T) -> Self::Output<'a> {
        self.push(value);
    }
    
    type Error = (); // #proper_error
    fn try_push<'a>(&'a mut self, value : T) -> Result<Self::Output<'a>, Self::Error> {
        self.push(value);
        Ok(())
    }
}

impl<T> SequencePush<T> for VecDeque<T>
{
    type Output<'a> = () where Self: 'a;
    fn push<'a>(&'a mut self, value : T) -> Self::Output<'a> {
        self.push_back(value);
    }
    
    type Error = (); //// #proper_error
    fn try_push<'a>(&'a mut self, value : T) -> Result<Self::Output<'a>, Self::Error> {
        self.push(value);
        Ok(())
    }
}

impl<T> SequencePush<T> for LinkedList<T>
{
    type Output<'a> = () where Self: 'a;
    fn push<'a>(&'a mut self, value : T) -> Self::Output<'a> {
        self.push_back(value);
    }
    
    type Error = (); // // #proper_error
    fn try_push<'a>(&'a mut self, value : T) -> Result<Self::Output<'a>, Self::Error> {
        self.push(value);
        Ok(())
    }
}

impl SequencePush<char> for String
{
    type Output<'a> = () where Self: 'a;
    fn push<'a>(&'a mut self, value : char) -> Self::Output<'a> {
        self.push(value);
    }
    
    type Error = ();
    fn try_push<'a>(&'a mut self, value : char) -> Result<Self::Output<'a>, Self::Error> {
        self.push(value);
        Ok(())
    }
}

impl<'b> SequencePush<&'b OsStr> for OsString
{
    type Output<'a> = () where Self: 'a;
    fn push<'a>(&'a mut self, value : &'b OsStr) -> Self::Output<'a> {
        self.push(value);
    }
    
    type Error = ();
    fn try_push<'a>(&'a mut self, value : &'b OsStr) -> Result<Self::Output<'a>, Self::Error> {
        self.push(value);
        Ok(())
    }
}


pub trait SequencePop<T>
{
    type Error;
    fn pop(&mut self) -> Result<T, Self::Error>;
}

impl<T> SequencePop<T> for Vec<T>
{
    type Error=();
    fn pop(&mut self) -> Result<T, Self::Error> { self.pop().ok_or(()) }
}

impl<T> SequencePop<T> for VecDeque<T>
{
    type Error=();
    fn pop(&mut self) -> Result<T, Self::Error> { self.pop_back().ok_or(()) }
}

impl<T> SequencePop<T> for LinkedList<T>
{
    type Error=();
    fn pop(&mut self) -> Result<T, Self::Error> { self.pop_back().ok_or(()) }
}

impl SequencePop<char> for String
{
    type Error=();
    fn pop(&mut self) -> Result<char, Self::Error> { self.pop().ok_or(()) }
}


//trait IndexableSequence<Idx> : Collection + GetIndex<Idx> + GetIndexMut<Idx>
