use crate::{Log, ScopedWith};
use std::{cell::RefCell, fmt::Display};

thread_local! {
    static PREFIXES: RefCell<Vec<Prefix>> = const { RefCell::new(Vec::new()) };
}

struct Prefix
{
    text: String,
}

fn push_prefix(value: &impl Display) -> usize
{
    let text = value.to_string();
    PREFIXES.with_borrow_mut(|prefixes| {
        let depth = prefixes.len();
        prefixes.push(Prefix { text });
        depth
    })
}

impl<T: Display> ScopedWith<T> for Log
{
    fn begin(&mut self, value: &T) { push_prefix(value); }

    fn end(&mut self)
    {
        PREFIXES.with_borrow_mut(|prefixes| {
            if prefixes.last().is_some()
            {
                prefixes.pop();
            }
        });
    }
}

pub(crate) fn current_prefix() -> Option<String>
{
    PREFIXES
        .try_with(|prefixes| {
            let prefixes = prefixes.borrow();
            let (first, rest) = prefixes.split_first()?;
            let mut result = first.text.clone();
            for prefix in rest
            {
                result.push_str("/");
                result.push_str(&prefix.text);
            }
            Some(result)
        })
        .ok()
        .flatten()
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn nested_prefixes_are_local_to_the_thread_and_restore_after_unwinding()
    {
        Log.scope("request", |_| {
            assert_eq!(current_prefix().as_deref(), Some("request"));
            assert_eq!(std::thread::spawn(current_prefix).join().unwrap(), None);

            let result = std::panic::catch_unwind(|| {
                Log.scope(42, |_| {
                    assert_eq!(current_prefix().as_deref(), Some("request/42"));
                    Log.begin(&"manual");
                    panic!("test unwind");
                })
            });
            assert!(result.is_err());
            assert_eq!(current_prefix().as_deref(), Some("request"));
        });
        assert_eq!(current_prefix(), None);
    }

    #[test]
    fn log_handles_share_the_thread_local_prefix_stack()
    {
        Log.begin(&"outer");
        Log.begin(&"inner");
        assert_eq!(current_prefix().as_deref(), Some("outer/inner"));
        ScopedWith::<&str>::end(&mut Log);
        assert_eq!(current_prefix().as_deref(), Some("outer"));
        ScopedWith::<&str>::end(&mut Log);
        assert_eq!(current_prefix(), None);
    }
}
