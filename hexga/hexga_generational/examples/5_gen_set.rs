use hexga_generational::prelude::*;


fn main()
{
    /*
    let mut set= GenHashSet::new();
    assert!(set.insert("abc".to_owned()).1.is_none());
    assert!(set.insert("abc".to_owned()).1.is_some());
    */

    let mut set = GenBTreeSet::new();
    assert!(set.insert("abc".to_owned()).1.is_none());
    assert!(set.insert("abc".to_owned()).1.is_some());
    dbg!(set);
}
