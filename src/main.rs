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

extern crate image;
extern crate rand; // For loading texture files

mod config;
mod graphics;
mod input;

use config::Configuration;
use graphics::{Graphics, TextureCollection};
use input::ActionId as InputActionId;
use input::Input;

use glium::glutin;

use std::io;

const FONT_PREFIX: &str = "data/fonts/";
const SHADER_PREFIX: &str = "data/shaders/";
const TEXTURE_PREFIX: &str = "data/textures/";

const CONFIG_NAME: &str = "config.yml";

const WINDOW_MIN_SIZE: (f64, f64) = (800.0, 600.0);
const WINDOW_DEFAULT_SIZE: (f64, f64) = (800.0, 600.0);

#[derive(Debug)]
struct WindowState {
	fullscreen: bool,
	closed: bool,
	mouse_pos: (f64, f64),
	wheel_delta: f32,
	last_pos: (f64, f64),
	last_size: (f64, f64),
	window_size: (f64, f64),
	change_size: i8,
	sharpness_delta: f32,
}

fn main() {
	let start_time = std::time::Instant::now();
	let mut config = Configuration::load_or_default(std::path::Path::new(CONFIG_NAME));

	if config.debug_mode {
		println!("Loaded config: {:?}", config);
	}

	// Use inside scope to close the window just before the program end.
	{
		let mut count: i8 = 3;

		let mut state = WindowState {
			fullscreen: false,
			closed: false,
			mouse_pos: (0.0, 0.0),
			wheel_delta: 0.0,
			last_pos: (0.0, 0.0),
			window_size: WINDOW_DEFAULT_SIZE,
			last_size: WINDOW_DEFAULT_SIZE,
			change_size: 0,
			sharpness_delta: 0.0,
		};

		if let Some(size) = config.window_size {
			state.last_size = size;
		}

		let mut events_loop = glutin::EventsLoop::new();
		let window_builder = glutin::WindowBuilder::new()
			.with_title("Rusty Game")
			.with_min_dimensions(WINDOW_MIN_SIZE.into())
			.with_dimensions(state.last_size.into());
		let context_builder = glutin::ContextBuilder::new().with_vsync(config.vsync);
		let display = glium::Display::new(window_builder, context_builder, &events_loop).unwrap();
		let mut graphics = Graphics::new(display, &config).unwrap();

		let mut input = Input::default();

		if let Some(position) = config.window_position {
			graphics.window().set_position(position.into());
			state.last_pos = position;

			// Sleep for up to 20 milliseconds to let the window reposition
			for _ in 1..20 {
				if let Some(position) = graphics.window().get_position() {
					if state.last_pos == position.into() {
						break;
					}
				}
				std::thread::sleep(std::time::Duration::from_millis(1));
			}
		}

		set_fullscreen(&graphics.window(), config.fullscreen, &mut state);
		input.set_on_up_action(
			input::Key {
				scancode: input::SCANCODE_F11,
				modifiers: input::MODIFIER_NONE,
			},
			InputActionId::ToggleFullscreen,
		);
		input.map_actions(|action| action_mapping(action));

		let texture_collection =
			TextureCollection::new(&graphics, &vec!["test.png", "dark.png"]).unwrap();
		let columns = count as u32 * 16;
		let rows = count as u32 * 9;
		let instance_count = columns * rows;
		let mut scene = graphics::scene::TestScene::generate(
			columns,
			rows,
			texture_collection,
			String::from("test.png"),
			String::from("dark.png"),
		);

		if config.debug_mode {
			println!(
				"Loaded in {:#?}",
				std::time::Instant::now().duration_since(start_time)
			);
			println!(
				"Drawing {}x{}={} instances in {} batches",
				columns,
				rows,
				instance_count,
				(instance_count as f32 / config.batch_size as f32).ceil() as i32
			);
		}

		let begin = std::time::Instant::now();
		let mut max_frametime = std::time::Duration::from_secs(0);
		let mut min_frametime = std::time::Duration::from_secs(1000);
		let mut frames = 0;

		while !state.closed {
			let frame_start = std::time::Instant::now();

			if state.change_size != 0 {
				count += state.change_size;
				let texture_collection = scene.free_texture_collection();
				if count < 0 {
					count = 0;
				}
				if count == 0 {
					scene = graphics::scene::TestScene::generate(
						1,
						1,
						texture_collection,
						String::from("test.png"),
						String::from("dark.png"),
					);
				} else {
					let columns = count as u32 * 16;
					let rows = count as u32 * 9;
					let instance_count = columns * rows;
					scene = graphics::scene::TestScene::generate(
						columns,
						rows,
						texture_collection,
						String::from("test.png"),
						String::from("dark.png"),
					);

					println!(
						"Drawing {}x{}={} instances in {} batches",
						columns,
						rows,
						instance_count,
						(instance_count as f32 / config.batch_size as f32).ceil() as i32
					);
				}
				state.change_size = 0;
			}
			scene.sharpness *= 1.0 + state.sharpness_delta / 8.0;
			state.sharpness_delta = 0.0;
			scene.update();
			let mouse = [
				(state.mouse_pos.0 / state.window_size.0) as f32,
				(state.mouse_pos.1 / state.window_size.1) as f32,
			];
			scene.view_origin = graphics.screen_to_world(mouse, &scene);
			scene.view_distance *= 1.0 + state.wheel_delta / 8.0;
			state.wheel_delta = 0.0;
			graphics.draw(&scene);

			events_loop.poll_events(|event| {
				process_event(
					&event,
					&mut input,
					&graphics.window(),
					&mut state,
					config.debug_mode,
				);
			});

			let frametime = std::time::Instant::now().duration_since(frame_start);
			frames += 1;
			if frametime > max_frametime {
				max_frametime = frametime;
			}
			if frametime < min_frametime {
				min_frametime = frametime;
			}
		}

		if config.debug_mode {
			let duration = std::time::Instant::now().duration_since(begin);
			let frame_rate = frames / duration.as_secs();
			let frametime = duration / frames as u32;
			println!(
				"Program ran for {:#?}, produced {} frames",
				duration, frames
			);
			println!(
				"Resulting in avereage frametime: {:#?} (avg fps {})",
				frametime, frame_rate
			);
			println!(
				"Max frametime: {:#?}, min frametime: {:#?}",
				max_frametime, min_frametime
			);
		}

		config.set_window_position(state.last_pos);
		config.set_fullscreen(state.fullscreen);
		config.set_window_size(state.last_size);
	}

	config.save_as(CONFIG_NAME).unwrap();

	if config.debug_mode {
		println!(
			"Total runtime: {:#?}",
			std::time::Instant::now().duration_since(start_time)
		);
		println!("Program closing, please press enter to finish!");

		let mut line = String::new();
		io::stdin()
			.read_line(&mut line)
			.expect("Failed to read line!");
	}
}

