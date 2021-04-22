#version 450

layout(location=0) in vec3 v_color;

//actually not sure why drawing to a vec4 in position 0 mgically draws to the screen
//probably something about how the framebuffer was set up
layout(location=0) out vec4 f_color;

void main() {
	f_color = vec4(v_color, 1.0);
}