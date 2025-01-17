pub mod app;
pub mod db;
pub mod mock;
pub mod models;
#[cfg(feature = "ssr")]
pub mod entities;
#[cfg(feature = "ssr")]
pub mod seaorm;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
