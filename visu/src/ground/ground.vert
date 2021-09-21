precision mediump float;

attribute vec3 position;
attribute vec3 normal;
attribute vec2 tex_coordinate;

uniform mat4 view;
uniform mat4 projection;

varying vec3 vNormal;
varying vec2 tex_uv;

void main() {
  vec4 worldPosition = vec4(position, 1.0);
  gl_Position = projection * view * worldPosition;
  vNormal = normal;
  tex_uv = tex_coordinate;
}