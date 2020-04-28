use crate::camera::Camera;
use wasm_bindgen::JsCast;
use web_sys::console::*;
use web_sys::WebGlRenderingContext as GL;
use web_sys::{WebGlBuffer, WebGlProgram, WebGlShader};

pub struct SimpleMesh {
    shader: WebGlProgram,
    vertex_buffer: WebGlBuffer,
    index_buffer: WebGlBuffer,
    num_indices: i32,
}

fn print_shader_error(gl: &GL, shader: &WebGlShader) {
    if !gl
        .get_shader_parameter(shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        let error = gl
            .get_shader_info_log(shader)
            .unwrap_or_else(|| String::from("Unknown error creating program object"));
        log_1(&error.into());
    }
}

fn print_program_error(gl: &GL, program: &WebGlProgram) {
    if !gl
        .get_program_parameter(program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        let error = gl
            .get_program_info_log(program)
            .unwrap_or_else(|| String::from("Unknown error creating program object"));
        log_1(&error.into());
    }
}

impl SimpleMesh {
    pub fn cube(gl: &GL) -> SimpleMesh {
        // Shader
        let vert_code = include_str!("./simple.vert");
        let frag_code = include_str!("./simple.frag");

        let vert_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
        gl.shader_source(&vert_shader, &vert_code);
        gl.compile_shader(&vert_shader);
        let frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
        gl.shader_source(&frag_shader, &frag_code);
        gl.compile_shader(&frag_shader);
        let shader = gl.create_program().unwrap();
        gl.attach_shader(&shader, &vert_shader);
        gl.attach_shader(&shader, &frag_shader);
        gl.link_program(&shader);
        print_shader_error(gl, &vert_shader);
        print_shader_error(gl, &frag_shader);
        print_program_error(gl, &shader);

        // Vertices
        let x: f32 = 1.;
        let y: f32 = 1.;
        let z: f32 = 1.;
        let vertices: Vec<f32> = vec![
            x, y, z, x, y, -z, x, -y, z, x, -y, -z, -x, y, z, -x, y, -z, -x, -y, z, -x, -y, -z,
        ];
        let vertices = js_sys::Float32Array::from(vertices.as_slice());
        let vertex_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vertices, GL::STATIC_DRAW);

        // Indices
        let indices: Vec<u16> = vec![
            0, 2, 6, 6, 4, 0, // front
            1, 3, 7, 7, 5, 1, // front
            0, 1, 3, 3, 2, 0, // left
            4, 5, 7, 7, 6, 4, // right
            0, 4, 5, 5, 1, 0, // up
            2, 7, 6, 7, 2, 3, // down
        ];
        // let indices: Vec<u16> = vec![
        //     0, 2, 1, 2, 0, 3, // up
        // ];
        let num_indices = indices.len() as i32;
        let indices = js_sys::Uint16Array::from(indices.as_slice());

        let index_buffer = gl.create_buffer().unwrap();

        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &indices, GL::STATIC_DRAW);

        SimpleMesh {
            shader,
            vertex_buffer,
            index_buffer,
            num_indices,
        }
    }

    pub fn render(&self, gl: &GL, camera: &Camera) {
        gl.use_program(Some(&self.shader));
        // Bind uniform values
        let projection = gl.get_uniform_location(&self.shader, "projection");
        let view = gl.get_uniform_location(&self.shader, "view");

        gl.uniform_matrix4fv_with_f32_array(view.as_ref(), false, &camera.view());
        gl.uniform_matrix4fv_with_f32_array(projection.as_ref(), false, &camera.projection());

        // Vertices
        let position = gl.get_attrib_location(&self.shader, "position") as u32;
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vertex_buffer));
        gl.vertex_attrib_pointer_with_i32(position, 3, GL::FLOAT, false, 0, 0);
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.index_buffer));

        gl.draw_elements_with_i32(GL::TRIANGLES, self.num_indices, GL::UNSIGNED_SHORT, 0);
    }
}
