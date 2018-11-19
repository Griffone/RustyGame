#version 330 core

in vec2 v_tex_coords;

uniform vec4 u_color;
uniform sampler2D u_texture;

out vec4 out_color;

void main() {
  out_color = u_color * texture(u_texture, v_tex_coords);
  if (out_color.a == 0) discard;
}