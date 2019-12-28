#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec2 Uv;

uniform mat4 View;
uniform mat4 Projection;

out VS_OUTPUT {
    vec3 Position;
    vec2 Uv;
} OUT;

void main()
{
    gl_Position = Projection * View * vec4(Position, 1.0);

    OUT.Position = Position;
    OUT.Uv = Uv;
}
