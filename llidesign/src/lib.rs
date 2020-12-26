#![recursion_limit="1024"]

mod app;
mod components;

use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    yew::start_app::<app::App>();
    Ok(())
}
