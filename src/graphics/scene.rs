use super::instance::Instance;
use super::math::{Rect, MAX_ROTATION, PI, Point};
use super::transform::Transform;
use super::texture::{TextureCollection, GLTexture, TextureID};

use rand::Rng;

// A Scene that can be rendered by Graphics object
pub trait Scene {
	/// Get an iterator over all objects in the scene.
	fn object_instances(&self) -> &[Instance];
	/// Minimal required view rectangle
	fn view_rect(&self) -> Rect;
	/// Should world coordinate ratio be preserved (actual view rectangle might be different from view rectangle to account for viewport ratio)
	fn preserve_ratio(&self) -> bool {
		true
	}
	/// Origin of the 'looker' object
	fn view_origin(&self) -> Point {
		[0.0, 0.0]
	}
	/// Visibility distance from view origin
	fn view_distance(&self) -> f32 {
		std::f32::INFINITY
	}
	fn view_sharpness(&self) -> f32 {
		1.0
	}

	/// Reference to texture used for drawing instances
	fn texture(&self) -> &GLTexture;
}

use std::time::Instant;

/// A basic temporary implementation of a Scene trait for testing purposes
#[derive(Debug)]
pub struct TestScene {
	pub objects: Vec<Instance>,
	pub view_rect: Rect,

	pub rotation_speeds: Vec<f32>,
	pub view_origin: Point,
	pub view_distance: f32,
	last_update: Instant,

	pub texture_collection: TextureCollection,
	pub sharpness: f32,
}

impl Scene for TestScene {
	fn object_instances(&self) -> &[Instance] {
		&self.objects
	}

	fn view_rect(&self) -> Rect {
		self.view_rect.clone()
	}

	fn view_origin(&self) -> Point {
		self.view_origin
	}

	fn view_distance(&self) -> f32 {
		self.view_distance
	}

	fn texture(&self) -> &GLTexture {
		&self.texture_collection.texture
	}

	fn view_sharpness(&self) -> f32 {
		self.sharpness
	}
}

impl TestScene {
	pub fn generate(columns: u32, rows: u32, texture_collection: TextureCollection, lit_texture: TextureID, unlit_texture: TextureID) -> TestScene {
		let mut rng = rand::thread_rng();
		let mut objects = Vec::with_capacity((columns * rows) as usize);
		let mut rotations = Vec::with_capacity((columns * rows) as usize);

		let lit_texture = texture_collection.get(&lit_texture).unwrap();
		let unlit_texture = texture_collection.get(&unlit_texture).unwrap();

		for x in 0..columns {
			for y in 0..rows {
				objects.push(Instance {
					transform: Transform::new(
						[x as f32 + 0.5, y as f32 + 0.5],
						rng.gen_range(0.0, MAX_ROTATION),
						[1.0, 1.0],
					),
					color_lit: [rng.gen(), rng.gen(), rng.gen(), 1.0],
					color_unlit: [rng.gen_range(0.0, 0.5), rng.gen_range(0.0, 0.5), rng.gen_range(0.0, 0.5), rng.gen()],
					texture_lit: lit_texture,
					texture_unlit: unlit_texture,
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
			view_distance: ((columns * rows) as f32).powf(1.0 / 4.0),
			view_origin: [0.0, 0.0],

			texture_collection: texture_collection,
			sharpness: 1.0,
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

	pub fn free_texture_collection(self) -> TextureCollection {
		self.texture_collection
	}
}
