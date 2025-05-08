🚧 **Warning: Experimental Crate!** 🚧

This crate is currently in **beta** and **experimental**.  
It is subject to **breaking changes** in future releases.  
Use it at your own risk, and keep in mind that the API may change in future versions.

## HexGa Slot

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

Todo:

GenSlot, MultiGenSlot