#![recursion_limit = "1024"]

mod background;
mod camera;
mod gl_utils;
mod ground;
mod mesh;
mod scene;
mod texture;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// use console_error_panic_hook::set_once as set_panic_hook;

mod app;

fn main() {
    // web_logger::init();
    yew::start_app::<app::App>();
}
