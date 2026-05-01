use super::*;

pub type AppCtx = ();

pub trait Application<E=AppEvent,Ctx=AppCtx>
{
    fn message(&mut self, msg: AppMessage<E>, ctx: &mut Ctx)
    {
        match msg
        {
            AppMessage::Event(ev) => self.event(ev, ctx),
            AppMessage::Flow(flow) => self.flow(flow, ctx),
        }
    }

    
    fn event(&mut self, ev: E, ctx: &mut Ctx) { let _ = ev; }
    fn flow(&mut self, flow: FlowMessage, ctx: &mut Ctx)
    {
        match flow
        {
            FlowMessage::Resumed => self.resumed(ctx),
            FlowMessage::Suspended => self.suspended(ctx),
            FlowMessage::Update(dt) => self.update(dt, ctx),
            FlowMessage::Draw => self.draw(ctx),
        }
    }

    fn resumed(&mut self, ctx: &mut Ctx) {}
    fn suspended(&mut self, ctx: &mut Ctx) {}

    fn update(&mut self, dt: DeltaTime, ctx: &mut Ctx) {}
    fn draw(&mut self, ctx: &mut Ctx) {}
}

impl<A,E,Ctx> MessageHandler<AppMessage<E>,Ctx> for AppMessageAdapter<A,E,Ctx>
where
    A: Application<E,Ctx>,
{
    fn message(&mut self, message: AppMessage<E>, ctx: &mut Ctx)
    {
        match message
        {
            AppMessage::Event(event) => self.app.event(event, ctx),
            AppMessage::Flow(flow) => match flow
            {
                FlowMessage::Resumed => Application::resumed(&mut self.app, ctx),
                FlowMessage::Suspended => Application::suspended(&mut self.app, ctx),
                FlowMessage::Update(dt) => Application::update(&mut self.app, dt, ctx),
                FlowMessage::Draw => Application::draw(&mut self.app, ctx),
            },
        }
    }
}
impl<A,E,Ctx> Application<E,Ctx> for AppMessageAdapter<A,E,Ctx>
where
    A: Application<E,Ctx>,
{
    fn message(&mut self, msg: AppMessage<E>, ctx: &mut Ctx) { self.app.message(msg, ctx); }
    fn flow(&mut self, flow: FlowMessage, ctx: &mut Ctx) { self.app.flow(flow, ctx);}
    fn event(&mut self, ev: E, ctx: &mut Ctx) { self.app.event(ev, ctx); }
    fn suspended(&mut self, ctx: &mut Ctx) { self.app.suspended(ctx); }
    fn resumed(&mut self, ctx: &mut Ctx) { self.app.resumed(ctx); }
    fn update(&mut self, dt: DeltaTime, ctx: &mut Ctx) { self.app.update(dt, ctx); }
    fn draw(&mut self, ctx: &mut Ctx) { self.app.draw(ctx); }
}

#[derive(Debug)]
pub struct AppMessageAdapter<A,E=AppEvent,Ctx=AppCtx>
where
    A: Application<E,Ctx>,
{
    pub app: A,
    phantom : PhantomData<(E,Ctx)>
}
impl<A, E,Ctx> AppMessageAdapter<A, E,Ctx>
where
    A: Application<E,Ctx>,
{
    pub fn new(app: A) -> Self { Self { app, phantom: PhantomData } }
}
impl<A, E, Ctx> From<A> for AppMessageAdapter<A, E, Ctx>
where
    A: Application<E,Ctx>,
{
    fn from(value: A) -> Self { Self::new(value) }
}
