#![allow(unused)]
use hexga_engine::event_loop::window::WindowAttribute;
pub use hexga_engine::prelude::*;

struct MonJeu;

impl App for MonJeu
{
    fn event(&mut self, ev: AppEvent, ctx: &mut ()) -> Option<AppEvent> 
    {
        None
    }

    fn update(&mut self, dt: Duration, ctx: &mut ()) {
        CurrentWindow.set_title(format!("{}", Time::since_launch()));
    }

    fn draw(&mut self, coef: coef, ctx: &mut ()) {
        
    }
}


fn main()
{
    let mut param = AppParam::default()
        .with_icon(
            Image::load_from_bytes(include_bytes!("icon.png"), Some("png")).expect("no icon")
        );
    (||MonJeu).run_with_param(param).expect("failed to run");
}
