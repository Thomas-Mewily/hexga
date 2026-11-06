use hexga_core::prelude::IndexID;
use hexga_generational::prelude::*;

fn main()
{
    let mut entities = GenVec::new();
    let enemy = entities.insert("zoombie");

    assert_eq!(enemy.get(&entities), Some(&"zoombie"));
    assert_eq!(entities.get(enemy), Some(&"zoombie"));
    assert_eq!(entities[enemy], "zoombie");

    entities.remove(enemy); // the key is no longer valid
    assert!(entities.get(enemy).is_none()); // the value don't exist

    entities.insert("slime");
    entities.insert("skeleton");

    for (id, entity) in entities
    {
        println!("{:?} => {}", id, entity)
    }
}
