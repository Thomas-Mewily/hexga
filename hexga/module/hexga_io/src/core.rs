use crate::*;

/// While waiting for the std:never type to stabilize
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub enum IoNotBasedOn {}

pub type Path = String;
#[allow(non_camel_case_types)]
pub type path = str;

pub type Extension = String;
#[allow(non_camel_case_types)]
pub type extension = str;

pub type Reason = String;
pub type TypeName = String;