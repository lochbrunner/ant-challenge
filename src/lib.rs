#![recursion_limit = "512"]

mod app;
mod camera;
mod gl_utils;
mod ground;
mod mesh;
mod scene;
mod texture;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    web_logger::init();
    yew::start_app::<app::App>();
    Ok(())
}
