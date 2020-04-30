precision mediump float;

attribute vec3 position;
attribute vec3 normal;
attribute vec2 tex_coordinate;

uniform mat4 view;
uniform mat4 model;
uniform mat4 projection;
uniform vec3 camera_pos;

varying vec4 worldPosition;
varying vec3 vNormal;

varying vec3 relative_camera_pos;
varying vec2 tex_uv;

void main() {
  worldPosition = model * vec4(position, 1.0);
  gl_Position = projection * view * worldPosition;
  relative_camera_pos = camera_pos - worldPosition.xyz;
  vNormal = normal;
  tex_uv = tex_coordinate;
}