#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec4 Color;

out VS_OUTPUT {
    vec4 Color;
} OUT;

uniform mat4 pvm;

void main()
{
    gl_Position = pvm * vec4(Position, 1.0f);
    OUT.Color = Color;
}