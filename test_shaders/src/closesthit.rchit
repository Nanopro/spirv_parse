#version 460
#extension GL_NV_ray_tracing : require
#extension GL_EXT_nonuniform_qualifier : enable
#extension VK_EXT_buffer_indexing : enable

layout(location = 0) rayPayloadInNV vec3 hitValue;

layout(set = 1, binding = 0) readonly buffer MeshId { uint mesh_id[]; };
layout(set = 1, binding = 1) readonly buffer VertexPositionBuffer { vec3 pos[]; } vertex_buffer[];
layout(set = 1, binding = 2) readonly buffer IndexBuffer { uint index[]; } index_buffer[];
layout(set = 1, binding = 3) readonly buffer VertexNormalBuffer { vec3 normal[]; } normal_buffer[];
layout(set = 1, binding = 4) readonly buffer VertexValueBuffer { float value[]; } value_buffer[];

hitAttributeNV vec3 attribs;

const vec3 LIGHT = -vec3(1.0, 1.0, 1.0);


vec3 Rainbow(in float value){
  if (0.0 <= value && value < 0.25) {
    return vec3(0.0, (1.0 * (value) / 0.25), 1.0);
  }
  else if (value < 0.5) {
    return vec3(0.0, 1.0, (1.0 * (0.5 - value) / 0.25));
  }
  else if (value < 0.75) {
    return vec3((1.0 * (value - 0.5) / 0.25), 1.0, 0.0);
  }
  else {
    return vec3(1.0, (1.0 * (1.0 - value) / 0.25), 0.0);
  }
}


void main()
{
  uint mesh = mesh_id[gl_InstanceCustomIndexNV];
  uint x = index_buffer[mesh].index[3 * gl_PrimitiveID + 0];
  uint y = index_buffer[mesh].index[3 * gl_PrimitiveID + 1];
  uint z = index_buffer[mesh].index[3 * gl_PrimitiveID + 2];


  float a = value_buffer[mesh].value[x];
  float b = value_buffer[mesh].value[y];
  float c = value_buffer[mesh].value[z];
  float value = (1.0f - attribs.x - attribs.y) * a + attribs.x * b + attribs.y * c;
  vec3 color = Rainbow(value);

  vec3 an = normal_buffer[mesh].normal[x];
  vec3 bn = normal_buffer[mesh].normal[y];
  vec3 cn = normal_buffer[mesh].normal[z];
  vec3 normal = (1.0f - attribs.x - attribs.y) * an + attribs.x * bn + attribs.y * cn;
  //vec3 normal = an;

  //float bright = max(max(dot(LIGHT, normal), dot(-LIGHT, normal)), 0.4);


  float bright = (max(dot(normalize(LIGHT), normalize(normal)), dot(-normalize(LIGHT), normalize(normal)))+0.5)/1.5;
  hitValue = color * bright;
}
