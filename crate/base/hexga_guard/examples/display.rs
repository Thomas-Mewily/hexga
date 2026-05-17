use std::sync::*;
use hexga_guard::*;

fn display<G>(guarded: &G) where G: Guarded<i32>
{
    print!("{} = ", std::any::type_name::<G>());
    match guarded.try_get()
    {
        Ok(guard) => 
        {
            let value = *guard;
            println!("{value}");
        },
        Err(e) => println!("Can't read: {:?}", e),
    }
}

fn main()
{
    let mutex = Mutex::new(42);
    display(&mutex);

    let rwlock = RwLock::new(64);
    display(&rwlock);
}