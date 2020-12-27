#version 450
layout (points) in;
layout (points, max_vertices = 1) out;


layout(location = 0) in VertexData {
    vec3 half_extent;
} vertex[];

layout(location = 0) out VertexOutput{
    vec3 color;
} vs_out;

void main(){
    gl_Position = gl_in[0].gl_Position;
    vs_out.color = vec3(1.0, 1.0, 1.0);
    EmitVertex();
    EndPrimitive();
}
