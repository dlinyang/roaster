#version 460

in vec3 position;
in vec3 normal;
in vec4 tex_coordinate;

out vec3 f_normal;
out vec3 f_pos;

uniform mat4 model;
uniform mat4 transform;
uniform mat4 project;

void main() {
    f_normal = mat3(transpose(inverse(model * transform))) * normal;
    vec4 pos = model *  transform * vec4(position,1.0);
    f_pos = vec3(pos);
    gl_Position = project * pos;
}