fn set_fullscreen(window: &glutin::GlWindow, fullscreen: bool, state: &mut WindowState) {
	if fullscreen {
		state.last_pos = match window.get_position() {
			Some(position) => position.into(),
			None => (0.0, 0.0),
		};
		state.last_size = match window.get_inner_size() {
			Some(size) => size.into(),
			None => WINDOW_DEFAULT_SIZE,
		};
		window.set_fullscreen(Some(window.get_current_monitor()));
	} else {
		window.set_fullscreen(None);
		window.set_position(state.last_pos.into());
	}
	state.fullscreen = fullscreen;
}

fn toggle_fullscreen() {
	println!("Need to figure out context passing.");
}

fn action_mapping(action_id: InputActionId) -> input::ActionCallback {
	match action_id {
		InputActionId::ToggleFullscreen => Box::new(|| toggle_fullscreen()),
		_ => Box::new(|| ()),
	}
}

// Process all window events
fn process_event(
	event: &glutin::Event,
	input: &mut Input,
	window: &glutin::GlWindow,
	state: &mut WindowState,
	debug_mode: bool,
) {
	match event {
		glutin::Event::WindowEvent { event, .. } => match event {
			glutin::WindowEvent::CloseRequested => state.closed = true,
			glutin::WindowEvent::KeyboardInput {
				input: keyboard_input,
				..
			} => input.process_key(&keyboard_input),
			glutin::WindowEvent::Moved(position) => {
				if !state.fullscreen {
					state.last_pos = (position.x, position.y);
				}
			}
			glutin::WindowEvent::Resized(size) => {
				if !state.fullscreen {
					state.last_size = (size.width, size.height);
				}
				state.window_size = (size.width, size.height);
			}
			glutin::WindowEvent::CursorMoved { position, .. } => {
				state.mouse_pos = (position.x, position.y);
			}
			glutin::WindowEvent::MouseWheel {
				delta, modifiers, ..
			} => {
				if modifiers.shift {
					match delta {
						glutin::MouseScrollDelta::LineDelta(_, delta) => {
							state.sharpness_delta = *delta
						}
						glutin::MouseScrollDelta::PixelDelta(delta) => {
							state.sharpness_delta = delta.y as f32
						}
					}
				} else if modifiers.alt {
					match delta {
						glutin::MouseScrollDelta::LineDelta(_, delta) => {
							state.change_size = *delta as i8
						}
						glutin::MouseScrollDelta::PixelDelta(delta) => {
							state.change_size = delta.y as i8
						}
					}
				} else {
					match delta {
						glutin::MouseScrollDelta::LineDelta(_, delta) => state.wheel_delta = *delta,
						glutin::MouseScrollDelta::PixelDelta(delta) => {
							state.wheel_delta = delta.y as f32
						}
					}
				}
			}
			other => {
				if debug_mode {
					println!("Unknown event: {:?}", other)
				}
			}
		},
		_ => (),
	}
}
