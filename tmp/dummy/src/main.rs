use hexga::prelude::*;

fn main() 
{
    for _ in 0..5 { println!(); }
    println!("Hello");

    for r in rect2(0., 0., 800., 400.).split_x(3.)
    {
        println!("{r}");
    }
    //MyApp::new().run();
    println!("Good bye");
}
