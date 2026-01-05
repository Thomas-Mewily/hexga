//! Unstable functionality exposed for extensibility; the API may change significantly.
use hexga::singleton::SingletonMultiThreadWriteResult;

use crate::app::AppCore;

use super::*;

pub fn try_context_mut() -> SingletonMultiThreadWriteResult<'static> { APP.try_write() }
pub fn try_context() -> impl Deref<Target=AppCore> { APP.read() }

pub fn context_mut() -> impl DerefMut<Target=AppCore> { APP.write() }
pub fn context() -> impl Deref<Target=AppCore> { APP.read() }