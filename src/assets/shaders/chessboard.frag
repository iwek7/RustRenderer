#version 330 core
out vec4 FragColor;

in vec4 ourColor;
in vec2 TexCoord;

uniform sampler2D ourTexture;
uniform vec2 u_resolution;

void main()
{
    vec2 st = gl_FragCoord.xy/u_resolution.xy;
    st.x *= u_resolution.x/u_resolution.y;
    vec3 color = vec3(0.);
    vec2 bl = step(vec2(0.1),st);       // bottom-left
    vec2 tr = step(vec2(0.1),1.0-st);   // top-right
    color = vec3(bl.x * bl.y * tr.x * tr.y);
   FragColor = mix(texture(ourTexture, TexCoord), vec4(color, 1.0f), 0.5f);
   //FragColor = vec4(color, 1.0f);
}