precision mediump float;

attribute vec3 position;

uniform mat4 view;
uniform mat4 projection;
varying vec4 worldPosition;

void main() {
  worldPosition = vec4(position, 1.0);
  gl_Position = projection * view * worldPosition;
}