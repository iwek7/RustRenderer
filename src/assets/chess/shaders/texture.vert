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
    // todo this needs to be done in engine code
    // for instance, now there is discrepency in camera position and pieces position
    // camera is not rendered so that this transform is not applied!
    gl_Position = mvp * vec4(aPos.x, aPos.y, 0.0, 1.0f);
    ourColor = aColor;
    TexCoord = aTexCoord;
}