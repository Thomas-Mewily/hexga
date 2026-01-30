use hexga_core::prelude::*;
use hexga_generational::prelude::*;

#[derive(Debug)]
pub struct Entity
{
    hp: i32,
}

fn increase_hp<'a>(mut entities: GenViewMut<'a,Entity>)
{
    for e in entities
    {

    }
}


fn main()
{
    let mut entities = [Entity{ hp: 42 }, Entity{ hp: 99 }].to_genvec();
    println!("{:?}", entities);

    increase_hp(entities.as_mut());
    println!("{:?}", entities);
}