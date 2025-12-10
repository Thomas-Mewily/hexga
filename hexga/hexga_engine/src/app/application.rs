use super::*;


// EventHandler<E> is a super trait for Application
pub trait MessageHandler<E>
{
    fn message(&mut self, message: E);
    //fn update(&mut self);
}

impl<S> MessageHandler<AppMessage> for S where S:Application
{
    fn message(&mut self, message: AppMessage) {
        match message
        {
            AppMessage::Event(event) => self.event(event),
            AppMessage::Flow(flow) => match flow
            {
                FlowMessage::Resumed => Application::resumed(self),
                FlowMessage::Paused => Application::paused(self),
                FlowMessage::Update(dt) => Application::update(self, dt),
                FlowMessage::Draw => Application::draw(self),
            },
        }
    }
}

pub trait Application: MessageHandler<AppMessage>
{
    fn event(&mut self, ev: AppEvent) { let _ = ev; }

    fn resumed(&mut self) {}
    fn paused(&mut self) {}

    fn update(&mut self, dt: DeltaTime) { }
    fn draw(&mut self) { }
}

