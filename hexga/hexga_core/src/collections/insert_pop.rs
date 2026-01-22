use super::*;



/// A special kind of collection where the order of insertion / deletion is conserved with push() and pop()
//pub trait CollectionSequenceMarker {}
//pub trait CollectionSequenceMarker {}
pub trait CollectionSequenceCommon<T,Idx> : TryGet<Idx, Output=T> + TryGetMut<T, Output=T> + GetManyMut<T, Output=T> + Length {}
pub trait CollectionSequence<T,Idx> : CollectionSequenceCommon<T,Idx> + Clear {}


impl<T,Idx> CollectionSequenceCommon<T,Idx> for [T] where Self: TryGet<Idx,Output = T> + TryGetMut<T, Output=T> + GetManyMut<T, Output=T> {}

impl<T,Idx, const N:usize> CollectionSequenceCommon<T,Idx> for [T;N] where Self: TryGet<Idx,Output = T> + TryGetMut<T, Output=T> + GetManyMut<T, Output=T> {}

impl<T,Idx> CollectionSequenceCommon<T,Idx> for Vec<T> where Self: TryGet<Idx,Output = T> + TryGetMut<T, Output=T> + GetManyMut<T, Output=T> {}
impl<T,Idx> CollectionSequence<T,Idx> for Vec<T> where Self: TryGet<Idx,Output = T> + TryGetMut<T, Output=T> + GetManyMut<T, Output=T> {}

impl<T,Idx> CollectionSequenceCommon<T,Idx> for LinkedList<T> where Self: TryGet<Idx,Output = T> + TryGetMut<T, Output=T> + GetManyMut<T, Output=T> {}
impl<T,Idx> CollectionSequence<T,Idx> for LinkedList<T> where Self: TryGet<Idx,Output = T> + TryGetMut<T, Output=T> + GetManyMut<T, Output=T> {}


pub trait CollectionSet<Q,K> : CollectionMap<Q,K,()>
    where K: Borrow<Q>, Q: ?Sized
{}
impl<Q,K,S> CollectionSet<Q,K> for S where S: CollectionMap<Q,K,()>, K: Borrow<Q>, Q: ?Sized {}



pub trait CollectionMap<Q,K,V=()> : TryInsert<K,V> + Remove<Q> + Length + Clear
    where K: Borrow<Q>, Q: ?Sized
{}

impl<Q,K,V,S> CollectionMap<Q,K,V> for HashMap<K,V,S>
    where
    Self: TryInsert<K,V> + Remove<Q> + Length + Clear,
    K: Borrow<Q>, Q: ?Sized
{}
impl<Q,K,V> CollectionMap<Q,K,V> for BTreeMap<K,V>
    where
    Self: TryInsert<K,V> + Remove<Q> + Length + Clear,
    K: Borrow<Q>, Q: ?Sized
{}

impl<Q,K,S> CollectionMap<Q,K> for HashSet<K,S>
    where
    Self: TryInsert<K> + Remove<Q> + Length + Clear,
    K: Borrow<Q>, Q: ?Sized
{}
impl<Q,K> CollectionMap<Q,K> for BTreeSet<K>
    where
    Self: TryInsert<K> + Remove<Q> + Length + Clear,
    K: Borrow<Q>, Q: ?Sized
{}



/*
impl<Idx,T> CollectionSequence<Idx,T> for VecDeque<T> {}
impl<Idx,T> CollectionSequence<Idx,T> for LinkedList<T> {}

impl<Idx,T> CollectionSequence<Idx,T> for String {}
impl<Idx,T> CollectionSequence<Idx,T> for str {}

impl<Idx,T> CollectionSequence<Idx,T> for OsString {}
impl<Idx,T> CollectionSequence<Idx,T> for OsStr {}
*/

pub trait Push<T>
{
    type Output;
    fn push(&mut self, value: T) -> Self::Output;
}


pub trait Pop<T>
{
    fn pop(&mut self) -> Option<T>;
}
pub trait TryPop<T> : Pop<T>
{
    type Error;
    fn try_pop(&mut self) -> Result<T, Self::Error>;
}

pub trait TryInsert<K,V=()> : Insert<K,V>
{
    type Error;
    fn try_insert(&mut self, key: K, value: V) -> Result<Self::Output, Self::Error>;
}
pub trait Insert<K,V=()>
{
    type Output;
    fn insert(&mut self, key: K, value: V) -> Option<Self::Output>;
}

/*
pub trait Push<T> : Sequence + Insert<T>
{
    fn push(&mut self, value: T) -> Self::Output { self.insert(value) }
}
impl<T,S> Push<T> for S where S: Sequence + Insert<T> {}

pub trait Insert<T>
{
    type Output;
    fn insert(&mut self, value : T) -> Self::Output;
}
pub trait TryInsert<T>
{
    type Output;
    type Error;
    fn try_insert(&mut self, value : T) -> Result<Self::Output, Self::Error>;
}*/


/*
pub trait TryPush<T> : Push<T>
{
    type Error;
    /// Push a value and return it's index.
    fn try_push(&mut self, value : T) -> Result<Self::Output, Self::Error>;
}

pub trait Push<T>
{
    type Output;
    /// Push a value and return it's index.
    fn push(&mut self, value : T) -> Self::Output;
}
*/
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

/*

    */
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
    #[inline(always)]
    fn push(&mut self, value : T) -> Self::Output
    {
        let l = self.len();
        self.push(value);
        l
    }
}
/*
impl<T> TryPush<T> for Vec<T>
{
    type Error = (); // #proper_error
    fn try_push(&mut self, value : T) -> Result<Self::Output, Self::Error> { Ok(Push::push(self, value)) }
}
*/

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
/*
impl<T> TryPush<T> for VecDeque<T>
{
    type Error = (); // #proper_error
    fn try_push(&mut self, value : T) -> Result<Self::Output, Self::Error> { Ok(Push::push(self, value)) }
}
*/
impl<T> Push<T> for LinkedList<T>
{
    type Output = ();
    fn push(&mut self, value : T) -> Self::Output {
        self.push_back(value);
    }
}
/*
impl<T> TryPush<T> for LinkedList<T>
{
    type Error = (); // #proper_error
    fn try_push(&mut self, value : T) -> Result<Self::Output, Self::Error> {
        self.push(value);
        Ok(())
    }
}
*/
impl Push<char> for String
{
    type Output = ();
    fn push(&mut self, value : char) -> Self::Output {
        self.push(value);
    }
}
/*
impl TryPush<char> for String
{
    type Error = (); // #proper_error
    fn try_push(&mut self, value : char) -> Result<Self::Output, Self::Error> {
        self.push(value);
        Ok(())
    }
}
*/

impl<'b> Push<&'b OsStr> for OsString
{
    type Output = ();
    fn push(&mut self, value : &'b OsStr) -> Self::Output {
        self.push(value);
    }
}
/*
impl<'b> TryPush<&'b OsStr> for OsString
{
    type Error = (); // #proper_error
    fn try_push(&mut self, value : &'b OsStr) -> Result<Self::Output, Self::Error> {
        self.push(value);
        Ok(())
    }
}
*/



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
