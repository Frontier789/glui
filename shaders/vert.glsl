#version 420 core

layout(location = 0) in vec3 position;
layout(location = 1) in vec2 tpt;
layout(location = 2) in vec3 nrm;
layout(location = 3) in vec3 tng;

uniform mat4 model;
uniform mat4 view;
uniform mat4 proj;

uniform sampler2D height;
uniform float seconds;

out vec2 texp;
out vec3 norm;
out vec3 tangent;
out vec3 bitangent;

void main()
{
    gl_Position = proj * view * model * vec4(position, 1);
    
    texp = tpt;
    
    norm      = mat3(model) * nrm;
    tangent   = mat3(model) * tng;
    bitangent = mat3(model) * cross(nrm,tng);
}