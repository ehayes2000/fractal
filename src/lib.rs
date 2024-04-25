use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram, WebGlShader};
mod utils;

const VERTEX_SHADER: &str = include_str!("vertex.glsl");
const FRAGMENT_SHADER: &str = include_str!("mandelbrot_fragment.glsl");
const MAX_ITERATIONS: u32 = 600;
const VERTICES: [f32; 18] = [
    -1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, -1.0, 0.0, -1.0, 1.0, 0.0, -1.0, -1.0, 0.0, 1.0, -1.0, 0.0,
];

#[wasm_bindgen]
pub struct Mandelbrot {
    program: WebGlProgram,
    gl: WebGl2RenderingContext,
    height: u32,
    width: u32,
    max_iterations: u32,
}

#[wasm_bindgen]
impl Mandelbrot {
    pub fn new(canvas: &HtmlCanvasElement) -> Result<Mandelbrot, JsValue> {
        let gl = canvas
            .get_context_with_context_options("webgl2", &JsValue::from_str("{antialias:true}"))?
            .expect("webgl2 support")
            .dyn_into::<WebGl2RenderingContext>()?;

        let vert_shader =
            Self::compile_shader(&gl, WebGl2RenderingContext::VERTEX_SHADER, VERTEX_SHADER)?;

        let frag_shader = Self::compile_shader(
            &gl,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            FRAGMENT_SHADER,
        )?;

        let program = Self::link_program(&gl, &vert_shader, &frag_shader)?;
        gl.use_program(Some(&program));

        Ok(Mandelbrot {
            program: program,
            gl: gl,
            height: canvas.height(),
            width: canvas.width(),
            max_iterations: 400,
        })
    }
    pub fn draw(&self, x: f32, xs: f32, y: f32, ys: f32) -> Result<(), JsValue> {
        let position_attribute_location = self.gl.get_attrib_location(&self.program, "position");
        let buffer = self.gl.create_buffer().ok_or("Failed to create buffer")?;
        self.gl
            .bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
        unsafe {
            let positions_array_buf_view = js_sys::Float32Array::view(&VERTICES);
            self.gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &positions_array_buf_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }
        let vao = self
            .gl
            .create_vertex_array()
            .ok_or("Could not create vertex array object")?;
        self.gl.bind_vertex_array(Some(&vao));

        self.gl.vertex_attrib_pointer_with_i32(
            position_attribute_location as u32,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );

        self.gl
            .enable_vertex_attrib_array(position_attribute_location as u32);

        self.gl.bind_vertex_array(Some(&vao));

        let vert_count = (VERTICES.len() / 3) as i32;
        let canvas_size_loc = self
            .gl
            .get_uniform_location(&self.program, "canvasSize")
            .expect("fragment shader canvas size uniform");
        self.gl.uniform2f(
            Some(&canvas_size_loc),
            self.width as f32,
            self.height as f32,
        );
        let view_bounds_loc = self
            .gl
            .get_uniform_location(&self.program, "viewportBounds")
            .expect("fragment shader viewbounds uniform");
        self.gl.uniform4f(Some(&view_bounds_loc), x, xs, y, ys);
        let max_iter_loc = self
            .gl
            .get_uniform_location(&self.program, "MAX_ITERATIONS")
            .expect("fragment shader MAX_ITERATIONS");
        self.gl
            .uniform1i(Some(&max_iter_loc), MAX_ITERATIONS as i32);
        Self::render(&self.gl, vert_count);
        Ok(())
    }
    pub fn test_draw(&self) -> Result<(), JsValue> {
        let position_attribute_location = self.gl.get_attrib_location(&self.program, "position");
        let buffer = self.gl.create_buffer().ok_or("Failed to create buffer")?;
        self.gl
            .bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
        unsafe {
            let positions_array_buf_view = js_sys::Float32Array::view(&VERTICES);
            self.gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &positions_array_buf_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }
        let vao = self
            .gl
            .create_vertex_array()
            .ok_or("Could not create vertex array object")?;
        self.gl.bind_vertex_array(Some(&vao));

        self.gl.vertex_attrib_pointer_with_i32(
            position_attribute_location as u32,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );

        self.gl
            .enable_vertex_attrib_array(position_attribute_location as u32);

        self.gl.bind_vertex_array(Some(&vao));

        let vert_count = (VERTICES.len() / 3) as i32;
        let canvas_size_loc = self
            .gl
            .get_uniform_location(&self.program, "canvasSize")
            .expect("fragment shader canvas size uniform");
        self.gl.uniform2f(
            Some(&canvas_size_loc),
            self.width as f32,
            self.height as f32,
        );
        let view_bounds_loc = self
            .gl
            .get_uniform_location(&self.program, "viewportBounds")
            .expect("fragment shader viewbounds uniform");
        self.gl.uniform4f(
            Some(&view_bounds_loc),
            -0.750222,
            0.001031,
            0.031161,
            0.000591,
        );
        let max_iter_loc = self
            .gl
            .get_uniform_location(&self.program, "MAX_ITERATIONS")
            .expect("fragment shader MAX_ITERATIONS");
        self.gl
            .uniform1i(Some(&max_iter_loc), MAX_ITERATIONS as i32);
        // let scaling_factor = self
        //     .gl
        //     .get_uniform_location(&self.program, "scaling_factor")
        //     .expect("scaling factor");
        // self.gl.unirom1i(Some(&scaling_factor), 1000 as i32);
        Self::render(&self.gl, vert_count);
        Ok(())
    }
}

impl Mandelbrot {
    fn render(context: &WebGl2RenderingContext, vert_count: i32) {
        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count);
    }

    fn compile_shader(
        context: &WebGl2RenderingContext,
        shader_type: u32,
        source: &str,
    ) -> Result<WebGlShader, String> {
        let shader = context
            .create_shader(shader_type)
            .ok_or_else(|| String::from("Could not create shader object"))?;
        context.shader_source(&shader, source);
        context.compile_shader(&shader);

        if context
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            Err(context
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unknown error creating shader")))
        }
    }

    fn link_program(
        context: &WebGl2RenderingContext,
        vert_shader: &WebGlShader,
        frag_shader: &WebGlShader,
    ) -> Result<WebGlProgram, String> {
        let program = context
            .create_program()
            .ok_or_else(|| String::from("Unable to create program"))?;

        context.attach_shader(&program, vert_shader);
        context.attach_shader(&program, frag_shader);
        context.link_program(&program);

        if context
            .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(program)
        } else {
            Err(context
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program object")))
        }
    }
}
