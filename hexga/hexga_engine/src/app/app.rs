use super::*;


singleton_thread_local!(pub App,AppContext,CONTEXT_APP);


impl SingletonInit for App
{
    fn replace(instance: Option<<Self as SingletonRef>::Target>) {
        match instance
        {
            Some(ctx) => 
            {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    use std::io::Write;

                    std::panic::set_hook(Box::new(|info| {
                        App::destroy();
                        eprintln!("panic occurred: {info}");
                    }));


                    let _res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        App::destroy();
                    }));
                }
                #[cfg(target_arch = "wasm32")]
                {
                    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
                    console_log::init_with_level(::log::Level::Debug).expect("Couldn't initialize logger");
                }
                CONTEXT_APP.replace(Some(ctx));
                // The Gpu is initialized in a special async way... 
            },
            None => 
            {
                CONTEXT_APP.replace(None);
                Gpu::destroy();
            },
        }
    }
}