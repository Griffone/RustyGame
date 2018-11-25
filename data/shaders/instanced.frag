#version 330 core

in vec2 v_tex_coords;
in vec4 v_color;
in vec2 v_pos;

uniform vec2 u_eye;
uniform float u_see_dist;

uniform sampler2D u_texture;
uniform sampler2D u_dark;

out vec4 out_color;

void main() {
  float dist = distance(u_eye, v_pos);
  float ratio = clamp(u_see_dist - dist, 0, 1);
  out_color = v_color * mix(texture(u_dark, v_tex_coords), texture(u_texture, v_tex_coords), ratio);
  if (out_color.a == 0) discard;
}