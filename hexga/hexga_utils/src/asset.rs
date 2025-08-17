use crate::*;

pub mod prelude
{
    pub use super::Asset;
}

#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Asset<T,E=(),L=()>
{
    Loading(L),
    Loaded(T),
    Error(E),
}

impl<T,E,L> Asset<T,E,L>
{
    pub fn as_loading(&self) -> Option<&L> { if let Self::Loading(l) = self { Some(l) } else { None }}
    pub fn as_loading_mut(&mut self) -> Option<&mut L> { if let Self::Loading(l) = self { Some(l) } else { None }}
    pub fn is_loading(&self) -> bool { self.as_loading().is_some() }

    pub fn as_loaded(&self) -> Option<&T> { if let Self::Loaded(t) = self { Some(t) } else { None }}
    pub fn as_loaded_mut(&mut self) -> Option<&mut T> { if let Self::Loaded(t) = self { Some(t) } else { None }}
    pub fn is_loaded(&self) -> bool { self.as_loaded().is_some() }

    pub fn as_error(&self) -> Option<&E> { if let Self::Error(e) = self { Some(e) } else { None }}
    pub fn as_error_mut(&mut self) -> Option<&mut E> { if let Self::Error(e) = self { Some(e) } else { None }}
    pub fn is_error(&self) -> bool { self.as_error().is_some() }

    pub fn get(&self) -> Option<&T> { self.as_loaded() }
    pub fn get_mut(&mut self) -> Option<&mut T> { self.as_loaded_mut() }
}
