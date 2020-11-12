#version 330 core

layout (location = 0) in vec3 Position;
layout(location = 1) in vec3 uv;
layout(location = 2) in vec3 normals;
uniform mat4 proj;
uniform mat4 view;
out vec3 normal;
out vec3 frag_pos;
out vec3 uv_out;
void main()
{
    gl_Position = proj*view*vec4(Position, 1.0);
    frag_pos = Position;
    normal = normals;
    uv_out = uv;
}