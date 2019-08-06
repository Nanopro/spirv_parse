#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable
#extension GL_EXT_multiview : enable


layout( location = 0 ) in vec3 position;
layout( location = 1 ) in vec3 normal;
layout( location = 2 ) in vec3 color;

layout (constant_id = 0) const int NUM_VIEW = 1;

layout (set = 0, binding = 0) uniform UBO{
    mat4 viewproj[NUM_VIEW];
} ubo;

layout ( push_constant ) uniform Model {
    mat4 model;
};


layout( location = 0 ) out vec3 out_position;
layout( location = 1 ) out vec3 out_normal;
layout( location = 2 ) out vec3 out_color;


void main(){
    gl_Position = ubo.viewproj[NUM_VIEW] * model * vec4(position, 1.0);
    out_position = position;
    out_normal = normal;
    out_color = color;
}
