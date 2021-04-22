use funny::Funny;
use winit::event::*;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

mod funny;

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
		//NOTE: winit says this is a good event for "non-game GUIs"
		//This seems to agree with what little I know of the Windows windowing system, as well
		//OTOW what is the best method when I do want a game-like program?
		//
		//winit says drawing in "MainEventsCleared" is a good idea. I'll try that later.
		Event::RedrawRequested(_handler) => {
			funny.update();
			match funny.render() {
				Ok(_) => (),
				Err(wgpu::SwapChainError::Lost) => funny.create_swap_chain(),
				Err(wgpu::SwapChainError::OutOfMemory) => {
					eprintln!("wgpu::SwapChainError::OutOfMemory occured!");
					*control_flow = ControlFlow::Exit
				},
				Err(something_else) => eprintln!("{:?}", something_else),
			}
		},
		Event::MainEventsCleared => {
			//That was fun, let's request another frame!
			window.request_redraw();
		},
		_ => (),
	});
}
