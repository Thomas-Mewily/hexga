#![allow(unused)]
pub use hexga_engine::prelude::*;

struct MonJeu;

impl App for MonJeu
{
    fn update(&mut self, dt: DeltaTime, ctx: &mut AppCtx) {
        //println!("update {dt}")
        let title = format!("{}", ctx.time().current);
        ctx.window().set_title(title);
    }

    fn draw(&mut self, ctx: &mut AppCtx) {
        println!("draw")
    }

    fn resumed(&mut self, ctx: &mut AppCtx) {
        println!("resumed");
    }

    fn suspended(&mut self, ctx: &mut AppCtx) {
        println!("suspended");
    }
}


fn main()
{
    (|| MonJeu).run().expect("failed to run")
}
