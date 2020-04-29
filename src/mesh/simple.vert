precision mediump float;

attribute vec3 position;
attribute vec3 normal;

uniform mat4 view;
uniform mat4 projection;

varying vec4 worldPosition;
varying vec3 vNormal;

void main() {
  worldPosition = vec4(position, 1.0);
  gl_Position = projection * view * worldPosition;
  vNormal = normal;
}