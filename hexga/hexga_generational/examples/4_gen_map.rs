use hexga_generational::prelude::*;

fn main()
{
    let mut map: GenHashMap<String, i32> = GenHashMap::new();
    assert!(map.insert("abc".to_owned(), 1).1.is_none());
    assert!(map.insert("abc".to_owned(), 2).1.is_some());
    dbg!(map);

    let mut map:GenBTreeMap<String, i32> = GenBTreeMap::new();
    assert!(map.insert("abc".to_owned(), 1).1.is_none());
    assert!(map.insert("abc".to_owned(), 2).1.is_some());
    dbg!(map);
}
