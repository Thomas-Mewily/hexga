#![allow(unused)]
pub use hexga_engine::prelude::*;

struct MonJeu;

impl App for MonJeu
{
    fn update(&mut self, dt: DeltaTime, ctx: &mut AppCtx) {
        
    }
}


fn main()
{
    (|| MonJeu).run().expect("failed to run")
}
