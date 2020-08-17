extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

mod gl_setup;
mod shaders;
mod common_funcs;
mod programs;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct GraphicsClient {
    gl: WebGlRenderingContext,
    program_color_2d: programs::Color2D,
}

#[wasm_bindgen]
impl GraphicsClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GraphicsClient {
        console_error_panic_hook::set_once();
        let gl = gl_setup::initialize_webgl_context().unwrap();

        log("[RUST] Graphics client was initialized");
        GraphicsClient {
            program_color_2d: programs::Color2D::new(&gl),
            gl,
        }
    }

    pub fn update(&mut self, time: f32, height: f32, width: f32) -> Result<(), JsValue> {
        Ok(())
    }

    pub fn render(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        self.program_color_2d.render(
            &self.gl, 0., 10., 0., 10., 10., 10.,
        )
    }
}