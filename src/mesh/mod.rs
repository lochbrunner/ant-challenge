use bincode;
use blender_mesh::{BlenderMesh, CreateSingleIndexConfig};
use nalgebra::{Similarity3, Vector3};
use std::collections::HashMap;
use web_sys::WebGlRenderingContext as GL;
use web_sys::{WebGlBuffer, WebGlProgram};

use crate::camera::Camera;
use crate::gl_utils;
use crate::texture::Texture;

pub struct Transformation {
    pub translation: Vector3<f32>,
    pub rotation: Vector3<f32>,
}

impl Transformation {
    pub fn new(translation: (f32, f32, f32), rotation: (f32, f32, f32)) -> Transformation {
        Transformation {
            translation: Vector3::new(translation.0, translation.1, translation.2),
            rotation: Vector3::new(rotation.0, rotation.1, rotation.2),
        }
    }
}

pub struct SimpleMesh {
    shader: WebGlProgram,
    vertex_buffer: WebGlBuffer,
    normal_buffer: WebGlBuffer,
    tex_coords_buffer: WebGlBuffer,
    index_buffer: WebGlBuffer,
    num_indices: i32,
    texture: Texture,
}

impl SimpleMesh {
    fn create_resources(
        gl: &GL,
        texture: &str,
        indices: &[u16],
        vertices: &[f32],
        normals: &[f32],
        tex_coords: &[f32],
    ) -> SimpleMesh {
        // Shader
        let vert_code = include_str!("./simple.vert");
        let frag_code = include_str!("./simple.frag");

        let shader = gl_utils::create_shader(gl, vert_code, frag_code);

        // Synchronize
        // let (indices, vertices, normals, tex_coords) = gl_utils::synchronize_buffers(
        //     vertex_indices,
        //     vertices,
        //     normal_indices,
        //     normals,
        //     tex_coords_indices,
        //     tex_coords,
        // );

        // log(&format!("Normals: {:?}", normals));
        // log(&format!("Vertices: {:?}", vertices));
        // log(&format!("tex_coords: {:?}", tex_coords));

        let vertices = js_sys::Float32Array::from(vertices);
        let vertex_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vertices, GL::STATIC_DRAW);

        let normals = js_sys::Float32Array::from(normals);
        let normal_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&normal_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &normals, GL::STATIC_DRAW);

        let tex_coords = js_sys::Float32Array::from(tex_coords);
        let tex_coords_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&tex_coords_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &tex_coords, GL::STATIC_DRAW);

        let num_indices = indices.len() as i32;
        let indices = js_sys::Uint16Array::from(indices);

        let index_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &indices, GL::STATIC_DRAW);

        // Texture
        let texture = Texture::new(gl, texture).unwrap();

        SimpleMesh {
            shader,
            vertex_buffer,
            normal_buffer,
            tex_coords_buffer,
            index_buffer,
            num_indices,
            texture,
        }
    }

    pub fn cube(gl: &GL) -> SimpleMesh {
        // Vertices
        let x: f32 = 1.;
        let y: f32 = 1.;
        let z: f32 = 1.;
        let vertices: Vec<f32> = vec![
            x, y, z, x, y, -z, x, -y, z, x, -y, -z, -x, y, z, -x, y, -z, -x, -y, z, -x, -y, -z,
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

        let tex_coords: Vec<f32> = vec![0., 0., 0., 1., 1., 0., 1., 1.];
        let tex_coords_indices: Vec<u16> = vec![
            0, 1, 3, 3, 2, 0, //
            0, 1, 3, 3, 2, 0, //
            0, 1, 3, 3, 2, 0, //
            0, 1, 3, 3, 2, 0, //
            2, 3, 1, 1, 0, 2, //
            0, 1, 3, 3, 2, 0, //
        ];

        // Synchronize
        let (indices, vertices, normals, tex_coords) = gl_utils::synchronize_buffers(
            &vertex_indices,
            &vertices,
            &normal_indices,
            &normals,
            &tex_coords_indices,
            &tex_coords,
        );

        SimpleMesh::create_resources(
            gl,
            "./cube-texture.png",
            &indices,
            &vertices,
            &normals,
            &tex_coords,
        )
    }

    pub fn mesh(gl: &GL, mesh_name: &str, texture: &str) -> SimpleMesh {
        let meshes = include_bytes!("../../dist/meshes.bin");
        let mut meshes: HashMap<String, BlenderMesh> = bincode::deserialize(meshes).unwrap();

        // let mut attributes_sets = Vec::new();
        for (name, _) in meshes.iter() {
            gl_utils::log(name);
        }
        let mut mesh = meshes.get_mut(mesh_name).unwrap();

        let attributes = mesh.combine_vertex_indices(&CreateSingleIndexConfig {
            bone_influences_per_vertex: None,
            calculate_face_tangents: false,
        });
        mesh.y_up();

        let mut indices: Vec<u16> = vec![];
        let mut vertices: Vec<f32> = vec![];
        let mut normals: Vec<f32> = vec![];
        let mut tex_coords: Vec<f32> = vec![];

        indices.extend_from_slice(attributes.indices());
        vertices.extend_from_slice(attributes.positions());
        normals.extend_from_slice(attributes.normals().expect("Mesh has no normals"));
        tex_coords.extend_from_slice(attributes.uvs().expect("Mesh has no tex coordinates"));

        SimpleMesh::create_resources(gl, texture, &indices, &vertices, &normals, &tex_coords)
    }

    pub fn render(&self, gl: &GL, camera: &Camera, transformation: &Transformation) {
        // Transformation
        let model = Similarity3::new(transformation.translation, transformation.rotation, 0.5f32);

        gl.use_program(Some(&self.shader));
        // Bind uniform values
        let projection = gl.get_uniform_location(&self.shader, "projection");
        let view = gl.get_uniform_location(&self.shader, "view");
        let model_loc = gl.get_uniform_location(&self.shader, "model");
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
        gl.uniform_matrix4fv_with_f32_array(
            model_loc.as_ref(),
            false,
            &model.to_homogeneous().as_slice(),
        );
        gl.uniform_matrix4fv_with_f32_array(projection.as_ref(), false, &camera.projection());

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

        gl.active_texture(GL::TEXTURE0);
        let texture_loc = gl.get_uniform_location(&self.shader, "texture").unwrap();
        gl.active_texture(GL::TEXTURE0);
        gl.uniform1i(Some(&texture_loc), 0);
        gl.bind_texture(GL::TEXTURE_2D, Some(self.texture.texture.as_ref()));

        // Indices
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.index_buffer));
        gl.draw_elements_with_i32(GL::TRIANGLES, self.num_indices, GL::UNSIGNED_SHORT, 0);
    }
}
