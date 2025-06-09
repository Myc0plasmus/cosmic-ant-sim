#version 330

uniform mat4 P;
uniform mat4 V;
uniform mat4 M;

layout(location = 0) in vec4 vertex;
layout(location = 1) in vec4 normal;

out vec3 FragPos;
out vec3 Normal;

void main(void) {
    mat4 MV = V * M;
    FragPos = vec3(MV * vertex); // position in camera space
    Normal = mat3(transpose(inverse(MV))) * normal.xyz; // transformed normal
    gl_Position = P * MV * vertex;
}
