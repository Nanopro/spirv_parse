#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

layout( location = 0 ) in vec3 position;
layout( location = 1 ) in vec3 normal;
layout( location = 2 ) in vec3 color;
layout( location = 3 ) in mat4 model_instanced;




layout (set = 0, binding = 0) uniform UBO{
    mat4 viewproj;
} ubo;



layout ( push_constant ) uniform Model {
    mat4 model;
};


layout(location = 0) out VertexData {
    vec3 position;
    vec3 normal;
    vec3 color;
} vertex;



void main(){

    gl_Position = ubo.viewproj * model * model_instanced * vec4(position, 1.0);


    vertex.position = position;
    vertex.normal = normal;
    vertex.color = color;
}
