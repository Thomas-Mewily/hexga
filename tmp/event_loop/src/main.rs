#![allow(unused)]
pub use hexga_event_loop::prelude::*;

struct MonJeu;

impl PlatformEventHandler for MonJeu
{

}


fn main()
{
    MonJeu.run_event_loop();
}
