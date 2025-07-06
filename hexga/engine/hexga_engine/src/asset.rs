use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Asset<T,L=(),E=()>
{
    Loadding(L),
    Loaded(T),
    Error(E)
}

impl<T,L,E> Asset<T,L,E>
{
    pub fn as_loading(&self) -> Option<&L> { if let Asset::Loadding(loading) = self { Some(loading) } else { None } }
    pub fn as_localized_event_mut(&mut self) -> Option<&mut L> { if let Asset::Loadding(loading) = self { Some(loading) } else { None } }
    pub fn is_loading(&self) -> bool { self.as_loading().is_some() }

    pub fn as_loaded(&self) -> Option<&T> { if let Asset::Loaded(loaded) = self { Some(loaded) } else { None } }
    pub fn as_loaded_mut(&mut self) -> Option<&mut T> { if let Asset::Loaded(loaded) = self { Some(loaded) } else { None } }
    pub fn is_loaded(&self) -> bool { self.as_loaded().is_some() }

    pub fn as_error(&self) -> Option<&E> { if let Asset::Error(error) = self { Some(error) } else { None } }
    pub fn as_error_mut(&mut self) -> Option<&mut E> { if let Asset::Error(error) = self { Some(error) } else { None } }
    pub fn is_error(&self) -> bool { self.as_error().is_some() }
}
impl<T,L,E> Default for Asset<T,L,E> where T: Default
{
    fn default() -> Self {
        Self::Loaded(___())
    }
}

