🚧 **Warning: Experimental Crate!** 🚧

This crate is currently in **beta** and **experimental**.  
It is subject to **breaking changes** in future releases.  
Use it at your own risk, and keep in mind that the API may change in future versions.

## HexGa Generational

Provide, `GenVec`, a generational index-based vector for and efficient element storage and retrieval, ideal for MAS (Multi-Agent System), where each agent can be removed at any time and has references to other agents.

Provides a stable ID for each inserted element.

- `insert()` in O(1)
- `get()`/`get_mut()` in O(1), (no hashing)
- `remove()` in O(1)
- iteration over elements.

Supports custom index types, including wrapping indices generation for to allow generation wrapping.

## Example

```rust
let mut entities = GenVec::new();
let enemi = entities.insert("zoombie");

assert_eq!(enemi.get(&entities), Some(&"zoombie"));
assert_eq!(entities[enemi], "zoombie");

assert!(entities.get(enemi).is_some());
entities.remove(enemi); // the key is no longer valid
assert!(entities.get(enemi).is_none()); // the value don't exist

entities.insert("slime");
entities.insert("skeleton");

for (id, entity) in entities
{
    println!("{:?} => {}", id, entity)
}
```

## Choose your policy !

By default, slots are saturating. This means that once the generation reaches the maximum index, the slot becomes invalid and will not be reused.

Inside this crate, `GenVec` is a type alias for `type GenVec<T> = GenVecOf<T,u32>`. To modify the policy, you can use the more general type, `GenVecOf` :

- `GenVecOf::<T, u32>` for a saturating generation approch (by default)
- `GenVecOf::<T, Wrapping<u8>>` for a wrapping generation approch (This means that after many insertions and deletions, there is a small chance that an invalid key could become valid again and point to an unintended element.)

## Use case

GenVec are ideal for MAS (Multi-Agent System), where each agent can be removed at any time and has references to other agents.

This is great for stuff like simulation, video game, ECS...

## Inspiration

This crate was mainly inspired by 
- RustConf 2018 - Closing Keynote - Using Rust For Game Development by Catherine West <https://youtu.be/aKLntZcp27M>
- The SlotMap data structure <https://crates.io/crates/slotmap>. If you're looking for a more widely used and established crate in the Rust ecosystem, you may want to check it out as well.

## Main Hexga crate

Check `hexga` : https://crates.io/crates/hexga if you are interested in a quick start, it regroup multiple hexga crates.