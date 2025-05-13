//! mainly inspired by miniquad
use crate::*;

pub type MetaVersion = String;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Backend {
    Metal(MetaVersion),
    OpenGl(),
}

pub trait RenderBackend
{
    
}