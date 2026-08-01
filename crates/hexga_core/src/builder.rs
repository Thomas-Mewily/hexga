// Infaillible Builder
pub trait Builder
{
    type Output;
    fn build(&self) -> Self::Output;
    fn build_in(&self, dest: &mut Self::Output) { *dest = self.build(); }

    fn build_in_option(&self, dest: &mut Option<Self::Output>)
    {
        match dest
        {
            Some(op) => self.build_in(op),
            None => *dest = Some(self.build()),
        }
    }
}
// pub trait TryBuilder ...


pub mod prelude
{
    pub use super::traits::*;
}
pub mod traits
{
    pub use super::Builder;
}