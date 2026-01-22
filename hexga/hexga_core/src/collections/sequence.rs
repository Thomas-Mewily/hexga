use super::*;



/*
pub trait Collection : Length /* + Capacity */ {}  // slice and str don't impl capacity
impl<T : ?Sized> Collection for T where T: Length {}
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

pub trait TryPush<T> : Push<T>
{
    type Error;
    /// Push a value and return it's index.
    fn try_push(&mut self, value : T) -> Result<Self::Output, Self::Error>;
}
/*
pub trait PushStronglyType<T>
{
    pub push_strongly_typed(&mut self, value : T) -> IndexOf<T,Self::Output>;
}
*/

pub trait Push<T>
{
    type Output;
    /// Push a value and return it's index.
    fn push(&mut self, value : T) -> Self::Output;
}
/*
pub trait Insert<T>
{
    type Output;
    /// Insert an element into the collection. Output is type dependent.
    fn insert(&mut self, value : T) -> Self::Output;

    type Error;
    /// Insert an element into the collection. Output is type dependent.
    fn try_insert(&mut self, value : T) -> Result<Self::Output, Self::Error>;
}


pub trait InsertSequence<T> : Push<T> + Extend<T>
{
    /// Some special collection kind like SlotVec or GenVec need to be aware that you are pushing a sequence.
    ///
    /// Return the index of the first inserted element, or none if the sequence is empty
    fn insert_sequence<'a, S>(&mut self, sequence : S) -> Option<Self::Output> where S : IntoIterator<Item=T>
    {
        let mut it = sequence.into_iter();

        self.extend_reserve(it.size_hint().0);

        let Some(first) = it.next() else { return None; };
        let r = self.push(first);

        while let Some(next) = it.next()
        {
            self.push(next);
        }
        Some(r)
    }
    fn try_insert_sequence<'a, S>(&mut self, sequence : S) -> Result<Option<Self::Output>, Self::Error> where S : IntoIterator<Item=T>
    {
        Ok(InsertSequence::insert_sequence(self, sequence))
    }
}
*/

pub trait TryPop<T> : Pop<T>
{
    type Error;
    fn try_pop(&mut self) -> Result<T, Self::Error>;
}

pub trait Pop<T>
{
    fn pop(&mut self) -> Option<T>;
}
/*
pub trait RemoveSequence<T> : InsertSequence<T>
{
    fn pop_sequence(&mut self, index : Self::Output) -> Option<impl Iterator<Item = T>> { self.try_pop_sequence(index).ok() }
    fn try_pop_sequence(&mut self, index : Self::Output) -> Result<impl Iterator<Item = T>, Self::Error>;
}
*/


impl<T> Push<T> for Vec<T>
{
    type Output = usize;
    fn push(&mut self, value : T) -> Self::Output
    {
        let l = self.len();
        self.push(value);
        l
    }
}
impl<T> TryPush<T> for Vec<T>
{
    type Error = (); // #proper_error
    fn try_push(&mut self, value : T) -> Result<Self::Output, Self::Error> { Ok(Push::push(self, value)) }
}

impl<T> Push<T> for VecDeque<T>
{
    type Output = usize;
    fn push(&mut self, value : T) -> Self::Output
    {
        let l = self.len();
        self.push_back(value);
        l
    }
}
impl<T> TryPush<T> for VecDeque<T>
{
    type Error = (); // #proper_error
    fn try_push(&mut self, value : T) -> Result<Self::Output, Self::Error> { Ok(Push::push(self, value)) }
}

impl<T> Push<T> for LinkedList<T>
{
    type Output = ();
    fn push(&mut self, value : T) -> Self::Output {
        self.push_back(value);
    }
}
impl<T> TryPush<T> for LinkedList<T>
{
    type Error = (); // #proper_error
    fn try_push(&mut self, value : T) -> Result<Self::Output, Self::Error> {
        self.push(value);
        Ok(())
    }
}

impl Push<char> for String
{
    type Output = ();
    fn push(&mut self, value : char) -> Self::Output {
        self.push(value);
    }
}
impl TryPush<char> for String
{
    type Error = (); // #proper_error
    fn try_push(&mut self, value : char) -> Result<Self::Output, Self::Error> {
        self.push(value);
        Ok(())
    }
}

impl<'b> Push<&'b OsStr> for OsString
{
    type Output = ();
    fn push(&mut self, value : &'b OsStr) -> Self::Output {
        self.push(value);
    }
}
impl<'b> TryPush<&'b OsStr> for OsString
{
    type Error = (); // #proper_error
    fn try_push(&mut self, value : &'b OsStr) -> Result<Self::Output, Self::Error> {
        self.push(value);
        Ok(())
    }
}




impl<T> TryPop<T> for Vec<T>
{
    type Error=(); // #proper_error
    fn try_pop(&mut self) -> Result<T, Self::Error> { self.pop().ok_or(()) }
}
impl<T> Pop<T> for Vec<T>
{
    fn pop(&mut self) -> Option<T> { self.pop() }
}

impl<T> TryPop<T> for VecDeque<T>
{
    type Error=(); // #proper_error
    fn try_pop(&mut self) -> Result<T, Self::Error> { self.pop_back().ok_or(()) }
}
impl<T> Pop<T> for VecDeque<T>
{
    fn pop(&mut self) -> Option<T> { self.pop_back() }
}

impl<T> TryPop<T> for LinkedList<T>
{
    type Error=(); // #proper_error
    fn try_pop(&mut self) -> Result<T, Self::Error> { self.pop_back().ok_or(()) }
}
impl<T> Pop<T> for LinkedList<T>
{
    fn pop(&mut self) -> Option<T> { self.pop_back() }
}


//trait IndexableSequence<Idx> : Collection + GetIndex<Idx> + GetIndexMut<Idx>
