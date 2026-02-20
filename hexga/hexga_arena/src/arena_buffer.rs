use hexga_core::boxed::DropOnlyBox;

use super::*;


#[derive(Clone)]
pub struct BufferArena
{
    block: AllocBlock,
    offset: usize,
}

impl Debug for BufferArena
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        let used = self.nb_used();
        let cap  = self.capacity();
        let used_pourcent  = self.used_pourcent();

        f.debug_struct("Arena")
            .field("used", &used)
            .field("capacity", &cap)
            .field("used %", &format_args!("{:.1}%", used_pourcent * 100.0))
            .finish()
    }
}


impl From<AllocBlock> for BufferArena
{
    fn from(block: AllocBlock) -> Self { Self{ block, offset: 0 } }
}
impl From<AllocLayout> for BufferArena
{
    fn from(layout: AllocLayout) -> Self { Self::from(AllocBlock::from(layout)) }
}
impl BufferArena
{
    pub fn capacity(&self) -> usize { self.block.len() }
    pub fn nb_used(&self) -> usize { self.offset }
}

impl Collection for BufferArena {}
impl Arenable for BufferArena
{
    fn contains(&self, ptr: NonNull<u8>) -> bool {
        self.block.deref().contains(unsafe { ptr.as_ref() })
    }
}
impl Length for BufferArena
{
    fn len(&self) -> usize {
        self.nb_used()
    }
}

impl WithCapacity for BufferArena
{
    type Param=();
    fn with_capacity_and_param(capacity: usize, _: Self::Param) -> Self {
        Self::from_size(capacity)
    }
}
impl Capacity for BufferArena
{
    fn capacity(&self) -> usize {
        self.capacity()
    }
}




impl ManagedBox for BufferArena
{
    type Box<T> = DropOnlyBox<T>;
}
unsafe impl AllocFromLayout for BufferArena
{
    type Output=AllocOutput;

    fn allocate_layout(&mut self, layout: AllocLayout) -> AllocResult<Self::Output> {
        assert_ne!(layout.align, 0);

        let base_ptr = self.block.as_mut_ptr() as usize;
        let align_mask = layout.align - 1;

        // align relative to base pointer
        let start = (base_ptr + self.offset + align_mask) & !align_mask;
        let offset = start - base_ptr;
        let end = offset.checked_add(layout.size).ok_or(AllocError)?;

        if end > self.capacity() {
            return Err(AllocError);
        }

        self.offset = end;

        let ptr = unsafe { self.block.as_mut_ptr().add(offset) };
        Ok(unsafe { NonNull::slice_from_raw_parts(
            NonNull::new_unchecked(ptr),
            layout.size
        ) })
    }
}

