#[macro_use]
extern crate glium;

use std::io;
use glium::{
	glutin,
	Surface
};

fn main() {

	// Use inside scope to close the window just before the program end.
	{
		let mut events_loop = glutin::EventsLoop::new();
		let window = glutin::WindowBuilder::new()
			.with_title("Rusty Game")
			.with_min_dimensions(glutin::dpi::LogicalSize::new(800.0, 600.0))
			.with_dimensions(glutin::dpi::LogicalSize::new(800.0, 600.0));
		let context = glutin::ContextBuilder::new()
			.with_vsync(true);
		let display = glium::Display::new(window, context, &events_loop).unwrap();

		let mut closed = false;
		while !closed {
			let mut target = display.draw();
			target.clear_color(0.0, 0.0, 1.0, 1.0);
			target.finish().unwrap();

			events_loop.poll_events(|event| {
				match event {
					// Shadowed event
					glutin::Event::WindowEvent { event, .. } => match event {
						glutin::WindowEvent::CloseRequested => closed = true,
						_ => (),
					},
					_ => (),
				}
			});
		}
	}

	println!("Program closing, please press enter to finish!");

	let mut line = String::new();
	io::stdin().read_line(&mut line)
		.expect("Failed to read line!");
}