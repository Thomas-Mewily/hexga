use std::{any::Any, collections::HashMap, path::PathBuf, sync::Arc};




pub struct MemoryFileSystem
{
    entri: FsEntry,
}

pub struct FsEntry
{
    name:   String,
    childs: Vec<FsEntry>,
    bytes:  Option<Vec<u8>>,
    data:   Option<Arc<dyn Any>>,
}


pub struct PathData<T,P=PathBuf>
{
    path: P,
    data: T,
}
