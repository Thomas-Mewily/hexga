#![allow(unused)]
use hexga_engine::prelude::*;

pub struct MyApp;

impl<'a> App<(), AppCtx<'a>>  for MyApp
{
    /*
    fn message(&mut self, ev: AppMessage<()>, ctx: &mut AppCtx<'a>) -> Option<AppMessage<()>> {
        println!("{:?}", ev);
        None
    }
    */
    
    fn event(&mut self, ev: AppEvent, ctx: &mut AppCtx) -> Option<AppEvent> {
        println!("{:?}", ev);
        None
    }
    /*

    fn draw(&mut self, ctx: &mut AppCtx<'a>) {
const VERTICES: &[Vertex] = &[
    Vertex::new().with_position(vec3(-0.0868241, 0.49240386, 0.0)).with_color(rgb(0.5, 0.0, 0.5)), // A
    Vertex::new().with_position(vec3(-0.49513406, 0.06958647, 0.0)).with_color(rgb(0.5, 0.0, 0.5)), // B
    Vertex::new().with_position(vec3(-0.21918549, -0.44939706, 0.0)).with_color(rgb(0.5, 0.0, 0.5)), // C
    Vertex::new().with_position(vec3(0.35966998, -0.3473291, 0.0)).with_color(rgb(0.5, 0.0, 0.5)), // D
    Vertex::new().with_position(vec3(0.44147372, 0.2347359, 0.0)).with_color(rgb(0.5, 0.0, 0.5)), // E
];

const INDICES: &[VertexIndex] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];


        Pen.geometry(
            VERTICES, INDICES
            );
    }
    */

}

fn main()
{
    MyApp::run_with_param(|| MyApp, AppParam::default());
}
