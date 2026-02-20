use super::*;

pub type Arena = ArenaOf;

// Todo: You should probably not pass a Vec<A> for the container,
// Because the iterator of vector act like a vector [0..n] and not a stack [n-1..=0]
// So allocating will first iterate over the older arena that are probably full
#[derive(Clone)]
pub struct ArenaOf<A=arena_buffer::BufferArena, C = SinglyLinkedNode<A>>
where
    A: Arenable,
    C: Push<A>,
{
    capacity: usize,
    used: usize,
    arenas: C,
    phantom: PhantomData<A>,
}

impl<A, C> Debug for ArenaOf<A, C>
where
    A: Arenable,
    C: Push<A> + Default,
    for<'a> &'a C: IntoIterator<Item = &'a A>,
    for<'a> &'a mut C: IntoIterator<Item = &'a mut A>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result
    {
        let used = self.used;
        let cap = self.capacity;
        let used_pourcent = self.used_pourcent();

        f.debug_struct("Arena")
            .field("used", &used)
            .field("capacity", &cap)
            .field("used %", &format_args!("{:.1}%", used_pourcent * 100.0))
            .field("nb_entry", &self.arenas.iter().count())
            .finish()
    }
}

impl<'a, A, C> IntoIterator for &'a ArenaOf<A, C>
where
    A: Arenable,
    C: Push<A>,
    &'a C: IntoIterator<Item = &'a A>,
{
    type Item = &'a A;
    type IntoIter = <&'a C as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter { self.arenas.into_iter() }
}

/*
// Do not expose in the API, new allocation will break the used/capacity counter of Arenas
impl<'a, A,C> IntoIterator for &'a mut Arenas<A,C>
where
    A: Arena, C: Push<A>, &'a mut C: IntoIterator<Item = &'a mut A>
{
    type Item = &'a mut A;
    type IntoIter = <&'a mut C as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.arenas.into_iter()
    }
}
*/

impl<'a, A, C> IntoIterator for ArenaOf<A, C>
where
    A: Arenable,
    C: Push<A> + IntoIterator<Item = A>,
{
    type Item = A;
    type IntoIter = <C as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter { self.arenas.into_iter() }
}

impl<A, C> ArenaOf<A, C>
where
    A: Arenable,
    C: Push<A> + Default,
    for<'a> &'a C: IntoIterator<Item = &'a A>,
{
    pub fn new() -> Self
    {
        Self::from_arenas(C::___())
    }
}

/*
impl<A,C> FromIterator<A> for ArenaOf<A,C>
where
    A: Arenable, C: Push<A> + Default + WithCapacity, for<'a> &'a C: IntoIterator<Item = &'a A>, <C as WithCapacity>::Param : Default
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut s = Self::___();
        s.arena
    }
}*/

impl<A, C> Push<A> for ArenaOf<A, C>
where
    A: Arenable,
    C: Push<A>,
{
    type Output = C::Output;
    fn push(&mut self, value: A) -> Self::Output { self.arenas.push(value) }
}
impl<A, C> TryPush<A> for ArenaOf<A, C>
where
    A: Arenable,
    C: TryPush<A>,
{
    type Error = C::Error;
    fn try_push(&mut self, value: A) -> Result<Self::Output, Self::Error>
    {
        self.arenas.try_push(value)
    }
}

impl<A, C> PushFront<A> for ArenaOf<A, C>
where
    A: Arenable,
    C: Push<A> + PushFront<A>,
{
    type Output = <C as PushFront<A>>::Output;
    fn push_front(&mut self, value: A) -> Self::Output { self.arenas.push_front(value) }
}
impl<A, C> TryPushFront<A> for ArenaOf<A, C>
where
    A: Arenable,
    C: Push<A> + TryPushFront<A>,
{
    type Error = <C as TryPushFront<A>>::Error;
    fn try_push_front(&mut self, value: A) -> Result<Self::Output, Self::Error>
    {
        self.arenas.try_push_front(value)
    }
}

// Should I impl Pop and PopFront (absolutely unsafe, but arena are all about unsafe ?)

impl<A, C> ArenaOf<A, C>
where
    A: Arenable,
    C: Push<A>,
    for<'a> &'a C: IntoIterator<Item = &'a A>,
{
    pub fn from_arenas(arenas: C) -> Self
    {
        let mut capacity = 0;
        let mut used = 0;
        for arena in &arenas
        {
            capacity += arena.capacity();
            used += arena.nb_used();
        }
        Self {
            capacity,
            used,
            arenas,
            phantom: PhantomData,
        }
    }
}

