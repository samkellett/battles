#version 150

in vec2 v_tex_coords;

out vec4 color;
uniform vec3 u_light;
uniform sampler2D diffuse_tex;

void main() {
    color = vec4(texture(diffuse_tex, v_tex_coords).rgb, 0.0);
}
