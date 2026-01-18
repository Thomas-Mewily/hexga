All the approch I tought when making this:


`struct Asset`: an handle to the Asset, cheaply sharable

Asset loading, reading and writing system
- Multi thread compatible
- Avoid locking when reading, and impl the deref trait if possible
- hot reloadable => all asset content should be editable at runtime.

```rust
impl<T> Asset<T>
{
    fn hot_reload(&self) { ... }
}
```



1) Make it just single thread.
Only the main thread can access the asset.
- ✅ deref, derefmut
- ❌ single threaded


2) Safetly multi threaded:
```rust
struct Asset<T>
{
    // 1 level of indirection + lock
    value: Arc<RwLock<T>>
}
```
- ✅ multi threaded
- ✅ can live editing asset without having to clone them
- ❌ locking when reading, can't impl deref
- 🟡 (mixed) locked when writing (should be ok)

3) Arc swap:

```rust
struct Asset<T>
{
    // 3 level of indirection
    value: Arc<ArcSwapAny<Arc<T>>>>
}
```

- ✅ multi threaded
- ✅ no "locking" when reading,
- ✅ fast way to write/swap
- ❌ but can't impl deref because reading is still guarded


4) GenID + Cache


```rust
struct Asset<T>
{
    id: GenID<AtomicU32>,
    // 1 level of indirection + some comparison
    cache: Arc<AssetData<T>>
}

struct AssetData<T>
{
    generation: AtomicU32,
    value: T, // or Arc<T> ?
}
```
- ✅ multi threaded
- ✅❌ deref ? Reference can be dangling if deref call are made
- ✅ no "locking" when reading, but need to compare the generation
- ✅ fast way to write/swap

- How to update the cache from a &self ? It is safe ? No
-> There will be a lock in the singleton. Make sure 2 thread don't replace the same cache at the same time.



```rust
// static singleton
struct AssetManager
{
    value: RwLock<GenVecOf<Arc<AssetData<T>>, AtomicU32>>
}
```

deref: if the `id.generation != cache.generation` update the `cache`.
Return a reference to the cache



Other idea:


- Asset are immuable, and hot reload produce a new asset

```rust
impl<T> Asset<T>
{
    fn hot_reload(&self) -> Asset<T> { ... }
}
```
-❌ hard to "factorize" the same asset when hot reloading
-❌ Can't load an asset in a async way (goodbye wasm file loading)

- writing/hot reloading an asset can only be done it the main thread, but reading it can be done in multiple thread ?