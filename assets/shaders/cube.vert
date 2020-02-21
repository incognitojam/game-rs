#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec2 Uv;
layout (location = 2) in float LightLevel;

uniform mat4 View;
uniform mat4 Projection;

out VS_OUTPUT {
    vec3 Position;
    vec2 Uv;
    float LightBrightness;
} OUT;

void main()
{
    gl_Position = Projection * View * vec4(Position, 1.0);

    OUT.Position = Position;
    OUT.Uv = Uv;
    OUT.LightBrightness = (1.0f / 16.0f) + (LightLevel / 16.0f);
}
