use super::*;

pub struct Assets<T> where T:Async
{
    _marker: PhantomData<T>,
}
impl<T> Debug for Assets<T> where T:Async
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Assets<{}>", std::any::type_name::<T>())
    }
}
impl<T> Default for Assets<T> where T:Async
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T> Clone for Assets<T> where T:Async
{
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> Copy for Assets<T> where T:Async {}
impl<T> Assets<T> where T:Async
{
    pub const fn new() -> Self { Self { _marker: PhantomData }}
}
impl<T> Deref for Assets<T> where T:Async
{
    type Target=AssetManager<T>;
    fn deref(&self) -> &Self::Target {
        AssetsUntyped::as_mut().manager()
    }
}
impl<T> DerefMut for Assets<T> where T:Async
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        AssetsUntyped::as_mut().manager_mut()
    }
}
