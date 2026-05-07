#![allow(unused)]
pub use hexga_engine::prelude::*;

struct MonJeu;

impl App for MonJeu
{
    fn draw(&mut self, l: &mut AppLoop<UserEvent>, ctx: &mut AppCtx) {
        
    }
    
    fn resumed(&mut self, l: &mut AppLoop<UserEvent>, ctx: &mut AppCtx) {
        
    }

    fn event(&mut self, ev: AppEvent<UserEvent>, l: &mut AppLoop<UserEvent>, ctx: &mut AppCtx) -> Option<AppEvent<UserEvent>> {
        None
    }
}

/*
pub trait Game
{
    type Input;
    type Output;
    fn update(&mut self, dt: DeltaTime, input: Self::Input) -> Result<S,()>
    //fn draw(&mut self, coef: coef);
}
*/

fn main()
{
    (|| MonJeu).run().expect("failed to run")
}
