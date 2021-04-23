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
	index_buffer: Buffer,
	index_count: u32,
	texture_bind_group: BindGroup,
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
		let mut asset_base_path = std::path::Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| "./".to_string())).to_path_buf();
		asset_base_path.push("assets");

		//Load spir-v code into cpu memory.
		//Ideally in real games you'd have a proper "assets" folder
		//Maybe look at how Veloren and ggez do asset management
		let mut vertex_shader_path = asset_base_path.clone();
		vertex_shader_path.push("compiled_shaders/triangle.vert.spv");

		let mut fragment_shader_path = asset_base_path.clone();
		fragment_shader_path.push("compiled_shaders/triangle.frag.spv");

		//Kiss the shader code goodbye onto the gpu
		let vertex_shader_module = device.create_shader_module(&ShaderModuleDescriptor {
			label: Some("vertex shader"),
			source: wgpu::util::make_spirv(&std::fs::read(vertex_shader_path).expect("vert shader")),
			flags: Default::default(),
		});

		let fragment_shader_module = device.create_shader_module(&ShaderModuleDescriptor {
			label: Some("fragment shader"),
			source: wgpu::util::make_spirv(&std::fs::read(fragment_shader_path).expect("frag_shader")),
			flags: Default::default(),
		});

		//Load an image to be used as a texture
		let mut image_path = asset_base_path;
		image_path.push("textures/spicy.png");
		let image_diffuse_bytes = std::fs::read(image_path).expect("image");
		let image_diffuse = image::load_from_memory(&image_diffuse_bytes).expect("parse image");
		let image_diffuse_rgba = image_diffuse.as_rgba8().expect("parse image as rgba8");

		//Create a texture for the image to be placed into, of just the right size
		let dimensions = image_diffuse_rgba.dimensions();
		let texture_size = Extent3d { width: dimensions.0, height: dimensions.1, depth: 1 };

		let diffuse_texture = device.create_texture(&TextureDescriptor {
			label: Some("my texture"),
			size: texture_size,
			mip_level_count: 1,
			sample_count: 1,
			dimension: TextureDimension::D2,
			format: TextureFormat::Rgba8UnormSrgb,
			usage: TextureUsage::SAMPLED | TextureUsage::COPY_DST,
		});

		//Copy the image data into the texture. This is done over the queue
		queue.write_texture(
			TextureCopyView { texture: &diffuse_texture, mip_level: 0, origin: Origin3d::ZERO },
			image_diffuse_rgba,
			TextureDataLayout {
				offset: 0,
				bytes_per_row: 4 * dimensions.0, // (r g b a) * width. Note that in some cases this must be a multiple of 256? Not when using write_texture tho
				rows_per_image: dimensions.1,
			},
			texture_size,
		);

		//"Now that our texture has data in it, we need a way to use it. This is where a TextureView and a Sampler come in."
		//Lots of properties here but the defaults suffice. Mostly boring stuff like mipmaps
		let texture_view = diffuse_texture.create_view(&TextureViewDescriptor::default());
		let texture_sampler = device.create_sampler(&SamplerDescriptor {
			label: Some("yeet"),
			address_mode_u: AddressMode::ClampToEdge, //What happens when you index outside the texture
			address_mode_v: AddressMode::ClampToEdge,
			address_mode_w: AddressMode::ClampToEdge,
			mag_filter: FilterMode::Linear,     //When enlarged, bilinearly filter between pixels
			min_filter: FilterMode::Nearest,    //When shrunk, crunch the image into chewy puxels
			mipmap_filter: FilterMode::Nearest, //i guess choose the nearest mipmap witout blending between them, but im not using mips either way
			..Default::default()
		});

		let texture_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
			label: Some("my bind group layout!!"),
			entries: &[
				BindGroupLayoutEntry {
					binding: 0,
					visibility: ShaderStage::FRAGMENT,
					ty: BindingType::Texture {
						multisampled: false,
						view_dimension: TextureViewDimension::D2,
						sample_type: TextureSampleType::Float { filterable: false },
					},
					count: None,
				},
				BindGroupLayoutEntry {
					binding: 1,
					visibility: ShaderStage::FRAGMENT,
					ty: BindingType::Sampler { comparison: false, filtering: true },
					count: None,
				},
			],
		});

		let texture_bind_group = device.create_bind_group(&BindGroupDescriptor {
			label: Some("my bind group!!"),
			layout: &texture_bind_group_layout,
			entries: &[
				BindGroupEntry { binding: 0, resource: BindingResource::TextureView(&texture_view) },
				BindGroupEntry { binding: 1, resource: BindingResource::Sampler(&texture_sampler) },
			],
		});
		//FINALLY done with texture shit

		//Create a vertex buffer.
		//The vertex format is described in model.rs, it's VERTEX_FORMAT_POSITION_COLOR if you will
		let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some("my vertex buffer!!!"),
			//Fill it in with some data, this'll get sent off to the gpu on creation
			contents: bytemuck::cast_slice(model::FUNNY_RECTANGLE),
			usage: BufferUsage::VERTEX,
		});
		
		let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some("my index buffer???????"),
			contents: bytemuck::cast_slice(model::FUNNY_RECTANGLE_INDICES),
			usage: BufferUsage::INDEX
		});
		let index_count = model::FUNNY_RECTANGLE_INDICES.len() as u32;

		//Time to make a render pipeline
		let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
			label: Some("render pipeline layout"),
			bind_group_layouts: &[&texture_bind_group_layout],
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
				cull_mode: CullMode::None,       //"none" is also available
				polygon_mode: PolygonMode::Fill, //Setting this to not "fill" requires some kind of feature idk see the guide
			},
			depth_stencil: None,
			multisample: MultisampleState { count: 1, mask: !0, alpha_to_coverage_enabled: false }, //Multisampling disabled
		});

		//Nice
		Self { surface, device, queue, sc_desc, sc, size, pipeline, vertex_buffer, index_buffer, index_count, texture_bind_group }
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

		//Activate the rendering pipeline and the shaders and stuff and draw
		render_pass.set_pipeline(&self.pipeline);
		render_pass.set_bind_group(0, &self.texture_bind_group, &[]);
		render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
		render_pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint16);
		render_pass.draw_indexed(0..self.index_count, 0, 0..1);

		drop(render_pass); //permit calling .finish() on the encoder
		self.queue.submit(std::iter::once(encoder.finish()));

		Ok(())
	}
}
