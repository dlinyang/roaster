#version 460

in vec3 f_normal;
in vec3 f_pos;

out vec4 color;

uniform vec3 light_color;
uniform vec3 light_pos;

void main() {
    vec3 diffuss = max(dot(normalize(f_normal),normalize(light_pos - f_pos)),0.0) * light_color;
    float ambientStrength = 0.1;
    vec3 ambient = ambientStrength * light_color;
    color = vec4((ambient + diffuss)* vec3(1.0,0.0,1.0),1.0);
}