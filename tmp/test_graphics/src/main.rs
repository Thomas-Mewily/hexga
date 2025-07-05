use hexga_engine::{prelude::*, AppRunParam};

struct App;

impl AppLoop for App
{
    fn update(&mut self, ctx: &mut AppCtx)
    {
        let now = std::time::SystemTime::now();
        if let Ok(duration) = now.duration_since(std::time::UNIX_EPOCH) {
            let seconds = duration.as_millis() % 1000;
            println!("updated: {}", seconds);
        }
    }

    fn draw(&mut self, ctx: &mut AppCtx) {
        let now = std::time::SystemTime::now();
        if let Ok(duration) = now.duration_since(std::time::UNIX_EPOCH) {
            let seconds = duration.as_millis() % 1000;
            println!("draw: {}", seconds);
        }
    }
}

fn main()
{
    App.run_with_param(AppRunParam::game()).unwrap();
}
