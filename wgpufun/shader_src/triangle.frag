#version 450

layout(location=0) in vec3 v_color;
layout(location=1) in vec2 v_uv;

layout(set=0, binding=0) uniform texture2D t_diffuse;
layout(set=0, binding=1) uniform sampler s_diffuse;

//actually not sure why drawing to a vec4 in position 0 mgically draws to the screen
//probably something about how the framebuffer was set up
layout(location=0) out vec4 f_color;

void main() {
	//f_color = texture(sampler2D(t_diffuse, s_diffuse), v_uv);
	vec3 tex = texture(sampler2D(t_diffuse, s_diffuse), v_uv).xyz;
	vec3 color = v_color;
	
	f_color = vec4(tex.x * color.x, tex.y * color.y, tex.z * color.z, 1.0);
}