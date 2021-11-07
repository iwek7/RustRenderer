#version 330 core
out vec4 fragColor;

in vec4 ourColor;
in vec2 TexCoord;

uniform sampler2D tx;
uniform vec2      resolution;// viewport resolution (in pixels)
uniform float     timeMillis;// shader playback time (in seconds)
uniform vec4      color;

void main()
{
    vec4 sampled = texture(tx, TexCoord) * color;
    fragColor = sampled;
}