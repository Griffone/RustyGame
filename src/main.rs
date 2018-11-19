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


mod config;
mod graphics;


use config::Configuration as Config;
use glium::{glutin, Surface};
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
	let mut config = Config::load_or_default("config.yml").unwrap();

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
		let window = display.gl_window();

		let program = graphics::load_program(&display, &config.vert_shader, &config.frag_shader).unwrap();
		let (vertex_buffer, index_buffer) = graphics::generate_quad(&display).unwrap();

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

		while !state.closed {
			{
				let mut width_to_height = 1.0f32;
				if let Some(size) = window.get_inner_size() {
					width_to_height = size.width as f32 / size.height as f32;
				}

				let uniforms = uniform! {
					u_translate: [0.0, 0.0f32],
					u_z_theta: [0.0, 0.0f32],
					u_scale: [1.0, width_to_height],

					u_color: [1.0, 1.0, 0.0, 1.0f32],
				};

				let mut target = display.draw();
				target.clear_color(0.0, 0.0, 1.0, 1.0);
				target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default()).unwrap();
				target.finish().unwrap();
			}


			events_loop.poll_events(|event| {
				process_event(&event, &window, &mut state, config.debug_mode);
			});
		}

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