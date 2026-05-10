use super::*;

pub trait Matches<Lexem=Self>
{
    type Output;
    fn matches(&self, lexem: &Lexem) -> Self::Output;
}
impl<S,L> Matches<L> for Option<S> where S: Matches<L>, S::Output : Default
{
    type Output= S::Output;
    fn matches(&self, lexem: &L) -> Self::Output {
        match self
        {
            Some(v) => v.matches(lexem),
            None => ___(),
        }
    }
}

impl<S,L,E> Matches<L> for Result<S,E> where S: Matches<L>, S::Output : Default
{
    type Output= S::Output;
    fn matches(&self, lexem: &L) -> Self::Output {
        match self
        {
            Ok(v) => v.matches(lexem),
            Err(_) => ___(),
        }
    }
}