#version 330 core

// Per vertex data
in vec2 position;
in vec2 tex_coords;

// Per instance data
in vec2 i_translation;
in vec2 i_z_theta;
in vec2 i_scale;

in vec4 i_color_lit;
in vec4 i_color_unlit;
in vec4 i_texture_lit;
in vec4 i_texture_unlit;

// Uniform data
uniform vec2 u_scale;       // camera screen-space transformations
uniform vec2 u_translation;
uniform vec2 u_view_origin;

out vec2 v_coords_lit;
out vec2 v_coords_unlit;
out vec4 v_color_lit;
out vec4 v_color_unlit;
out float v_view_distance;

void main() {
    float sinTheta = sin(i_z_theta[1]);
    float cosTheta = cos(i_z_theta[1]);

    mat2 rotation;
    rotation[0] = vec2(cosTheta, sinTheta);
    rotation[1] = vec2(-sinTheta, cosTheta);

    vec2 pos = rotation * position;
    pos *= i_scale;

    pos += i_translation;
    v_view_distance = distance(pos, u_view_origin);

    pos -= u_translation;
    pos *= u_scale;

    v_coords_lit = i_texture_lit.xy + (i_texture_lit.zw - i_texture_lit.xy) * tex_coords;
    v_coords_unlit = i_texture_unlit.xy + (i_texture_unlit.zw - i_texture_unlit.xy) * tex_coords;
    v_color_lit = i_color_lit;
    v_color_unlit = i_color_unlit;
    gl_Position = vec4(pos, i_z_theta[0], 1);
}