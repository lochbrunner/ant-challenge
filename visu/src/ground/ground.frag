precision mediump float;

uniform vec3 ambient;
uniform vec3 point_light;
uniform vec3 point_light_dir;
uniform vec2 tex_scale;
uniform sampler2D grass_texture;
uniform sampler2D mud_texture;
uniform sampler2D stone_texture;
uniform sampler2D map_texture;

varying vec3 vNormal;
varying vec2 tex_uv;

void main() {
  vec3 normal = normalize(vNormal);
  vec3 point_light_dir_normalized = normalize(point_light_dir);
  float diff = max(dot(normal, -point_light_dir_normalized), 0.0);
  vec3 diffuse = diff * point_light;

  vec4 map = texture2D(map_texture, tex_uv);
  float grass = 1. - map.x - map.y;
  float mud = map.x;
  float stone = map.y;

  vec4 textureColor = grass * texture2D(grass_texture, tex_uv * tex_scale) +
                      mud * texture2D(mud_texture, tex_uv * tex_scale) +
                      stone * texture2D(stone_texture, tex_uv * tex_scale);

  vec4 lighting = vec4(ambient + diffuse, 1.0);
  gl_FragColor = textureColor * lighting;
}