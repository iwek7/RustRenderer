#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec4 aColor;
layout (location = 2) in vec2 aTexCoord;

out vec4 ourColor;
out vec2 TexCoord;

uniform mat4 mvp;
uniform vec2 win_size;

void main()
{
    gl_Position = mvp * vec4(2.0 * aPos.x / win_size.x - 1.0, 2.0 * aPos.y / win_size.y - 1.0, 0.0, 1.0f);
    ourColor = aColor;
    TexCoord = aTexCoord;
}