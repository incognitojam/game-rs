#version 330 core

uniform sampler2D TexFace;

in VS_OUTPUT {
    vec3 Position;
    vec2 Uv;
} IN;

out vec4 Color;

void main()
{
    Color = texture(TexFace, IN.Uv);
}
