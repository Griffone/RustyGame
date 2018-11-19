#version 330 core

layout(location = 0) in vec2 position;
layout(location = 1) in vec2 tex_coords;

uniform vec2 u_translate; // Includes both world-space and view-space translations
uniform vec2 u_z_theta; // Screen-space z coordinate and theta angle for rotation
uniform vec2 u_scale;     // X and Y scale

out vec2 v_tex_coords;

void main() {
    float sinTheta = sin(u_z_theta[1]);
    float cosTheta = cos(u_z_theta[1]);

    mat2 rotation;
    rotation[0] = vec2(cosTheta, sinTheta);
    rotation[1] = vec2(-sinTheta, cosTheta);

    vec2 pos = rotation * position;
    pos *= u_scale;
    pos += u_translate;

    v_tex_coords = tex_coords;
    gl_Position = vec4(pos, u_z_theta[0], 1);
}