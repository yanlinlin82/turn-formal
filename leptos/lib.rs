use leptos::prelude::*;
use leptos::*;
use wasm_bindgen::prelude::*;

// We don't need to maintain components and pages modules here
// since they're now in main.rs
use crate::leptos::app::App;

#[cfg(feature = "visualization")]
pub mod app;

#[cfg(feature = "visualization")]
pub mod components;

#[cfg(feature = "visualization")]
pub mod pages;

#[cfg(feature = "visualization")]
mod adapters;

#[wasm_bindgen]
pub fn main() {
    _ = console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> });
}
