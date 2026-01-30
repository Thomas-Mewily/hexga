use hexga_core::prelude::*;
use hexga_generational::{gen_vec::GenArrayVec, prelude::*};

#[derive(Debug)]
pub struct Entity
{
    hp: i32,
}

fn increase_hp<'a>(entities: GenViewMut<'a, Entity>)
{
    for (_id, entity) in entities
    {
        entity.hp += 1;
    }
}

fn main()
{
    let mut entities = GenArrayVec::<Entity, 10>::try_from_iter(
        [Entity { hp: 42 }, Entity { hp: 99 }]
    ).unwrap();
    println!("{:?}", entities);

    increase_hp(entities.as_mut());
    println!("{:?}", entities);
}
