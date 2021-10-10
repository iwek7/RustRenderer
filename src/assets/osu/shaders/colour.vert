#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec4 Color;

out VS_OUTPUT {
    vec4 Color;
} OUT;

uniform mat4 mvp;

void main()
{
    gl_Position = mvp * vec4(aPos.x, aPos.y, aPos.z, 1.0f);
    OUT.Color = Color;
}