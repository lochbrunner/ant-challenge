// use crate::camera::Camera;
// use crate::shader::Shader;
// use bincode;
// use blender_mesh::{BlenderMesh, CreateSingleIndexConfig};
// use nalgebra::{Isometry3, Vector3};
// use std::collections::HashMap;
// use web_sys::WebGlRenderingContext as GL;

pub mod simple_mesh;

// pub struct Vao(js_sys::Object);

// pub struct MeshRenderOpts {
//     pub pos: (f32, f32, f32),
//     pub clip_plane: [f32; 4],
// }

// impl MeshRenderOpts {
//     pub fn matrix(&self) -> Isometry3<f32> {
//         Isometry3::new(
//             Vector3::new(self.pos.0, self.pos.1, self.pos.2),
//             nalgebra::zero(),
//         )
//     }
// }

// pub struct NonSkinnedMesh {
//     pub mesh: BlenderMesh,
//     pub shader: Shader,
//     // pub vao: Vao,
//     // pub opts: &'a MeshRenderOpts,
// }

// static MESH_NON_SKINNED_VS: &'static str = include_str!("../shader/mesh-non-skinned-vertex.glsl");
// static MESH_NON_SKINNED_FS: &'static str = include_str!("../shader/mesh-non-skinned-fragment.glsl");

// impl NonSkinnedMesh {
//     pub fn new(name: &str, gl: &GL) -> NonSkinnedMesh {
//         // Later: One file one mesh object
//         let meshes = include_bytes!("../../dist/meshes.bin");
//         let mut meshes: HashMap<String, BlenderMesh> = bincode::deserialize(meshes).unwrap();
//         let (_, mut mesh) = meshes
//             .remove_entry(name)
//             .expect(&format!("Loading mesh {}", name));

//         mesh.combine_vertex_indices(&CreateSingleIndexConfig {
//             bone_influences_per_vertex: None,
//             calculate_face_tangents: false,
//         });
//         mesh.y_up();

//         let shader = Shader::new(&gl, MESH_NON_SKINNED_VS, MESH_NON_SKINNED_FS).unwrap();

//         NonSkinnedMesh { mesh, shader }
//     }

//     fn buffer_attributes(&self, gl: &GL) {
//         let shader = &self.shader;
//         let mesh = &self.mesh;

//         let pos_attrib = gl.get_attrib_location(&shader.program, "position");
//         let normal_attrib = gl.get_attrib_location(&shader.program, "normal");
//         let uv_attrib = gl.get_attrib_location(&shader.program, "uvs");

//         gl.enable_vertex_attrib_array(pos_attrib as u32);
//         gl.enable_vertex_attrib_array(normal_attrib as u32);
//         gl.enable_vertex_attrib_array(uv_attrib as u32);
//     }

//     pub fn render(&self, gl: &GL, opts: MeshRenderOpts, camera: &Camera) {
//         let shader = &self.shader;
//         let model_uni = shader.get_uniform_location(gl, "model");
//         let view_uni = shader.get_uniform_location(gl, "view");
//         let camera_pos_uni = shader.get_uniform_location(gl, "cameraPos");
//         let perspective_uni = shader.get_uniform_location(gl, "perspective");
//         let clip_plane_uni = shader.get_uniform_location(gl, "clipPlane");
//         // let mesh_texture_uni = shader.get_uniform_location(gl, "meshTexture");

//         gl.uniform4fv_with_f32_array(clip_plane_uni.as_ref(), &opts.clip_plane.clone()[..]);
//         gl.uniform_matrix4fv_with_f32_array(view_uni.as_ref(), false, &camera.view());

//         let model = opts.matrix();
//         gl.uniform_matrix4fv_with_f32_array(
//             model_uni.as_ref(),
//             false,
//             model.to_homogeneous().as_slice(),
//         );

//         let camera_pos = camera.get_eye_pos();
//         let camera_pos = [camera_pos.x, camera_pos.y, camera_pos.z];
//         gl.uniform3fv_with_f32_array(camera_pos_uni.as_ref(), &camera_pos);

//         gl.uniform_matrix4fv_with_f32_array(perspective_uni.as_ref(), false, &camera.projection());

//         // gl.uniform1i(mesh_texture_uni.as_ref(), TextureUnit::Stone.texture_unit());

//         // let num_indices = self.mesh.vertex_position_indices.len();
//         gl.draw_elements_with_i32(
//             GL::TRIANGLES,
//             self.mesh.num_indices(),
//             GL::UNSIGNED_SHORT,
//             0,
//         );
//     }
// }
