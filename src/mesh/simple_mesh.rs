use std::collections::HashMap;
use web_sys::console::*;
use web_sys::WebGlRenderingContext as GL;
use web_sys::{WebGlBuffer, WebGlProgram, WebGlShader};

use crate::camera::Camera;

pub struct SimpleMesh {
    shader: WebGlProgram,
    vertex_buffer: WebGlBuffer,
    normal_buffer: WebGlBuffer,
    index_buffer: WebGlBuffer,
    num_indices: i32,
}

#[inline]
fn log(text: &str) {
    log_1(&text.into());
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
        log(&error);
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
        log(&error);
    }
}

// #[derive(Hash, PartialEq, Eq)]
// struct BufferPair(f32, f32);

fn synchronize_buffers(
    indices_a: &[u16],
    values_a: &[f32],
    indices_b: &[u16],
    values_b: &[f32],
) -> (Vec<u16>, Vec<f32>, Vec<f32>) {
    let mut common_indices = Vec::new();
    let mut dense_values_a = Vec::new();
    let mut dense_values_b = Vec::new();

    // let mut used_value_pairs: HashMap<BufferPair, u16> = HashMap::new();

    for (i, (orig_index_a, orig_index_b)) in indices_a.iter().zip(indices_b.iter()).enumerate() {
        common_indices.push(i as u16);
        let orig_index_a = (*orig_index_a as usize) * 3;
        let value_a = &values_a[orig_index_a..(orig_index_a + 3)];
        dense_values_a.extend_from_slice(value_a);
        let orig_index_b = (*orig_index_b as usize) * 3;
        let value_b = &values_b[orig_index_b..(orig_index_b + 3)];
        dense_values_b.extend_from_slice(value_b);
        // TODO: Optimize use common indices
        // used_value_pairs.insert(BufferPair(value_a, value_b), i as u16);
    }

    (common_indices, dense_values_a, dense_values_b)
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

        // Normals
        let normals: Vec<f32> = vec![
            0., 0., 1., // front
            0., 0., -1., // back
            1., 0., 0., // left
            -1., 0., 0., // right
            0., 1., 0., // up
            0., -1., 0., // down
        ];
        let normal_indices: Vec<u16> = vec![
            0, 0, 0, 0, 0, 0, // front
            1, 1, 1, 1, 1, 1, // back
            2, 2, 2, 2, 2, 2, // left
            3, 3, 3, 3, 3, 3, // right
            4, 4, 4, 4, 4, 4, // up
            5, 5, 5, 5, 5, 5, // down
        ];

        // Indices
        let vertex_indices: Vec<u16> = vec![
            0, 2, 6, 6, 4, 0, // front
            1, 3, 7, 7, 5, 1, // back
            0, 1, 3, 3, 2, 0, // left
            4, 5, 7, 7, 6, 4, // right
            0, 4, 5, 5, 1, 0, // up
            2, 7, 6, 7, 2, 3, // down
        ];

        // Synchronize
        let (indices, vertices, normals) =
            synchronize_buffers(&vertex_indices, &vertices, &normal_indices, &normals);

        log(&format!("Normals: {:?}", normals));
        log(&format!("Vertices: {:?}", vertices));

        let vertices = js_sys::Float32Array::from(vertices.as_slice());
        let vertex_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vertices, GL::STATIC_DRAW);

        let normals = js_sys::Float32Array::from(normals.as_slice());
        let normal_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&normal_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &normals, GL::STATIC_DRAW);

        let num_indices = indices.len() as i32;
        let indices = js_sys::Uint16Array::from(indices.as_slice());

        let index_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &indices, GL::STATIC_DRAW);

        SimpleMesh {
            shader,
            vertex_buffer,
            normal_buffer,
            index_buffer,
            num_indices,
        }
    }

    pub fn render(&self, gl: &GL, camera: &Camera) {
        gl.use_program(Some(&self.shader));
        // Bind uniform values
        let projection = gl.get_uniform_location(&self.shader, "projection");
        let view = gl.get_uniform_location(&self.shader, "view");
        let ambient_loc = gl.get_uniform_location(&self.shader, "ambient");
        let point_light_loc = gl.get_uniform_location(&self.shader, "point_light");
        let point_light_dir_loc = gl.get_uniform_location(&self.shader, "point_light_dir");
        let camera_pos_loc = gl.get_uniform_location(&self.shader, "camera_pos");

        let ambient = [0.24725, 0.1995, 0.0745];
        let point_light = [1.0, 1.0, 1.0];
        let point_light_dir = [-1.0, -1.0, 0.5];
        let camera_pos = camera.get_eye_pos();
        let camera_pos = [camera_pos.x, camera_pos.y, camera_pos.z];

        gl.uniform3fv_with_f32_array(camera_pos_loc.as_ref(), &camera_pos);
        gl.uniform3fv_with_f32_array(point_light_dir_loc.as_ref(), &point_light_dir);
        gl.uniform3fv_with_f32_array(point_light_loc.as_ref(), &point_light);
        gl.uniform3fv_with_f32_array(ambient_loc.as_ref(), &ambient);
        gl.uniform_matrix4fv_with_f32_array(view.as_ref(), false, &camera.view());
        gl.uniform_matrix4fv_with_f32_array(projection.as_ref(), false, &camera.projection());

        // Vertices
        let position = gl.get_attrib_location(&self.shader, "position") as u32;
        gl.enable_vertex_attrib_array(position as u32);
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vertex_buffer));
        gl.vertex_attrib_pointer_with_i32(position, 3, GL::FLOAT, false, 0, 0);

        // Normals
        let normal = gl.get_attrib_location(&self.shader, "normal") as u32;
        gl.enable_vertex_attrib_array(normal as u32);
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.normal_buffer));
        gl.vertex_attrib_pointer_with_i32(normal, 3, GL::FLOAT, false, 0, 0);

        // Indices
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.index_buffer));
        gl.draw_elements_with_i32(GL::TRIANGLES, self.num_indices, GL::UNSIGNED_SHORT, 0);
    }
}
