#version 330 core

// Per vertex data
in vec2 position;
in vec2 tex_coords;

// Per instance data
in vec2 i_translation;
in vec2 i_z_theta;
in vec2 i_scale;
in vec4 i_color;

// Uniform data
uniform vec2 u_scale;       // camera screen-space transformations
uniform vec2 u_translation;

//uniform vec2 u_eye;

out vec2 v_tex_coords;
out vec4 v_color;
out vec2 v_pos;

void main() {
    float sinTheta = sin(i_z_theta[1]);
    float cosTheta = cos(i_z_theta[1]);

    mat2 rotation;
    rotation[0] = vec2(cosTheta, sinTheta);
    rotation[1] = vec2(-sinTheta, cosTheta);

    vec2 pos = rotation * position;
    pos *= i_scale;

    pos += i_translation;
    v_pos = pos;

    pos -= u_translation;
    pos *= u_scale;

    v_tex_coords = tex_coords;
    v_color = i_color;
    gl_Position = vec4(pos, i_z_theta[0], 1);
}