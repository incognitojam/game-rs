#version 330 core

uniform vec3 CameraPos;
uniform sampler2D TexFace;

in VS_OUTPUT {
    vec3 Position;
    vec3 Normal;
    vec2 Uv;
} IN;

out vec4 Color;

void main()
{
    vec3 color = texture(TexFace, IN.Uv).rgb;

    // normal
    vec3 normal = IN.Normal;

    // diffuse
    vec3 lightDir = normalize(CameraPos - IN.Position);
    float diff = max(dot(lightDir, normal), 0.2);
    vec3 diffuse = diff * color;

    Color = vec4(diffuse, 1.0);
}
