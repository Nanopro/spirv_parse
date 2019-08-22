#version 450
layout (binding = 5) uniform sampler2D uSampler[10];

layout (binding = 6) buffer shader_data2
{
	vec4 camera_position;
	vec4 light_position;
	vec4 light_diffuse;
} ttt[15];
layout (binding = 7) uniform samplerBuffer texxxxx;
layout (binding = 9) uniform sampler tttt;
layout (binding = 8, rg32f) uniform writeonly imageBuffer imgggg;

layout (binding = 1) uniform sampler2D samplerColor;
layout (binding = 2) uniform texture2D textureColor;
layout (binding = 3, r32f) uniform writeonly image2D imageColor;

layout (input_attachment_index = 0, set = 3, binding = 0) uniform subpassInput inputColor;
layout (input_attachment_index = 1, set = 4, binding = 1) uniform subpassInput inputDepth;

layout (binding = 0) uniform UBO
{
	float blurScale;
	float blurStrength;
} ubo;

layout (std430, binding=2) buffer shader_data
{
	vec4 camera_position;
	vec4 light_position;
	vec4 light_diffuse;
} tttttt;


layout (constant_id = 0) const int blurdirection = 0;

layout (location = 0) in vec2 inUV;

layout (location = 0) out vec4 outFragColor;

void main()
{
	float weight[5];
	weight[0] = 0.227027;
	weight[1] = 0.1945946;
	weight[2] = 0.1216216;
	weight[3] = 0.054054;
	weight[4] = 0.016216;

	vec2 tex_offset = 1.0 / textureSize(samplerColor, 0) * ubo.blurScale; // gets size of single texel
	vec3 result = texture(samplerColor, inUV).rgb * weight[0]; // current fragment's contribution
	for(int i = 1; i < 5; ++i)
	{
		if (blurdirection == 1)
		{
			// H
			result += texture(samplerColor, inUV + vec2(tex_offset.x * i, 0.0)).rgb * weight[i] * ubo.blurStrength;
			result += texture(samplerColor, inUV - vec2(tex_offset.x * i, 0.0)).rgb * weight[i] * ubo.blurStrength;
		}
		else
		{
			// V
			result += texture(samplerColor, inUV + vec2(0.0, tex_offset.y * i)).rgb * weight[i] * ubo.blurStrength;
			result += texture(samplerColor, inUV - vec2(0.0, tex_offset.y * i)).rgb * weight[i] * ubo.blurStrength;
		}
	}
	outFragColor = vec4(result, 1.0);
}