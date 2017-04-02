#version 150

in vec2 position;

uniform mat4 perspective;
uniform mat4 modelView;

void main() {
    gl_Position = perspective * modelView * vec4(position, 0.0, 1.0);
}
