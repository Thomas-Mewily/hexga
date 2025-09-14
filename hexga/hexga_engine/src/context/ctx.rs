use super::*;

singleton_thread_local!(pub Ctx,Context,CONTEXT_APP);

impl SingletonInit for Ctx
{
    fn replace(instance: Option<<Self as SingletonRef>::Target>) {
        match instance
        {
            Some(ctx) => 
            {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    use std::io::Write;

                    env_logger::Builder::from_env(
                        env_logger::Env::default().default_filter_or("debug")
                    )
                    .filter_module("wgpu_core", ::log::LevelFilter::Warn)
                    .filter_module("wgpu_hal", ::log::LevelFilter::Warn)
                    .filter_module("naga", ::log::LevelFilter::Warn)
                    .format(|buf, record| {
                        writeln!(buf, "{}", record.args())
                    })
                    .init();

                    std::panic::set_hook(Box::new(|info| {
                        Ctx::destroy();
                        eprintln!("panic occurred: {info}");
                    }));


                    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        Ctx::destroy();
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