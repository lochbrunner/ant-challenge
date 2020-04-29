precision mediump float;

attribute vec3 position;
attribute vec3 normal;

uniform mat4 view;
uniform mat4 projection;
uniform vec3 camera_pos;

varying vec4 worldPosition;
varying vec3 vNormal;

varying vec3 relateive_camera_pos;

void main() {
  worldPosition = vec4(position, 1.0);
  gl_Position = projection * view * worldPosition;
  relateive_camera_pos = camera_pos - worldPosition.xyz;
  vNormal = normal;
}