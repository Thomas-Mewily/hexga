#![allow(unused)]
use hexga_engine::event_loop::{
    input::KeyCode, traits::WithEventLoopShortcut, window::{UserAttentionType, WindowAttribute, Windowable}
};
pub use hexga_engine::prelude::*;

struct MonJeu;

impl App for MonJeu
{
    fn event(&mut self, ev: AppEvent, ctx: &mut ()) -> Option<AppEvent>
    {
        /*
        for m  in CurrentWindow.available_monitors()
        {
            dbg!(m);
        }*/

        println!("{ev:?}");
        //None
        Some(ev)
    }

    fn update(&mut self, dt: Duration, ctx: &mut ())
    {
        
        //CurrentWindow.set_title(format!("{}", Time::since_launch()));
    }

    fn draw(&mut self, coef: coef, ctx: &mut ())
    {
        let VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.0868241, 0.49240386, 0.0].into(),
        color: [0.5, 0.0, 0.5].into(),
        uv: zero(),
    }, // A
    Vertex {
        position: [-0.49513406, 0.06958647, 0.0].into(),
        color: [0.5, 0.0, 0.5].into(),
        uv: zero(),
    }, // B
    Vertex {
        position: [-0.21918549, -0.44939706, 0.0].into(),
        color: [0.5, 0.0, 0.5].into(),
        uv: zero(),
    }, // C
    Vertex {
        position: [0.35966998, -0.3473291, 0.0].into(),
        color: [0.5, 0.0, 0.5].into(),
        uv: zero(),
    }, // D
    Vertex {
        position: [0.44147372, 0.2347359, 0.0].into(),
        color: [0.5, 0.0, 0.5].into(),
        uv: zero(),
    }, // E
];
        //CurrentWindow.set_title(format!("{}", Time::since_launch()));
    }
}


#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run_wasm() { run(); }

pub fn run()
{
    let mut param = AppParam::default().with_icon(
        Image::load_from_bytes(include_bytes!("icon.png"), Some("png")).expect("no icon"),
    );

    
    (|| MonJeu).run_with_param(param).expect("failed to run");
}
