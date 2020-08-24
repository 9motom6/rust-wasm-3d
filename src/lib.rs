
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

#[macro_use]
extern crate lazy_static;

mod app_state;
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
    // program_color_2d_gradient: programs::Color2DGradient,
    program_graph_3d: programs::Graph3D,
}

#[wasm_bindgen]
impl GraphicsClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GraphicsClient {
        console_error_panic_hook::set_once();
        let gl = gl_setup::initialize_webgl_context().unwrap();

        log("[RUST] Graphics client was initialized");
        Self {
            program_color_2d: programs::Color2D::new(&gl),
            // program_color_2d_gradient: programs::Color2DGradient::new(&gl),
            program_graph_3d: programs::Graph3D::new(&gl),
            gl,
        }
    }

    pub fn update(&mut self, time: f32, height: f32, width: f32) -> Result<(), JsValue> {
        app_state::update_dynamic_data(time, height, width);
        Ok(())
    }

    pub fn render(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        let current_state = app_state::get_curr_state();

        self.program_color_2d.render(
            &self.gl,
            current_state.control_bottom,
            current_state.control_top,
            current_state.control_left,
            current_state.control_right,
            current_state.canvas_height,
            current_state.canvas_width,
        );

        // self.program_color_2d_gradient.render(
        //     &self.gl,
        //     current_state.control_bottom + 20.,
        //     current_state.control_top - 20.,
        //     current_state.control_left + 20.,
        //     current_state.control_right - 20.,
        //     current_state.canvas_height,
        //     current_state.canvas_width,
        // );

        self.program_graph_3d.render(
            &self.gl,
            current_state.control_bottom,
            current_state.control_top,
            current_state.control_left,
            current_state.control_right,
            current_state.canvas_height,
            current_state.canvas_width,
            current_state.rotation_x_axis,
            current_state.rotation_y_axis,
        );
    }
}