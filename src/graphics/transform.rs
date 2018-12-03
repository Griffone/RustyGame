use super::math::{Lerp, MAX_ROTATION};

// Transformation of a drawable object
#[derive(Clone, Debug)]
pub struct Transform {
	pub translation: [f32; 2],	// Position of the object in world space
	pub rotation: f32,			// Rotation around the origin of the object in radians
	pub scale: [f32; 2],		// Scaling of the object in world coordinate-space
}

impl Default for Transform {
	fn default() -> Self {
		Self {translation: [0.0, 0.0], rotation: 0.0, scale: [1.0, 1.0]}
	}
}

impl Transform {
	pub fn new(translation: [f32; 2], rotation: f32, scale: [f32; 2]) -> Self {
		Self {translation: translation, rotation: rotation, scale: scale}
	}

	pub fn rotate(&mut self, angle: f32) {
		self.rotation = (self.rotation + angle) % MAX_ROTATION;
	}

	pub fn set_rotation(&mut self, angle: f32) {
		debug_assert!(angle >= 0.0 && angle <= MAX_ROTATION, "Rotation angle out of bounds!");
		self.rotation = angle;
	}

	pub fn translate(&mut self, delta: [f32; 2]) {
		self.translation[0] += delta[0];
		self.translation[1] += delta[1];
	}

	pub fn set_position(&mut self, position: [f32; 2]) {
		self.translation = position;
	}

	pub fn set_scale(&mut self, scale: [f32; 2]) {
		self.scale = scale;
	}
}

impl Lerp for Transform {
	fn lerp(a: &Self, b: &Self, t: f32) -> Self {
		Self {translation: Lerp::lerp(&a.translation, &b.translation, t), rotation: Lerp::lerp(&a.rotation, &b.rotation, t), scale: Lerp::lerp(&a.scale, &b.scale, t)}
	}
}