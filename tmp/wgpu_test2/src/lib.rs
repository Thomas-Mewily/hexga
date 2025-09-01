mod shared;
pub use shared::*;

#[cfg(target_os = "android")]
ndk_glue::ndk_main!(android_main);

#[cfg(target_os = "android")]
fn android_main(app: ndk_glue::App) {
    // Optional: initialize logging
    ndk_glue::init_logging("wgpu_test2");

    // Run your shared code (winit + wgpu, etc.)
    run();
}