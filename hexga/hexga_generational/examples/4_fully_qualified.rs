use hexga_array_vec::ArrayVec;
use hexga_core::prelude::*;
use hexga_generational::{gen_vec::{Entry, GenArrayVec, GenVecOf}, prelude::*};

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
    let mut entities: GenVecOf<Entity, u32, ArrayVec<Entry<Entity>, 10>> = GenArrayVec::<Entity, 10>::new();
    entities.try_push(Entity { hp: 42 }).unwrap();
    entities.try_push(Entity { hp: 99 }).unwrap();

    let mut entities: GenVecOf<Entity, u32, Vec<Entry<Entity>>> = GenVec::<Entity>::new();
    entities.try_push(Entity { hp: 42 }).unwrap();
    entities.try_push(Entity { hp: 99 }).unwrap();

    println!("{:?}", entities);

    increase_hp(entities.as_mut());
    println!("{:?}", entities);
}
