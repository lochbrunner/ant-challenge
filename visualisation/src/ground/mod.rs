use web_sys::WebGlRenderingContext as GL;
use web_sys::{WebGlBuffer, WebGlProgram};

use crate::camera::Camera;
use crate::gl_utils;
use crate::texture::Texture;

pub struct Ground {
    shader: WebGlProgram,
    vertex_buffer: WebGlBuffer,
    normal_buffer: WebGlBuffer,
    tex_coords_buffer: WebGlBuffer,
    index_buffer: WebGlBuffer,
    num_indices: i32,
    grass_texture: Texture,
    mud_texture: Texture,
    stone_texture: Texture,
    map_texture: Texture,
}

impl Ground {
    pub fn new(gl: &GL) -> Ground {
        // Shader
        let vert_code = include_str!("./ground.vert");
        let frag_code = include_str!("./ground.frag");

        let shader = gl_utils::create_shader(gl, vert_code, frag_code);

        // Using blenders z up
        let x: f32 = 64.;
        let y: f32 = 64.;

        let vertices: Vec<f32> = vec![x, y, 0., x, -y, 0., -x, y, 0., -x, -y, 0.];

        let vertex_indices: Vec<u16> = vec![0, 1, 3, 3, 2, 0];

        let normals: Vec<f32> = vec![0., 0., 1.];
        let normal_indices: Vec<u16> = vec![0, 0, 0, 0, 0, 0];

        let tex_coords: Vec<f32> = vec![0., 0., 0., 1., 1., 0., 1., 1.];
        let tex_coords_indices: Vec<u16> = vec![0, 1, 3, 3, 2, 0];

        // Synchronize
        let (indices, vertices, normals, tex_coords) = gl_utils::synchronize_buffers(
            &vertex_indices,
            &vertices,
            &normal_indices,
            &normals,
            &tex_coords_indices,
            &tex_coords,
        );

        // gl_utils::log(&format!("vertices: {:?}", vertices));
        // gl_utils::log(&format!("tex_coords: {:?}", tex_coords));

        let vertices = js_sys::Float32Array::from(&vertices[..]);
        let vertex_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vertices, GL::STATIC_DRAW);

        let normals = js_sys::Float32Array::from(&normals[..]);
        let normal_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&normal_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &normals, GL::STATIC_DRAW);

        let tex_coords = js_sys::Float32Array::from(&tex_coords[..]);
        let tex_coords_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&tex_coords_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &tex_coords, GL::STATIC_DRAW);

        let num_indices = indices.len() as i32;
        let indices = js_sys::Uint16Array::from(&indices[..]);

        let index_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &indices, GL::STATIC_DRAW);

        let grass_texture = Texture::new(gl, "./gras-green.jpg").unwrap();
        let mud_texture = Texture::new(gl, "./mud.png").unwrap();
        let stone_texture = Texture::new(gl, "./stone.jpeg").unwrap();
        let map_texture = Texture::new(gl, "./map.png").unwrap();

        Ground {
            shader,
            vertex_buffer,
            normal_buffer,
            tex_coords_buffer,
            index_buffer,
            num_indices,
            grass_texture,
            mud_texture,
            stone_texture,
            map_texture,
        }
    }

    pub fn render(&self, gl: &GL, camera: &Camera) {
        gl.use_program(Some(&self.shader));

        // Texture
        let tex_scale_loc = gl.get_uniform_location(&self.shader, "tex_scale");
        gl.uniform2fv_with_f32_array(tex_scale_loc.as_ref(), &[4., 4.]);

        // Bind uniform values
        let projection = gl.get_uniform_location(&self.shader, "projection");
        let view = gl.get_uniform_location(&self.shader, "view");

        let ambient_loc = gl.get_uniform_location(&self.shader, "ambient");
        let point_light_loc = gl.get_uniform_location(&self.shader, "point_light");
        let point_light_dir_loc = gl.get_uniform_location(&self.shader, "point_light_dir");

        let ambient = [0.6, 0.6, 0.6];
        let point_light = [1.0, 1.0, 1.0];
        let point_light_dir = [-1.0, -1.0, 0.5];

        // Light
        gl.uniform3fv_with_f32_array(point_light_dir_loc.as_ref(), &point_light_dir);
        gl.uniform3fv_with_f32_array(point_light_loc.as_ref(), &point_light);
        gl.uniform3fv_with_f32_array(ambient_loc.as_ref(), &ambient);
        // Camera
        gl.uniform_matrix4fv_with_f32_array(projection.as_ref(), false, &camera.projection());
        gl.uniform_matrix4fv_with_f32_array(view.as_ref(), false, &camera.view());

        // Vertices
        let position = gl.get_attrib_location(&self.shader, "position") as u32;
        gl.enable_vertex_attrib_array(position);
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vertex_buffer));
        gl.vertex_attrib_pointer_with_i32(position, 3, GL::FLOAT, false, 0, 0);

        // Normals
        let normal = gl.get_attrib_location(&self.shader, "normal") as u32;
        gl.enable_vertex_attrib_array(normal);
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.normal_buffer));
        gl.vertex_attrib_pointer_with_i32(normal, 3, GL::FLOAT, false, 0, 0);

        // Texture
        let tex_coordinate = gl.get_attrib_location(&self.shader, "tex_coordinate") as u32;
        gl.enable_vertex_attrib_array(tex_coordinate);
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.tex_coords_buffer));
        gl.vertex_attrib_pointer_with_i32(tex_coordinate, 2, GL::FLOAT, false, 0, 0);

        let grass_loc = gl
            .get_uniform_location(&self.shader, "grass_texture")
            .unwrap();
        gl.active_texture(GL::TEXTURE0);
        gl.bind_texture(GL::TEXTURE_2D, Some(self.grass_texture.texture.as_ref()));
        gl.uniform1i(Some(&grass_loc), 0);

        let mud_loc = gl
            .get_uniform_location(&self.shader, "mud_texture")
            .unwrap();
        gl.active_texture(GL::TEXTURE1);
        gl.bind_texture(GL::TEXTURE_2D, Some(self.mud_texture.texture.as_ref()));
        gl.uniform1i(Some(&mud_loc), 1);

        let map_loc = gl
            .get_uniform_location(&self.shader, "map_texture")
            .unwrap();
        gl.active_texture(GL::TEXTURE2);
        gl.bind_texture(GL::TEXTURE_2D, Some(self.map_texture.texture.as_ref()));
        gl.uniform1i(Some(&map_loc), 2);

        let stone_loc = gl
            .get_uniform_location(&self.shader, "stone_texture")
            .unwrap();
        gl.active_texture(GL::TEXTURE3);
        gl.bind_texture(GL::TEXTURE_2D, Some(self.stone_texture.texture.as_ref()));
        gl.uniform1i(Some(&stone_loc), 3);

        // Indices
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.index_buffer));
        gl.draw_elements_with_i32(GL::TRIANGLES, self.num_indices, GL::UNSIGNED_SHORT, 0);
    }
}
