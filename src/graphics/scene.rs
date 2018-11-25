use super::instance::Instance;
use super::math::{Rect, MAX_ROTATION, PI};
use super::transform::Transform;

use rand::Rng;

// A Scene that can be rendered by Graphics object
pub trait Scene {
	/// Get an iterator over all objects in the scene.
	fn object_instances(&self) -> &[Instance];
	/// Minimal required view rectangle
	fn view_rect(&self) -> Rect;
	/// Should world coordinate ratio be preserved (actual view rectangle might be different from view rectangle to account for viewport ratio)
	fn preserve_ratio(&self) -> bool;
}

use std::time::Instant;

/// A basic temporary implementation of a Scene trait for testing purposes
pub struct TestScene {
	pub objects: Vec<Instance>,
	pub view_rect: Rect,

	pub rotation_speeds: Vec<f32>,
	last_update: Instant,
}

impl Scene for TestScene {
	fn object_instances(&self) -> &[Instance] {
		&self.objects
	}

	fn view_rect(&self) -> Rect {
		self.view_rect.clone()
	}

	fn preserve_ratio(&self) -> bool {
		true
	}
}

impl TestScene {
	pub fn generate(columns: u32, rows: u32) -> TestScene {
		let mut rng = rand::thread_rng();
		let mut objects = Vec::with_capacity((columns * rows) as usize);
		let mut rotations = Vec::with_capacity((columns * rows) as usize);

		for x in 0..columns {
			for y in 0..rows {
				objects.push(Instance {
					transform: Transform::new(
						[x as f32 + 0.5, y as f32 + 0.5],
						rng.gen_range(0.0, MAX_ROTATION),
						[1.0, 1.0],
					),
					color: [rng.gen(), rng.gen(), rng.gen(), 1.0],
				});
				rotations.push(rng.gen_range(-PI, PI));
			}
		}

		TestScene {
			objects: objects,
			view_rect: Rect::new(
				[0.0, 0.0],
				[columns as f32, rows as f32],
			),
			rotation_speeds: rotations,
			last_update: Instant::now(),
		}
	}

	pub fn update(&mut self) {
		let now = Instant::now();
		let delta = now.duration_since(self.last_update).subsec_micros() as f32 / 1000000.0;

		for (src, dest) in self.rotation_speeds.iter().zip(self.objects.iter_mut()) {
			dest.transform.rotate(delta * src);
		}
		self.last_update = now;
	}
}
