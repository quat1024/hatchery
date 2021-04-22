use funny::Funny;
use winit::event::*;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

mod funny;
mod model;

fn main() {
	env_logger::init();

	println!("CARGO_MANIFEST_DIR: {}", std::env::var("CARGO_MANIFEST_DIR").unwrap());

	let event_loop = EventLoop::new();
	let window = WindowBuilder::new()
		.with_resizable(true)
		.with_title("my super awesome window")
		.with_inner_size(winit::dpi::LogicalSize::new(640, 480))
		.build(&event_loop)
		.expect("couldn't build window");

	let mut funny = futures::executor::block_on(Funny::new(&window));

	event_loop.run(move |event, _window_target, control_flow| match event {
		//the tutorial grabs "event" here 👇 as ref, but i don't know if the extra indirection is needed
		Event::WindowEvent { window_id, event } if window_id == window.id() => {
			if !funny.handle_input(&event) {
				match event {
					WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
					WindowEvent::KeyboardInput {
						input: KeyboardInput { state: ElementState::Pressed, virtual_keycode: Some(VirtualKeyCode::Escape), .. },
						..
					} => *control_flow = ControlFlow::Exit,
					WindowEvent::Resized(physical_size) => {
						funny.resize(physical_size);
					},
					WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
						funny.resize(*new_inner_size);
					},
					_ => (),
				}
			}
		},
		//NOTE: winit says drawing in "redraw requested" is good for non-game UIs, but unconditionally drawing in "main events cleared"
		//is okay too, if you actually wanna draw something every frame
		Event::MainEventsCleared => {
			funny.update();
			match funny.render() {
				Ok(_) => (),
				Err(wgpu::SwapChainError::Lost | wgpu::SwapChainError::Outdated) => funny.recreate_swap_chain(),
				Err(wgpu::SwapChainError::OutOfMemory) => {
					eprintln!("Swap chain error: {:?}", wgpu::SwapChainError::OutOfMemory);
					*control_flow = ControlFlow::Exit
				},
				Err(something_else) => eprintln!("Swap chain error: {:?}", something_else),
			}
		},
		_ => (),
	});
}
