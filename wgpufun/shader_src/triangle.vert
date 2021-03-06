#version 450

//See the layout in model::Vertex?
layout(location=0) in vec3 a_position;
layout(location=1) in vec2 a_uv;
layout(location=2) in vec3 a_color;

layout(location=0) out vec3 v_color;
layout(location=1) out vec2 v_uv;

void main() {
	//Copy the color and texture unchanged from the vertex data I'm responsible for, into the inputs that the fragment shader's responsible for
	v_color = a_color;
	v_uv = a_uv;
	
	//Set the magic gl_position variable to position this vertex in 3d space
	//Homogenous coordinates etc etc
	gl_Position = vec4(a_position, 1.0);
}