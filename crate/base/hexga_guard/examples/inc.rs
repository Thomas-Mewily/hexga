use std::sync::*;
use hexga_guard::*;

fn display_and_inc<G>(guarded: &G) where G: GuardedMut<i32>
{
    print!("{} = ", std::any::type_name::<G>());
    match guarded.try_get_mut()
    {
        Ok(mut guard) => 
        {
            *guard += 1;
            let value = *guard;
            println!("{value}");
        },
        Err(e) => println!("Can't write: {:?}", e),
    }
}

fn main()
{
    let mutex = Mutex::new(42);
    display_and_inc(&mutex);
    display_and_inc(&mutex);

    let rwlock = RwLock::new(64);
    display_and_inc(&rwlock);
    display_and_inc(&rwlock);
}