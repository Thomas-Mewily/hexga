#![allow(unused)]
pub use hexga_engine::prelude::*;

struct MonJeu;

impl App for MonJeu
{
    fn event(&mut self, ev: AppEvent, ctx: &mut ()) -> Option<AppEvent> 
    {
        None
    }

    fn update(&mut self, dt: Duration, ctx: &mut ()) {
        
    }

    fn draw(&mut self, coef: coef, ctx: &mut ()) {
        
    }
}


fn main()
{
    (||MonJeu).run().expect("failed to run");
}
