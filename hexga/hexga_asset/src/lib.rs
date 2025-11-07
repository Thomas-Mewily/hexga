#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use std::{any::{Any, TypeId}, collections::HashMap, marker::PhantomData, pin::Pin};

use hexga_core::prelude::*;
use hexga_generational::prelude::*;

pub struct AssetManager
{
    managers: HashMap<TypeId, Pin<Box<dyn Any>>>
}

pub struct AssetData<T:Async>
{
    state: AssetState<T>,
}
pub enum AssetState<T:Async>
{
    Loading(DynFuture<T>),
    Loaded(T),
    Error,
}


pub struct AssetManagerOf<T>
{
    assets : Table<Asset<T>>,
    default_value: T,
    error_value: T,
}

pub struct Asset<T>
{
    id : TableID,
    phantom: PhantomData<T>,
}

pub struct AssetUntyped
{
    type_id: std::any::TypeId,
    id: TableID,
}