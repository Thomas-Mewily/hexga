use crate::*;

pub trait Policy
{
    const CAN_PANIC : bool;
    const DEBUG_PREFIX : &'static str = "";
    const DEBUG_SUFFIX : &'static str = "";
}


/// [Try] can never fail/panic
#[derive(Debug, Default)]
pub struct Try;
impl Policy for Try { const CAN_PANIC : bool = false; const DEBUG_PREFIX : &'static str = "Try"; }

/// [Normal] can never fail/panic
#[derive(Debug, Default)]
pub struct Normal;
impl Policy for Normal { const CAN_PANIC : bool = false; }

/// [Unsafe] can fail/panic
#[derive(Debug, Default)]
pub struct Unchecked;
impl Policy for Unchecked { const CAN_PANIC : bool = true; const DEBUG_SUFFIX : &'static str = "Unchecked"; }

/// [Panic] can fail/panic
#[derive(Debug, Default)]
pub struct Panic;
impl Policy for Panic { const CAN_PANIC : bool = true; const DEBUG_SUFFIX : &'static str = "OrPanic"; }
