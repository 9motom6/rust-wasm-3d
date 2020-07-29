extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct GraphicsClient {
}

#[wasm_bindgen]
impl GraphicsClient {

    #[wasm_bindgen(constructor)]
    pub fn new() -> GraphicsClient {
        log("[RUST] Graphics client was initialized");
        GraphicsClient {}
    }

    pub fn update(&mut self, time: f32, height: f32, width: f32) -> Result<(), JsValue> {
        Ok(())
    }

    pub fn render(&self) {
    }
}