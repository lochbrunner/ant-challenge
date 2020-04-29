precision mediump float;

varying vec3 vNormal;

vec3 ambient = vec3(0.24725, 0.1995, 0.0745);
vec3 sunlightColor = vec3(1.0, 1.0, 1.0);
vec3 sunlightDir = normalize(vec3(-1.0, -1.0, 0.5));

void main() {
  vec3 normal = normalize(vNormal);
  float diff = max(dot(normal, -sunlightDir), 0.0);
  vec3 diffuse = diff * sunlightColor;

  vec4 textureColor = vec4(0.5, 0.5, 0.5, 1.0);
  vec4 lighting = vec4(ambient + diffuse, 1.0);

  gl_FragColor = textureColor * lighting;
}