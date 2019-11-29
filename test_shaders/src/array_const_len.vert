#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable
#extension GL_EXT_multiview : enable




layout( location = 0 ) in vec3 position;
layout( location = 1 ) in vec3 color;

layout (constant_id = 0) const uint NUM_VIEW = 1;


layout (set = 0, binding = 0) uniform UBO{
    mat4 viewproj[NUM_VIEW];
} ubo;



layout ( push_constant ) uniform Model {
    mat4 model;
};



layout(location = 0) out VertexData {
    vec3 color;
} vertex;




void main(){
    //ubo.viewproj[gl_ViewIndex]  * model *
    gl_Position = ubo.viewproj[gl_ViewIndex]  * model * vec4(position, 1.0);

    vertex.color = color;

}
