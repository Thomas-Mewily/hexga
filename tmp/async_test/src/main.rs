#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use std::future::Future;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

type DynFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;

pub trait Loadable: Sized + 'static
{
    fn load(path: String) -> Asset<Self>;
}

#[derive(Debug)]
pub struct UrlTitle
{
    pub title: String,
}

impl Loadable for UrlTitle
{
    fn load(path: String) -> Asset<Self>
    {
        let p = path.clone();
        let fut = Box::pin(async move {
            UrlTitle {
                title: format!("Title of {}", p),
            }
        });

        Asset {
            path,
            content: AssetState::Loading(fut),
        }
    }
}

pub struct Asset<T: 'static>
{
    pub path: String,
    pub content: AssetState<T>,
}

pub enum AssetState<T: 'static>
{
    Loading(DynFuture<T>),
    Loaded(T),
    Error,
}

type AsyncCtx<'a> = std::task::Context<'a>;

impl<T: 'static> AssetState<T>
{
    pub fn poll(mut self, ctx: &mut AsyncCtx) -> Self
    {
        match self
        {
            AssetState::Loading(mut fut) => match fut.as_mut().poll(ctx)
            {
                std::task::Poll::Ready(r) => Self::Loaded(r),
                std::task::Poll::Pending => Self::Loading(fut),
            },
            _ => self,
        }
    }

    pub fn as_mut(&mut self) -> Option<&mut T>
    {
        match self
        {
            AssetState::Loaded(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_ref(&self) -> Option<&T>
    {
        match self
        {
            AssetState::Loaded(v) => Some(v),
            _ => None,
        }
    }
}

impl<T: 'static> Asset<T>
{
    pub fn poll(mut self, ctx: &mut AsyncCtx) { self.content = self.content.poll(ctx); }
}
impl<T: 'static> Deref for Asset<T>
{
    type Target = AssetState<T>;
    fn deref(&self) -> &Self::Target { &self.content }
}
impl<T: 'static> DerefMut for Asset<T>
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.content }
}

// -----------------------------------------------------------------------------
// Example main demonstrating how to use all of this
// -----------------------------------------------------------------------------
fn main()
{
    // Create an "asset" that's still loading
    let mut asset = UrlTitle::load("https://example.com".into());

    // Create a dummy waker and context for polling

    fn dummy_raw_waker() -> RawWaker
    {
        fn no_op(_: *const ()) {}
        fn clone(_: *const ()) -> RawWaker { dummy_raw_waker() }
        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, no_op, no_op, no_op);
        RawWaker::new(std::ptr::null(), &VTABLE)
    }

    let waker = unsafe { Waker::from_raw(dummy_raw_waker()) };
    let mut ctx = Context::from_waker(&waker);

    // Poll until loaded (simulate executor)
    loop
    {
        asset.content = asset.content.poll(&mut ctx);
        if let AssetState::Loaded(_) = asset.content
        {
            break;
        }
    }

    dbg!(asset.as_ref());
}
