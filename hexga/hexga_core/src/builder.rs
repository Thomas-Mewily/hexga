
// Infaillible Builder
pub trait Builder
{
    type Output;
    fn build(&self) -> Self::Output;
    fn build_in(&self, dest: &mut Self::Output) { *dest = self.build(); }
}