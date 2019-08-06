#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

layout (set = 0, binding = 0) uniform UBO{
    mat4 viewproj;
} ubo;

layout (set = 0, binding = 2) uniform UBO2{
    mat4 viewproj;
} ubo2;
layout (set = 1, binding = 2) uniform UBO3{
    mat4 viewproj;
} ubo3;

layout(location = 0) in VertexData {
    vec3 position;
    vec3 normal;
    vec3 color;
} vertex;

layout(location = 0) out vec4 out_color;

const vec3 LIGHT = -vec3(1.0, 1.0, 1.0);

const vec3 La = vec3(0.4, 0.4, 0.4);
const vec3 Ld = vec3(1.0, 1.0, 1.0);
const vec3 Ls = vec3(1.0, 1.0, 1.0);


void main(){
    float bright = (max(dot(normalize(LIGHT), normalize(vertex.normal)), dot(-normalize(LIGHT), normalize(vertex.normal)))+0.5)/1.5;
    out_color = ubo.viewproj *  ubo3.viewproj * ubo2.viewproj * vec4(vertex.color * bright, 1.0);
}