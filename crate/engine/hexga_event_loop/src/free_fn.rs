use std::sync::Once;

use super::*;

static INIT_PANIC: Once = Once::new();

pub fn init()
{
    log::init();

    INIT_PANIC.call_once(||
        {
            let default_hook = std::panic::take_hook();

            std::panic::set_hook(Box::new(move |info| {
                /*
                #[cfg(not(target_arch = "wasm32"))]
                {
                    eprintln!("panic occurred: {info}");
                }
                */

                #[cfg(target_arch = "wasm32")]
                {
                    // Use the console_error_panic_hook for WASM
                    console_error_panic_hook::hook(info);
                }

                default_hook(info);
            }));
        }
    );
}