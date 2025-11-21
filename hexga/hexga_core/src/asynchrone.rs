#[allow(unused_imports)]
use std::{any::Any, pin::Pin};

pub trait Async : Send + Sync + 'static {}
impl<T> Async for T where T: Send + Sync + 'static {}

pub type AnyAsync = dyn Any + Send + Sync + 'static;

/*
pub struct DynFuture<T>
{
    inner: Pin<Box<dyn Future<Output = T> + Send>>
}
impl<T> DynFuture<T> {
    pub fn new<F>(fut: F) -> Self
    where
        F: Future<Output = T> + Send + 'static,
    {
        DynFuture {
            inner: Box::pin(fut),
        }
    }
}
impl<T> Future for DynFuture<T> {
    type Output = T;
    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        self.get_mut().inner.as_mut().poll(cx)
    }
}
*/