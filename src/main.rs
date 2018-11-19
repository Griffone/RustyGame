// Import all external crates in project root.
// This is to avoid later namespace confusions as well as be able to use #[macro_use] macro.

#[macro_use]
extern crate glium; // Rust OpenGL wrapper library used for graphics throughout the project

extern crate rusttype; // Rust TrueType library used for text drawing
extern crate unicode_normalization; // Small Unicode utility that combines multiple code points (ex. diacritics).

extern crate serde; // Serialize-deserialize rust library, used for configs and save files (probably)
extern crate serde_yaml;
#[macro_use]
extern crate serde_derive;

extern crate image;	// For loading texture files


mod config;
mod graphics;


use config::Configuration;
use graphics::Graphics;
use glium::glutin;
use std::io;


const FONT_PREFIX: &str = "data/fonts/";
const SHADER_PREFIX: &str = "data/shaders/";


#[derive(Debug)]
struct WindowState {
	fullscreen: bool,
	closed: bool,
	last_pos: (f64, f64),
}

fn main() {
	let mut config = Configuration::load_or_default(std::path::Path::new("config.yml"));

	if config.debug_mode {
		println!("Loaded config: {:?}", config);
	}

	// Use inside scope to close the window just before the program end.
	{
		let mut events_loop = glutin::EventsLoop::new();
		let window_builder = glutin::WindowBuilder::new()
			.with_title("Rusty Game")
			.with_min_dimensions((800.0, 600.0).into())
			.with_dimensions((800.0, 600.0).into());
		let context_builder = glutin::ContextBuilder::new().with_vsync(true);
		let display = glium::Display::new(window_builder, context_builder, &events_loop).unwrap();
		let graphics = Graphics::new(display, &config).unwrap();
		let window = graphics.window();

		let mut t = 0.0;

		let mut state = WindowState {
			fullscreen: false,
			closed: false,
			last_pos: (0.0, 0.0),
		};
		if let Some(position) = config.window_position {
			window.set_position(position.into());
			state.last_pos = position;
		}
		
		// Sleep for up to 20 milliseconds to let the window reposition
		for _ in 1..20 {
			if let Some(position) = window.get_position() {
				if state.last_pos == position.into() {
					break;
				}
			}
			std::thread::sleep(std::time::Duration::from_millis(1));
		}
		set_fullscreen(&window, config.fullscreen, &mut state);

		let begin = std::time::Instant::now();
		let mut frames = 0;
		while !state.closed {
			t += 0.01;
			graphics.draw(t);
			frames += 1;

			events_loop.poll_events(|event| {
				process_event(&event, &window, &mut state, config.debug_mode);
			});
		}
		let duration = std::time::Instant::now().duration_since(begin);
		let frame_rate = frames / duration.as_secs();
		println!("Produces {} frames over {:#?}, resuling in {} fps", frames, duration, frame_rate);

		config.set_window_position(state.last_pos);
		config.set_fullscreen(state.fullscreen);
	}

	config.save_as("config.yml").unwrap();

	if config.debug_mode {
		println!("Program closing, please press enter to finish!");

		let mut line = String::new();
		io::stdin()
			.read_line(&mut line)
			.expect("Failed to read line!");
	}
}

fn remember_position(window: &glutin::GlWindow, state: &mut WindowState) {
	state.last_pos = match window.get_position() {
		Some(position) => position.into(),
		None => (0.0, 0.0),
	};
}

fn set_fullscreen(window: &glutin::GlWindow, fullscreen: bool, state: &mut WindowState) {
	if fullscreen {
		remember_position(window, state);
		window.set_fullscreen(Some(window.get_current_monitor()));
	} else {
		window.set_fullscreen(None);
		window.set_position(state.last_pos.into());
	}
	state.fullscreen = fullscreen;
}

// Process all window events
fn process_event(
	event: &glutin::Event,
	window: &glutin::GlWindow,
	state: &mut WindowState,
	debug_mode: bool,
) {
	match event {
		glutin::Event::WindowEvent { event, .. } => match event {
			glutin::WindowEvent::CloseRequested => state.closed = true,
			glutin::WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
				Some(glutin::VirtualKeyCode::F11) => {
					if input.state == glutin::ElementState::Released {
						set_fullscreen(window, !state.fullscreen, state);
					}
				}
				Some(other) => {
					if debug_mode {
						println!(
							"Unknown key press: {:?} : {:?} ({:?})",
							other, input.state, input.modifiers
						);
					}
				}
				_ => (),
			},
			glutin::WindowEvent::Moved(position) => {
				if !state.fullscreen {
					state.last_pos = (position.x, position.y);
				}
			}
			_ => (),
		},
		_ => (),
	}
}