use hexga_generational::prelude::*;


fn main()
{
    let mut set= GenHashSet::new();

    let (id, first_insertion) = set.insert("abc".to_owned());
    assert_eq!(first_insertion, true);
    let (id2, first_insertion2) = set.insert("abc".to_owned());
    assert_eq!(first_insertion2, false);
    assert_eq!(id, id2);
}
