use super::*;

#[macro_export]
macro_rules! dbg_here {
    () => {
        log::debug!(
            "At {}:{} in {}",
            file!(),
            line!(),
            std::module_path!()
        )
    };
}


pub trait FmtOptionalZero : Sized
{
    /// Formats the value with a space separator after it only if it is not zero.
    fn fmt_if_not_zero(&self) -> FmtOptional<'_, Self> where Self: Zero + PartialEq
    {
        FmtOptional::new(if self.is_zero() { None } else { Some(self) })
    }
}
impl<T> FmtOptionalZero for T where T: Sized {}