use wgpu::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
	position: [f32; 3],
	texture: [f32; 2],
	color: [f32; 3],
}

impl Vertex {
	// pub fn _layout<'a>() -> VertexBufferLayout<'a> {
	// 	//Tell wgpu what my struct looks like and the purpose of each field.
	// 	VertexBufferLayout {
	// 		array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
	// 		step_mode: InputStepMode::Vertex,
	// 		attributes: &[
	// 			VertexAttribute {
	// 				offset: 0, //offset from the start of the structure
	// 				shader_location: 0,
	// 				format: VertexFormat::Float3,
	// 			},
	// 			VertexAttribute {
	// 				offset: std::mem::size_of::<[f32; 3]>() as BufferAddress, //the previous "position"
	// 				shader_location: 1,
	// 				format: VertexFormat::Float3,
	// 			},
	// 		],
	// 	}
	// }

	//the guide also provides this as an example, using a macro provided by wgpu because i mean.. look at the code above, lol
	//i do have to return a 'static one since it's baked into the executable, i guess?
	//the article says that *dejectedly* but i mean, it's probably better right?
	pub fn layout() -> VertexBufferLayout<'static> {
		//also the code as-is in the article doesn't.. compile?
		//had to split this out into a separate field
		static ATTRS: &[VertexAttribute] = &vertex_attr_array![0 => Float3, 1 => Float2, 2 => Float3];

		VertexBufferLayout { array_stride: std::mem::size_of::<Vertex>() as BufferAddress, step_mode: InputStepMode::Vertex, attributes: ATTRS }
	}
}

// pub const FUNNY_TRIANGLE: &[Vertex] = &[
// 	Vertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
// 	Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
// 	Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
// ];

pub const FUNNY_RECTANGLE: &[Vertex] = &[
	//upper left
	Vertex { position: [-0.9, 0.9, 0.0], texture: [0.0, 0.0], color: [1.0, 0.0, 0.0] },
	//lower left
	Vertex { position: [-0.9, -0.9, 0.0], texture: [0.0, 1.0], color: [1.0, 1.0, 0.0] },
	//upper right
	Vertex { position: [0.9, 0.9, 0.0], texture: [1.0, 0.0], color: [0.0, 1.0, 1.0] },
	//lower right
	Vertex { position: [0.9, -0.9, 0.0], texture: [1.0, 1.0], color: [0.0, 0.0, 1.0] },
];

pub const FUNNY_RECTANGLE_INDICES: &[u16] = &[
	0, 1, 2, //upper left triangle CCW order
	1, 3, 2 //lower right triangle CCW order
];