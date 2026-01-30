use super::*;

pub trait Application
{
    fn event(&mut self, ev: AppEvent) { let _ = ev; }

    fn resumed(&mut self) {}
    fn suspended(&mut self) {}

    fn update(&mut self, dt: DeltaTime) {}
    fn draw(&mut self) {}
}

impl<A> MessageHandler<AppMessage> for AppMessageAdapter<A>
where
    A: Application,
{
    fn message(&mut self, message: AppMessage)
    {
        match message
        {
            AppMessage::Event(event) => self.app.event(event),
            AppMessage::Flow(flow) => match flow
            {
                FlowMessage::Resumed => Application::resumed(&mut self.app),
                FlowMessage::Suspended => Application::suspended(&mut self.app),
                FlowMessage::Update(dt) => Application::update(&mut self.app, dt),
                FlowMessage::Draw => Application::draw(&mut self.app),
            },
        }
    }
}
impl<A> Application for AppMessageAdapter<A>
where
    A: Application,
{
    fn event(&mut self, ev: AppEvent) { self.app.event(ev); }
    fn suspended(&mut self) { self.app.suspended(); }
    fn resumed(&mut self) { self.app.resumed(); }
    fn update(&mut self, dt: DeltaTime) { self.app.update(dt); }
    fn draw(&mut self) { self.app.draw(); }
}

#[derive(Debug)]
pub struct AppMessageAdapter<A>
where
    A: Application,
{
    pub app: A,
}
impl<A> AppMessageAdapter<A>
where
    A: Application,
{
    pub fn new(app: A) -> Self { Self { app } }
}
impl<A> From<A> for AppMessageAdapter<A>
where
    A: Application,
{
    fn from(value: A) -> Self { Self::new(value) }
}
