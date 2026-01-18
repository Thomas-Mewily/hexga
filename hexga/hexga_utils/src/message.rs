pub mod prelude
{
    pub use super::MessageHandler;
}

pub trait MessageHandler<E>
{
    fn message(&mut self, message: E);
    //fn update(&mut self);
}


