#version 330 core
out vec4 fragColor;

in vec4 ourColor;
in vec2 TexCoord;

uniform sampler2D tx;
uniform vec2      resolution;// viewport resolution (in pixels)
uniform float     timeMillis;// shader playback time (in seconds)

uniform float fadeOffAlpha = 1f;

float random(vec2 co){
    return fract(sin(dot(co, vec2(12.9898, 78.233))) * 43758.5453);
}

void main()
{
    vec4 sampled = texture(tx, TexCoord);
    float final_alpha =  sampled.a * fadeOffAlpha;
    // check if fade off is happening
//    if (fadeOffAlpha < 1.0) {
//        vec2 uv = (2.0 * gl_FragCoord.xy - resolution.xy) / resolution.y;
//        float rand = random(uv);
//        if(rand < fadeOffAlpha) {
//            final_alpha *= 1.0;
//        } else {
//            final_alpha *= 0.0;
//        }
//    }
    fragColor = vec4(sampled.rgb, final_alpha);
}

