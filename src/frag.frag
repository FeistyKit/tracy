#version 300 es

precision mediump float;
layout(origin_upper_left) in vec4 gl_FragCoord;

uniform vec2 Centre;
uniform vec3 Colour;
uniform float Radius;

#define RADIUS_MOD 0.4

void main(void)
{
    float x = Centre.x - gl_FragCoord.x;
    float y = Centre.y - gl_FragCoord.y;
    float alpha = 1.0 - sqrt(x * x + y * y) / (Radius * RADIUS_MOD);

    gl_FragColor = vec4(Colour, alpha);
}
