use std::{collections::HashMap, sync::{LazyLock, RwLock, atomic::{AtomicUsize, Ordering}}};
use std::ops::Deref;
use super::*;

#[derive(Debug)]
struct StringTable
{
    // (String, usage count, next free slot)
    // Once set, the string is immuable
    values : Vec<(String, AtomicUsize, usize)>,
    // usize max if no free slot remaning
    next_free : usize,
    lookup: HashMap<String, usize>,
}

impl Default for StringTable
{
    fn default() -> Self {
        Self { values:Vec::new(), next_free: usize::MAX, lookup: HashMap::new() }
    }
}

static SYMBOL_TABLE : RwLock<LazyLock<StringTable>> = RwLock::new(LazyLock::new(StringTable::default));

#[derive(PartialEq, Eq, Hash)]
pub struct StringInterned
{
    id : usize,
}

impl PartialOrd for StringInterned
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.deref().partial_cmp(&other.deref())
    }
}
impl Ord for StringInterned
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.deref().cmp(&other.deref())
    }
}

impl StringInterned
{
    pub const fn id(&self) -> usize { self.id }
}

impl Debug for StringInterned
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*self)
    }
}
impl Display for StringInterned
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*self)
    }
}

impl Clone for StringInterned {
    fn clone(&self) -> Self {
        let guard = SYMBOL_TABLE.read().unwrap();
        guard.values[self.id].1.fetch_add(1, Ordering::Relaxed);
        StringInterned { id: self.id }
    }
}

impl StringInterned
{
    pub fn from_id(id: usize) -> Result<Self, &'static str>
    {
        let guard = SYMBOL_TABLE.read().unwrap();
        
        if id >= guard.values.len() {
            return Err("ID out of bounds");
        }
        
        if guard.values[id].0.is_empty() {
            return Err("Slot is free (string was interned but later dropped)");
        }
        
        guard.values[id].1.fetch_add(1, Ordering::Relaxed);
        
        Ok(StringInterned { id })
    }
}

fn string_or_str_to_string_internedfrom<T>(v: T) -> StringInterned 
where T: AsRef<str> + Into<String>
{
    let value = v.as_ref();

    {
        let guard = SYMBOL_TABLE.read().unwrap();
        if let Some(&id) = guard.lookup.get(value) {
            guard.values[id].1.fetch_add(1, Ordering::Relaxed);
            return StringInterned { id };
        }
        drop(guard);
    }

    {
        let mut guard = SYMBOL_TABLE.write().unwrap();
        // Double-check in case another thread inserted it while we were waiting
        if let Some(&id) = guard.lookup.get(value) {
            guard.values[id].1.fetch_add(1, Ordering::Relaxed);
            return StringInterned { id };
        }


        let id = if guard.next_free != usize::MAX 
        {
            let slot = guard.next_free;
            guard.next_free = guard.values[slot].2;
            slot
        } else {
            let new_id = guard.values.len();
            assert!(new_id < usize::MAX - 1);
            guard.values.push((String::new(), AtomicUsize::new(0), usize::MAX));
            new_id
        };

        guard.values[id] = (value.to_string(), AtomicUsize::new(1), usize::MAX);
        guard.lookup.insert(value.into(), id);
        drop(guard);
        
        StringInterned { id }
    }
}

impl From<&str> for StringInterned
{
    fn from(v: &str) -> Self 
    {
        string_or_str_to_string_internedfrom(v)
    }
}
impl From<String> for StringInterned
{
    fn from(v: String) -> Self 
    {
        string_or_str_to_string_internedfrom(v)
    }
}

impl Drop for StringInterned {
    fn drop(&mut self) {
        {
            let guard = SYMBOL_TABLE.read().unwrap();
            let prev = guard.values[self.id].1.fetch_sub(1, Ordering::Relaxed);
            
            if prev != 1 {
                return;
            }
            drop(guard);
        }
        
        let mut guard = SYMBOL_TABLE.write().unwrap();
        
        // Double-check that the slot still exists and is still at count 0
        // (another thread might have re-used it or incremented it)
        
        let current_count = guard.values[self.id].1.load(Ordering::Relaxed);
        if current_count != 0 {
            return; // Another thread incremented it, don't remove
        }
        
        {
            let table = guard.deref_mut().deref_mut();
            table.lookup.remove(&table.values[self.id].0);
        }
        
        // Empty string marks it as free
        // Could also resize it's capacity if it's too big.
        guard.values[self.id].0.clear(); 
        guard.values[self.id].2 = guard.next_free;
        guard.next_free = self.id;
    }
}



impl Deref for StringInterned {
    type Target = str;
    
    fn deref(&self) -> &Self::Target {
        let guard = SYMBOL_TABLE.read().unwrap();
        let ptr = guard.values[self.id].0.as_ptr();
        let len = guard.values[self.id].0.len();
        
        // SAFETY: 
        // 1. The string in the table is immutable once set
        // 2. The string is never deallocated (we keep the slot)
        // 3. The string's memory is stable (String stores on heap)
        // 4. The string will outlive any reference because it's never freed
        unsafe {
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len))
        }
    }
}