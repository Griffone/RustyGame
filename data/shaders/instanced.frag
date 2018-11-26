#version 330 core

in vec2 v_coords_lit;
in vec2 v_coords_unlit;
in vec4 v_color_lit;
in vec4 v_color_unlit;
in float v_view_distance;

uniform float u_view_distance;

uniform sampler2D u_texture;

out vec4 out_color;

void main() {
  float ratio = clamp(u_view_distance - v_view_distance, 0, 1);
  vec4 lit_color = v_color_lit * texture(u_texture, v_coords_lit);
  vec4 unlit_color = v_color_unlit * texture(u_texture, v_coords_unlit);
  out_color = mix(unlit_color, lit_color, ratio);
  if (out_color.a == 0) discard;
}