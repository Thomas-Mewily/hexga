use hexga_generational::prelude::*;

#[derive(Debug)]
struct Entity
{
    name: &'static str,
    // GenID, by nature, can be NULL / always return none,
    // So there is no need to wrap it in an Option
    eating: GenID,
}

fn main()
{
    let mut entities = GenVec::new();

    let zombie = entities.insert(Entity {
        name: "zombie",
        eating: GenID::NULL,
    });
    let slime = entities.insert(Entity {
        name: "slime",
        eating: zombie,
    });

    let ouroboros = entities.insert_cyclic(|id| Entity {
        name: "Ouroboros",
        eating: id,
    });

    if let Some(slime_entity) = entities.get_mut(slime)
    {
        slime_entity.eating = ouroboros;
    }

    for (id, entity) in &entities
    {
        println!(
            "{:?} => name: {}, target: {:?}",
            id, entity.name, entity.eating
        );
    }
}
