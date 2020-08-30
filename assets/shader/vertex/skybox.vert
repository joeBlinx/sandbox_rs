#version 330

layout(location = 0) in vec3 pos;

out vec3 tex_coords;

uniform mat4 proj;
uniform mat4 view;
void main() {

    gl_Position =  (proj*mat4(mat3(view))*vec4(pos, 1.)).xyww;
    tex_coords = pos;
}
