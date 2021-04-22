#![allow(dead_code)]

use wgpu::*;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::window::Window;

pub struct Funny {
	surface: Surface,
	device: Device,
	queue: Queue,
	sc_desc: SwapChainDescriptor,
	sc: SwapChain,
	size: PhysicalSize<u32>,
	pipeline: RenderPipeline,
}

impl Funny {
	pub async fn new(window: &Window) -> Self {
		let size = window.inner_size();

		let instance = Instance::new(BackendBit::PRIMARY);

		//safety: Window is, indeed, a valid window handle
		let surface = unsafe { instance.create_surface(window) };

		//an adapter is a handle to a graphics device
		let adapter = instance
			.request_adapter(&RequestAdapterOptions { power_preference: PowerPreference::default(), compatible_surface: Some(&surface) })
			.await
			.expect("could not create adapter");

		//device: An *open* connection to a graphics device
		//queue: Command queue for this device
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

		let sc_desc = SwapChainDescriptor {
			usage: TextureUsage::RENDER_ATTACHMENT,
			format: adapter.get_swap_chain_preferred_format(&surface),
			width: size.width,
			height: size.height,
			present_mode: PresentMode::Fifo, //"vsync"
		};

		let sc = device.create_swap_chain(&surface, &sc_desc);

		let base_path =	std::path::Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| "./".to_string())).to_path_buf();
		
		let mut vertex_shader_path = base_path.clone();
		vertex_shader_path.push("shader_spv/triangle.vert.spv");
		
		let mut fragment_shader_path = base_path;
		fragment_shader_path.push("shader_spv/triangle.frag.spv");
		
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

		let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
			label: Some("render pipeline layout"),
			bind_group_layouts: &[],
			push_constant_ranges: &[],
		});

		let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
			label: Some("render pipeline"),
			layout: Some(&render_pipeline_layout),
			vertex: VertexState { module: &vertex_shader_module, entry_point: "main", buffers: &[] },
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
				strip_index_format: None,
				front_face: FrontFace::Ccw,
				cull_mode: CullMode::Back,
				polygon_mode: PolygonMode::Fill,
			},
			depth_stencil: None,
			multisample: MultisampleState { count: 1, mask: !0, alpha_to_coverage_enabled: false },
		});

		Self { surface, device, queue, sc_desc, sc, size, pipeline }
	}

	pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
		//Update my copy of the size
		self.size = new_size;

		//Recreate the swap chain with this new size
		self.sc_desc.width = new_size.width;
		self.sc_desc.height = new_size.height;
		self.create_swap_chain();
	}

	pub fn create_swap_chain(&mut self) {
		self.sc = self.device.create_swap_chain(&self.surface, &self.sc_desc);
	}

	// return true if you handled the window event just fine and don't want to pass it on to the windowing system
	pub fn handle_input(&mut self, _event: &WindowEvent) -> bool {
		false
	}

	pub fn update(&mut self) {
		//nothing yet
	}

	pub fn render(&mut self) -> Result<(), SwapChainError> {
		//"Here's where the magic happens."

		let frame = self.sc.get_current_frame()?.output;

		let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor { label: Some("render encoder") });

		//Clear the screen
		let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
			label: Some("Clear"),
			color_attachments: &[RenderPassColorAttachmentDescriptor {
				attachment: &frame.view,
				resolve_target: None,
				ops: Operations { load: LoadOp::Clear(Color { r: 0.02, g: 0.02, b: 0.02, a: 1.0 }), store: true },
			}],
			depth_stencil_attachment: None,
		});

		render_pass.set_pipeline(&self.pipeline);
		render_pass.draw(0..3, 0..1);

		drop(render_pass); //permit calling .finish() on the encoder
		self.queue.submit(std::iter::once(encoder.finish()));

		Ok(())
	}
}
