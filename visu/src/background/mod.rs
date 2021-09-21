use web_sys::WebGlRenderingContext as GL;
use web_sys::{WebGlBuffer, WebGlProgram};

pub struct Background {
    shader: WebGlProgram,
    vertex_buffer: WebGlBuffer,
}

impl Background {
    pub fn new(gl: &GL) -> Self {
        // Shader
        let vert_code = include_str!("./basic.vert");
        let frag_code = include_str!("./basic.frag");
        let vert_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
        gl.shader_source(&vert_shader, &vert_code);
        gl.compile_shader(&vert_shader);
        let frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
        gl.shader_source(&frag_shader, &frag_code);
        gl.compile_shader(&frag_shader);
        let shader_program = gl.create_program().unwrap();
        gl.attach_shader(&shader_program, &vert_shader);
        gl.attach_shader(&shader_program, &frag_shader);
        gl.link_program(&shader_program);

        // Verts
        // This list of vertices will draw two triangles to cover the entire canvas.
        let vertices: Vec<f32> = vec![
            -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
        ];
        let vertex_buffer = gl.create_buffer().unwrap();
        let verts = js_sys::Float32Array::from(vertices.as_slice());
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts, GL::STATIC_DRAW);

        Self {
            shader: shader_program,
            vertex_buffer,
        }
    }

    pub fn render(&self, gl: &GL, timestamp: f64) {
        gl.use_program(Some(&self.shader));
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vertex_buffer));
        // Attach the position vector as an attribute for the GL context.
        let position = gl.get_attrib_location(&self.shader, "a_position") as u32;
        gl.vertex_attrib_pointer_with_i32(position, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(position);
        // Attach the time as a uniform for the GL context.
        let time = gl.get_uniform_location(&self.shader, "u_time");
        gl.uniform1f(time.as_ref(), timestamp as f32);
        gl.draw_arrays(GL::TRIANGLES, 0, 6);
    }
}
