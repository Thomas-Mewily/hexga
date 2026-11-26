use hexga_engine::prelude::*;

pub struct MyApp;

impl<'a> App<(), AppCtx<'a>> for MyApp
{
    fn event(&mut self, ev: AppEvent, _ctx: &mut AppCtx<'a>) -> Option<AppEvent>
    {
        println!("{:?}", ev);
        None
    }

    fn update(&mut self, _dt: Duration, _ctx: &mut AppCtx<'a>) {}

    fn draw(&mut self, _ctx: &mut AppCtx<'a>)
    {
        //Pen.set_background_color(rgb(0.025, 0.035, 0.08));

        // RGB vertices blend into a smooth gradient across the triangle.
        let triangle = [
            Vertex::new().with_position(vec3(0.0, 0.75, 0.0)).with_color(Color::RED),
            Vertex::new().with_position(vec3(-0.9, -0.75, 0.0)).with_color(Color::GREEN),
            Vertex::new().with_position(vec3(0.9, -0.75, 0.0)).with_color(Color::BLUE),
        ];
        Pen.geometry(&triangle, &[0, 1, 2]);

        /*
        // HSL colors convert to the same render color type as RGB values.
        let mut purple = Circle::new(vec3(-0.48, -0.55, 0.0), 0.28, ToColor::to_rgba(hsl(0.82, 0.85, 0.58))).with_segments(48);
        let mut cyan = Circle::new(vec3(0.0, -0.55, 0.0), 0.28, ToColor::to_rgba(hsl(0.48, 0.85, 0.55))).with_segments(48);
        let mut orange = Circle::new(vec3(0.48, -0.55, 0.0), 0.28, ToColor::to_rgba(hsl(0.12, 0.9, 0.58))).with_segments(48);
        let _ = purple.draw();
        let _ = cyan.draw();
        let _ = orange.draw();
        */
    }
}

fn main() { MyApp::run_with_param(|| MyApp, AppParam::default()).expect("failed to run triangle example"); }
