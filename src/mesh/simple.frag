precision mediump float;

uniform vec3 ambient;
uniform vec3 point_light;
uniform vec3 point_light_dir;
uniform sampler2D texture;

vec3 reflection_base = vec3(0.628281, 0.555802, 0.366065) * 0.4;

varying vec3 vNormal;
varying vec2 tex_uv;
varying vec3 relative_camera_pos;

void main() {
  vec3 normal = normalize(vNormal);
  vec3 point_light_dir_normalized = normalize(point_light_dir);
  float diff = max(dot(normal, -point_light_dir_normalized), 0.0);
  vec3 diffuse = diff * point_light;

  vec3 reflect_dir = reflect(-point_light_dir, normal);
  float specular =
      pow(max(dot(normalize(relative_camera_pos), reflect_dir), 0.0), 32.0);
  vec3 reflection = specular * reflection_base;

  vec4 textureColor = texture2D(texture, tex_uv);

  vec4 lighting = vec4(ambient + diffuse + reflection, 1.0);

  gl_FragColor = textureColor * lighting;
}