impl<A, C> ManagedBox for ArenaOf<A, C>
where
    A: Arenable,
    C: Push<A>,
    A: ManagedBox,
{
    type Box<T> = <A as ManagedBox>::Box<T>;
}
unsafe impl<A, C> AllocFromLayout for ArenaOf<A, C>
where
    A: Arenable,
    C: Push<A>,
    for<'a> &'a mut C: IntoIterator<Item = &'a mut A>,
{
    type Output = AllocOutput;

    fn alloc_layout(&mut self, layout: AllocLayout) -> AllocResult<Self::Output>
    {
        let mut empty = true;
        for arena in self.arenas.into_iter()
        {
            empty = false;
            self.capacity -= arena.capacity();
            self.used -= arena.nb_used();

            let alloc_result = arena.alloc_layout(layout);

            self.capacity += arena.capacity();
            self.used += arena.nb_used();

            match alloc_result
            {
                Ok(ptr) => return Ok(ptr),
                Err(_) =>
                {}
            }
        }

        let mut new_capacity = layout;
        new_capacity.size = (self.capacity * 2).max(layout.size);
        if empty
        {
            // 16K bytes by default.
            // If you want any other amount, create the Arenas from an existing arena.
            new_capacity.size = new_capacity.size.max(AllocBlock::DEFAULT_SIZE);
        }
        let mut next_arena = A::from_alloc_layout(new_capacity);
        let ptr = next_arena.alloc_layout(layout)?;
        self.capacity += next_arena.capacity();
        self.used += next_arena.nb_used();
        self.arenas.push(next_arena);
        Ok(ptr)
    }
}


impl<A, C> ArenaOf<A, C>
where
    A: Arenable,
    C: Push<A>,
{
    fn from_arena(arena: A) -> Self
    where
        C: Default,
    {
        let capacity = arena.capacity();
        let used = arena.nb_used();
        let mut arenas = C::default();
        arenas.push(arena);
        Self {
            arenas,
            capacity,
            used,
            phantom: PhantomData,
        }
    }
}
impl<A, C> WithCapacity for ArenaOf<A, C>
where
    A: Arenable,
    C: Push<A> + Default,
{
    type Param = A::Param;
    fn with_capacity_and_param(capacity: usize, param: Self::Param) -> Self
    {
        let arena = A::with_capacity_and_param(capacity, param);
        Self::from_arena(arena)
    }
}

impl<A, C> Arenable for ArenaOf<A, C>
where
    A: Arenable,
    C: Push<A> + Default,
    for<'a> &'a C: IntoIterator<Item = &'a A>,
    for<'a> &'a mut C: IntoIterator<Item = &'a mut A>,
{
    fn contains(&self, ptr: NonNull<u8>) -> bool { self.iter().any(|a| a.contains(ptr)) }
}

impl<A, C> Length for ArenaOf<A, C>
where
    A: Arenable,
    C: Push<A>,
{
    fn len(&self) -> usize { self.used }
}
impl<A, C> Capacity for ArenaOf<A, C>
where
    A: Arenable,
    C: Push<A>,
{
    fn capacity(&self) -> usize { self.capacity }
}
impl<A, C> Collection for ArenaOf<A, C>
where
    A: Arenable,
    C: Push<A>,
{
}

impl<A, C> From<AllocLayout> for ArenaOf<A, C>
where
    A: Arenable,
    C: Push<A> + Default,
{
    fn from(layout: AllocLayout) -> Self { Self::from_arena(A::from(layout)) }
}

impl<A, C> ArenaOf<A, C>
where
    A: Arenable,
    C: Push<A>,
{
    pub fn iter<'a>(&'a self) -> <&'a C as IntoIterator>::IntoIter
    where
        &'a C: IntoIterator<Item = &'a A>,
    {
        self.arenas.into_iter()
    }
    #[allow(unused)]
    pub(crate) fn iter_mut<'a>(&'a mut self) -> <&'a mut C as IntoIterator>::IntoIter
    where
        &'a mut C: IntoIterator<Item = &'a mut A>,
    {
        self.arenas.into_iter()
    }
}
