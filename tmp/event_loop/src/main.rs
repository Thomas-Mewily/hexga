#![allow(unused)]
pub use hexga_engine::prelude::*;

struct MonJeu;

impl App for MonJeu
{
    fn update(&mut self, dt: DeltaTime, ctx: &mut AppCtx) {
        
    }

    fn draw(&mut self, ctx: &mut AppCtx, l: &mut AppCtx) {
        
    }

    fn paused(&mut self, ctx: &mut AppCtx<AppDefaultUserEvent,AppDefaultCtx>) {
        
    }
}


fn main()
{
    (|| MonJeu).run().expect("failed to run")
}
