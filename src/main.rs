// Import all external crates in project root.
// This is to avoid later namespace confusions as well as be able to use #[macro_use] macro.

#[macro_use]
extern crate glium;	// Rust OpenGL wrapper library used for graphics throughout the project

extern crate rusttype;	// Rust TrueType library used for text drawing
extern crate unicode_normalization;	// Small Unicode utility that combines multiple code points (ex. diacritics).

extern crate serde;			// Serialize-deserialize rust library, used for configs and save files (probably)
extern crate serde_yaml;	
#[macro_use]
extern crate serde_derive;

mod config;

use std::io;
use glium::{
	glutin,
	Surface
};
use config::Configuration as Config;

struct WindowState {
	fullscreen: bool,
	closed: bool,
}

fn main() {
	let mut config = Config::load_or_default("config.yml").unwrap();

	println!("Config: {:?}", config);
	// Use inside scope to close the window just before the program end.
	{
		let mut events_loop = glutin::EventsLoop::new();
		let window_builder = glutin::WindowBuilder::new()
			.with_title("Rusty Game")
			.with_min_dimensions((800.0, 600.0).into())
			.with_dimensions((800.0, 600.0).into());
		let context_builder = glutin::ContextBuilder::new()
			.with_vsync(true);
		let display = glium::Display::new(window_builder, context_builder, &events_loop).unwrap();
		let window = display.gl_window();

		let mut state = WindowState{fullscreen: config.fullscreen_enabled, closed: false};
		set_fullscreen(&window, state.fullscreen);

		while !state.closed {
			let mut target = display.draw();
			target.clear_color(0.0, 0.0, 1.0, 1.0);
			target.finish().unwrap();

			events_loop.poll_events(|event| {
				process_event(&event, &window, &mut state);
			});
		}

		config.set_fullscreen_enabled(state.fullscreen);
	}

	println!("Config: {:?}", config);
	config.save_as("config.yml").unwrap();

	println!("Program closing, please press enter to finish!");

	let mut line = String::new();
	io::stdin().read_line(&mut line)
		.expect("Failed to read line!");
}

fn set_fullscreen(window: &glutin::GlWindow, fullscreen: bool) {
	let monitor = if fullscreen {
		Some(window.get_current_monitor())
	} else {
		None
	};
	window.set_fullscreen(monitor);
}

// Process all window events
fn process_event(event: &glutin::Event, window: &glutin::GlWindow, state: &mut WindowState) {
	match event {
		// Shadowed event
		glutin::Event::WindowEvent { event, .. } => match event {
			glutin::WindowEvent::CloseRequested => state.closed = true,
			glutin::WindowEvent::KeyboardInput { input, .. } => {
				match input.virtual_keycode {
					Some(glutin::VirtualKeyCode::F11) => {
						if input.state == glutin::ElementState::Pressed {
							state.fullscreen = !state.fullscreen;
							set_fullscreen(window, state.fullscreen);
						}
					},
					_ => ()
				}
			},
			_ => (),
		},
		_ => (),
	}
}