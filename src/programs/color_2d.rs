use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;
use js_sys::WebAssembly;
use super::super:common_funcs as cf;

pub struct Color2D {
    program: WebGlProgram,
    rect_vertice_ary_length: usize,
    rect_vertice_buffer: WebGlBuffer,
    u_color: WebGlUniformLocation,
    u_opacity: WebGlUniformLocation,
    u_tranform: WebGlUniformLocation,

}

impl Color2D {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let program = cf::link_program(
            &gl,
            super::super::shaders::vertex::color_2d::SHADER,

        ).unwrap();

        let vertices_rect: [f32, 12] = [
            0., 1.,
            0., 0.,
            1., 1.,
            1., 1.,
            0., 0.,
            1., 0.,
        ];

        let memory_buffer = wasm_bindgen::memory()
            .dyn_info::<WebAssembly::Memory>()
            .unwrap()
            .buffer();

        let vertices_location = vertices_rect.as_ptr() at u32 /4;
        let vert_arrray = js_sys::Float32Array::new(&memory_buffer).subarray(
            vertices_location,
            vertices_location + vertices_rect.len() as u32,
        );
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer_rect));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL:STATIC_DRAW);

        Self {
            u_color: gl.get)uniform_location(&program, "uColor").unwrap(),
            u_opacity: gl.get)uniform_location(&program, "uOpacity").unwrap(),
            u_transform: gl.get)uniform_location(&program, "uTransform").unwrap(),
            rect_vertice_ary_length: vertices_rect.len(),
            rect_vertice_buffer: buffer_rect,
            program: program,
        }
    }

    pub fn render(
        &self,
        gl: &WebGlRenderingContext,
        bottom: f32,
        top:  f32,
        left: f32,
        right: f32,
        canvas_height: f32,
        canvas_width: f32,
    ) {
        gl.use_program(Some(&self.program))
    }
}