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

		println!("Adapter: {:?}", adapter);

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

		println!("Device: {:?}", device);
		println!("Queue: {:?}", queue);

		let sc_desc = SwapChainDescriptor {
			usage: TextureUsage::RENDER_ATTACHMENT,
			format: adapter.get_swap_chain_preferred_format(&surface),
			width: size.width,
			height: size.height,
			present_mode: PresentMode::Fifo, //"vsync"
		};

		let sc = device.create_swap_chain(&surface, &sc_desc);

		Self { surface, device, queue, sc_desc, sc, size }
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
		
		let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor {
		    label: Some("render encoder"),
		});
		
		//Clear the screen
		let render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
		    label: Some("Clear"),
		    color_attachments: &[
				RenderPassColorAttachmentDescriptor {
					attachment: &frame.view,
					resolve_target: None,
					ops: Operations {
						load: LoadOp::Clear( Color {
						    r: 1.0,
						    g: 0.7,
						    b: 0.2,
						    a: 1.0,
						}),
						store: true
					}
				}
			],
		    depth_stencil_attachment: None,
		});
		
		//permit calling .finish() on the encoder
		drop(render_pass);
		
		self.queue.submit(std::iter::once(encoder.finish()));
		
		Ok(())
	}
}