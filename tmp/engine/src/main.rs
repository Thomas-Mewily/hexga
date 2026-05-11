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

        println!("{}", Time::since_launch());
        let pos = (MainWindow.pos());
        dbg!(&pos);
        dbg!(&pos.moved_x(1));
        MainWindow.set_pos(pos.moved_x(1));
        println!();
    }

    fn draw(&mut self, coef: coef, ctx: &mut ()) {
        
    }
}


fn main()
{
    (||MonJeu).run().expect("failed to run");
}
