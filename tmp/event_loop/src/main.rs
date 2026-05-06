#![allow(unused)]
pub use hexga_engine::prelude::*;

struct MonJeu;

impl App for MonJeu
{
    
}


fn main()
{
    (|| MonJeu).run().expect("failed to run")
}
