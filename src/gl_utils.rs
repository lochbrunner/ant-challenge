// use std::collections::HashMap;
use web_sys::console::*;
use web_sys::WebGlRenderingContext as GL;
use web_sys::{WebGlProgram, WebGlShader};

use itertools::izip;

// #[inline]
pub fn log(text: &str) {
    log_1(&text.into());
}

pub fn print_shader_error(gl: &GL, shader: &WebGlShader) {
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

pub fn print_program_error(gl: &GL, program: &WebGlProgram) {
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

pub fn create_shader(gl: &GL, vert_code: &str, frag_code: &str) -> WebGlProgram {
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
    shader
}

// #[derive(Hash, PartialEq, Eq)]
// struct IndexTriplet(i16, i16, i16);

pub fn synchronize_buffers(
    indices_a: &[u16],
    values_a: &[f32],
    indices_b: &[u16],
    values_b: &[f32],
    indices_c: &[u16],
    values_c: &[f32],
) -> (Vec<u16>, Vec<f32>, Vec<f32>, Vec<f32>) {
    let mut common_indices = Vec::new();
    let mut dense_values_a = Vec::new();
    let mut dense_values_b = Vec::new();
    let mut dense_values_c = Vec::new();

    // let mut used_value_pairs: HashMap<IndexTriplet, u16> = HashMap::new();

    for (i, (orig_index_a, orig_index_b, orig_index_c)) in
        izip!(indices_a, indices_b, indices_c).enumerate()
    {
        common_indices.push(i as u16);
        let orig_index_a = (*orig_index_a as usize) * 3;
        let value_a = &values_a[orig_index_a..(orig_index_a + 3)];
        dense_values_a.extend_from_slice(value_a);

        let orig_index_b = (*orig_index_b as usize) * 3;
        let value_b = &values_b[orig_index_b..(orig_index_b + 3)];
        dense_values_b.extend_from_slice(value_b);

        let orig_index_c = (*orig_index_c as usize) * 2;
        let value_c = &values_c[orig_index_c..(orig_index_c + 2)];
        dense_values_c.extend_from_slice(value_c);
        // TODO: Optimize use common indices
        // used_value_pairs.insert(BufferPair(value_a, value_b), i as u16);
    }

    (
        common_indices,
        dense_values_a,
        dense_values_b,
        dense_values_c,
    )
}
