#![allow(dead_code)]

use wgpu::util::DeviceExt;
use wgpu::BufferUsage;
use wgpu::*;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::window::Window;

use crate::model;

pub struct Funny {
	surface: Surface,
	device: Device,
	queue: Queue,
	sc_desc: SwapChainDescriptor,
	sc: SwapChain,
	size: PhysicalSize<u32>,
	pipeline: RenderPipeline,
	vertex_buffer: Buffer,
}

impl Funny {
	pub async fn new(window: &Window) -> Self {
		let size = window.inner_size();

		//BackendBit is a specific-to-wgpu thing and nothing to do with WebGPU itself
		//PRIMARY means all the backends that wgpu provides primary support for.
		//These are vulkan, dx12, WebGPU in the browser, and Metal. (Notably, OpenGL is not a primary platform.)
		let instance = Instance::new(BackendBit::PRIMARY);

		//Ok cool create a surface
		//Safety: Window is, indeed, a valid window handle
		let surface = unsafe { instance.create_surface(window) };

		//An adapter is a reference to a graphics device like a GPU
		let adapter = instance
			.request_adapter(&RequestAdapterOptions { power_preference: PowerPreference::default(), compatible_surface: Some(&surface) })
			.await
			.expect("could not create adapter");

		//A device is an *opened* connection to a graphics device
		//You also get the command queue for this device right here
		let (device, queue) = adapter
			.request_device(
				&DeviceDescriptor {
					features: Features::empty(), //nothing special
					limits: Limits::default(),
					label: None,
				},
				None,
			)
			.await
			.expect("could not create device and queue");

		//Describe a swap chain we'd like to have
		let sc_desc = SwapChainDescriptor {
			usage: TextureUsage::RENDER_ATTACHMENT,
			format: adapter.get_swap_chain_preferred_format(&surface),
			width: size.width,
			height: size.height,
			present_mode: PresentMode::Fifo, //"vsync"
		};

		//and create it
		let sc = device.create_swap_chain(&surface, &sc_desc);

		//Load a vertex and fragment shader for our render pipeline.
		//wgpu only supports SPIR-V shader code, and uses `naga` (Rust) or `spirv_cross` (C) to transpile it to whatever format the backend accepts.
		//Actually that's a lie, wgpu can accept WGSL shaders, but it's less finished(?) and not available on wasm.
		//Because SPIR-V isn't editable by claw, the buildscript uses `shaderc` to transpile good old GLSL shaders which are easy to edit.
		let base_path = std::path::Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| "./".to_string())).to_path_buf();

		//Load spir-v code into cpu memory.
		//Ideally in real games you'd have a proper "assets" folder
		//Maybe look at how Veloren and ggez do asset management
		let mut vertex_shader_path = base_path.clone();
		vertex_shader_path.push("shader_spv/triangle.vert.spv");

		let mut fragment_shader_path = base_path;
		fragment_shader_path.push("shader_spv/triangle.frag.spv");

		//Kiss the shader code goodbye onto the gpu
		let vertex_shader_module = device.create_shader_module(&ShaderModuleDescriptor {
			label: Some("vertex shader"),
			source: wgpu::util::make_spirv(&std::fs::read(vertex_shader_path).expect("triangle.vert.spv")),
			flags: Default::default(),
		});

		let fragment_shader_module = device.create_shader_module(&ShaderModuleDescriptor {
			label: Some("fragment shader"),
			source: wgpu::util::make_spirv(&std::fs::read(fragment_shader_path).expect("triangle.vert.spv")),
			flags: Default::default(),
		});

		//Create a vertex buffer.
		//The vertex format is described in model.rs, it's VERTEX_FORMAT_POSITION_COLOR if you will
		let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some("my vertex buffer!!!"),
			//Fill it in with some data, this'll get sent off to the gpu on creation
			contents: bytemuck::cast_slice(model::FUNNY_TRIANGLE),
			usage: BufferUsage::VERTEX,
		});

		//Time to make a render pipeline. I don't need any of these things right now
		let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
			label: Some("render pipeline layout"),
			bind_group_layouts: &[],
			push_constant_ranges: &[],
		});

		let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
			label: Some("render pipeline"),
			layout: Some(&render_pipeline_layout),
			vertex: VertexState {
				module: &vertex_shader_module,
				entry_point: "main",
				buffers: &[
					//of type &[VertexBufferLayout]
					model::Vertex::layout(),
				],
			},
			fragment: Some(FragmentState {
				module: &fragment_shader_module,
				entry_point: "main",
				targets: &[ColorTargetState {
					format: sc_desc.format,
					alpha_blend: BlendState::REPLACE,
					color_blend: BlendState::REPLACE,
					write_mask: ColorWrite::ALL,
				}],
			}),
			primitive: PrimitiveState {
				topology: PrimitiveTopology::TriangleList,
				strip_index_format: None,        //only applies to ____Strip primitive topologies
				front_face: FrontFace::Ccw,      //"right handed coordinate space"
				cull_mode: CullMode::Back,       //"none" is also available
				polygon_mode: PolygonMode::Fill, //Setting this to not "fill" requires some kind of feature idk see the guide
			},
			depth_stencil: None,
			multisample: MultisampleState { count: 1, mask: !0, alpha_to_coverage_enabled: false }, //Multisampling disabled
		});

		//Nice
		Self { surface, device, queue, sc_desc, sc, size, pipeline, vertex_buffer }
	}

	pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
		//Update my copy of the size
		self.size = new_size;

		//Recreate the swap chain with this new size
		self.sc_desc.width = new_size.width;
		self.sc_desc.height = new_size.height;
		self.recreate_swap_chain();
	}

	pub fn recreate_swap_chain(&mut self) {
		//Lol having 0 dimension makes it crash
		if self.sc_desc.width != 0 && self.sc_desc.height != 0 {
			self.sc = self.device.create_swap_chain(&self.surface, &self.sc_desc);
		}
	}

	// Not strictly a wgpu concept, but ties into the winit main loop
	// return true if you handled the window event just fine and don't want to pass it on to the windowing system
	pub fn handle_input(&mut self, _event: &WindowEvent) -> bool {
		false
	}

	pub fn update(&mut self) {
		//nothing to do yet
	}

	pub fn render(&mut self) -> Result<(), SwapChainError> {
		//"Here's where the magic happens."

		//Get a framebuffer to render to
		let frame = self.sc.get_current_frame()?.output;

		//Let's create a pipeline of commands
		let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor { label: Some("render encoder") });

		//This command clears the screen
		let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
			label: Some("Clear"),
			color_attachments: &[RenderPassColorAttachmentDescriptor {
				attachment: &frame.view,
				resolve_target: None,
				ops: Operations { load: LoadOp::Clear(Color { r: 0.02, g: 0.02, b: 0.02, a: 1.0 }), store: true },
			}],
			depth_stencil_attachment: None,
		});

		//Activate the rendering pipeline and the shaders and stuff
		render_pass.set_pipeline(&self.pipeline);
		render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
		render_pass.draw(0..model::FUNNY_TRIANGLE.len() as u32, 0..1); //Ideally i would store the length alongside the actual vertex buffer lol

		drop(render_pass); //permit calling .finish() on the encoder
		self.queue.submit(std::iter::once(encoder.finish()));

		Ok(())
	}
}
