#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec4 Color;

out VS_OUTPUT {
    vec4 Color;
} OUT;

uniform mat4 mvp;
uniform vec2 win_size;

void main()
{
    gl_Position = mvp * vec4(2.0 * aPos.x / win_size.x - 1.0, 2.0 * aPos.y / win_size.y - 1.0, aPos.z, 1.0f);
    OUT.Color = Color;
}