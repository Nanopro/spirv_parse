#version 460
#extension GL_EXT_ray_tracing : require
#extension GL_EXT_nonuniform_qualifier : enable
#extension GL_EXT_debug_printf : enable

struct Vertex{
  vec3 pos;
  vec3 norm;
  vec2 uv;
};

const uint STRIDE = 8;

layout(location = 0) rayPayloadInEXT vec3 hitValue;


layout(set = 1, binding = 0) readonly buffer HasVertexBuffer { uint has[]; } has_vertex_buffer;
layout(set = 1, binding = 1) readonly buffer VertexBuffer { float data[]; } vertex_buffer[];

layout(set = 2, binding = 0) readonly buffer HasIndexBuffer { uint has[]; } has_index_buffer;
layout(set = 2, binding = 1) readonly buffer IndexBuffer { uint index[]; } index_buffer[];

hitAttributeEXT vec3 attribs;

const vec3 LIGHT = -vec3(1.0, 1.0, 1.0);


Vertex unpack(uint index){
  Vertex v;
  v.pos = vec3(
  vertex_buffer[gl_InstanceCustomIndexEXT].data[ STRIDE * index + 0 ],
  vertex_buffer[gl_InstanceCustomIndexEXT].data[ STRIDE * index + 1 ],
  vertex_buffer[gl_InstanceCustomIndexEXT].data[ STRIDE * index + 2 ]
  );
  v.norm = vec3(
  vertex_buffer[gl_InstanceCustomIndexEXT].data[ STRIDE * index + 3 ],
  vertex_buffer[gl_InstanceCustomIndexEXT].data[ STRIDE * index + 4 ],
  vertex_buffer[gl_InstanceCustomIndexEXT].data[ STRIDE * index + 5 ]
  );
  v.uv = vec2(
  vertex_buffer[gl_InstanceCustomIndexEXT].data[ STRIDE * index + 6 ],
  vertex_buffer[gl_InstanceCustomIndexEXT].data[ STRIDE * index + 7 ]
  );



  return v;
}



void main()
{

  uint ai = index_buffer[gl_InstanceCustomIndexEXT].index[3 * gl_PrimitiveID + 0];
  uint bi = index_buffer[gl_InstanceCustomIndexEXT].index[3 * gl_PrimitiveID + 1];
  uint ci = index_buffer[gl_InstanceCustomIndexEXT].index[3 * gl_PrimitiveID + 2];


  Vertex a = unpack(ai);
  Vertex b = unpack(bi);
  Vertex c = unpack(ci);

  vec3 normal = (1.0f - attribs.x - attribs.y) * a.norm + attribs.x * b.norm + attribs.y * c.norm;
  vec3 color = vec3( (1.0f - attribs.x - attribs.y) * a.uv + attribs.x * b.uv + attribs.y * c.uv, 1.0 );
  float bright = (max(dot(normalize(LIGHT), normalize(normal)), dot(-normalize(LIGHT), normalize(normal)))+0.5)/1.5;
  hitValue = color * bright;

}
