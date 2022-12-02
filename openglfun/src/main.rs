//Use the "windows subsystem" (as opposed to the console subsystem) when not compiled in debug mode.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::ptr::NonNull;

use beryllium::event::Event;
use beryllium::gl_window::GlAttr;
use beryllium::gl_window::GlProfile;
use beryllium::init::InitFlags;
use beryllium::init::Sdl;
use beryllium::window::WindowFlags;
use gl46::GlFns;
use gl46::GL_COLOR_BUFFER_BIT;
use zstring::zstr;
use zstring::ZStr;

pub fn main() -> Result<(), Box<dyn Error>> {
	//Create SDL
	let sdl = Sdl::init(InitFlags::EVERYTHING)?;

	//Configure GL options that i want for the window (they're globals with sdl, apparently)
	sdl.gl_set_attribute(GlAttr::MajorVersion, 4)?;
	sdl.gl_set_attribute(GlAttr::MinorVersion, 6)?;
	sdl.gl_set_attribute(GlAttr::Profile, GlProfile::Core as _)?;

	//Create the window
	let glwindow = sdl.create_gl_window(zstr!("woah"), None, (800, 600), WindowFlags::ALLOW_HIGHDPI | WindowFlags::RESIZABLE)?;
	glwindow.set_swap_interval(1)?; //SDL concept, enable vsync

	//Load OpenGL function pointers
	//Requires some song-and-dance. This stuff is Lokathorware, and lokathor must have experimented with this ZStr abstraction for beryllium but not for his other crates.
	let gl = unsafe {
		let loader = |ptr_to_zstring: *const u8| (glwindow.get_proc_address(ZStr::from_non_null_unchecked(NonNull::new_unchecked(ptr_to_zstring as _))) as _);
		GlFns::load_from(&loader)?
	};

	//Set the GL clear color to an ugly looking green
	unsafe {
		gl.ClearColor(0.5, 0.6, 0.2, 1.0);
	}

	//handle events
	'exit: loop {
		while let Some(e) = sdl.poll_event() {
			match e {
				//Fired when user presses the Close button or presses alt-f4 or whatever
				Event::Quit => break 'exit,

				//Fairly noisy events that i don't want to spam the log with, but they do get fired like all the other events
				Event::MouseMotion { .. } | Event::Keyboard { .. } => (),

				//Log the other events
				etc => println!("Other event: {:?}", etc),
			}
		}

		//Clear the color buffer
		unsafe {
			gl.Clear(GL_COLOR_BUFFER_BIT);
		}

		//Any OpenGL drawing commands (like the one i have, an incredibly exciting clear command) go to the backbuffer. It's time to show this buffer to the screen.
		//This function also blocks until the vsync timer expires. My computer has a 144hz display and this reports about 5-6ms between calls.
		let then = std::time::Instant::now();
		glwindow.swap_backbuffer();
		println!("swap_backbuffer call took about {} seconds", then.elapsed().as_secs_f32())
	}

	//beryllium takes care of calling SDL_Quit on Drop. We're all good.
	Ok(())
}
