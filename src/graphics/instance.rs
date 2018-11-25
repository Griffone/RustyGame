// An instance of a renderable object

use super::math::Lerp;
use super::transform::Transform;

#[derive(Clone)]
pub struct Instance {
	pub transform: Transform,
	pub color: [f32; 4],
}

impl Instance {
	fn new(transform: Transform, color: [f32; 4]) -> Self {
		Self {transform: transform, color: color}
	}
}

impl Lerp for Instance {
	fn lerp(a: &Self, b: &Self, t: f32) -> Self {
		Self {transform: Lerp::lerp(&a.transform, &b.transform, t), color: Lerp::lerp(&a.color, &b.color, t)}
	}
}

// Data structure that is passed to shaders for each instance
#[derive(Copy, Clone)]
pub struct PerInstance {
	pub i_translation: [f32; 2], // Translation in world space
	pub i_z_theta: [f32; 2],     // Z-order and angle of rotation around origin in radians
	pub i_scale: [f32; 2],       // Scaling of the object in world space
	pub i_color: [f32; 4],       // The color of the object
}
implement_vertex!(PerInstance, i_translation, i_z_theta, i_scale, i_color);

impl Default for PerInstance {
	fn default() -> Self {
		Self {i_translation: [0.0, 0.0], i_z_theta: [0.5, 0.0], i_scale: [1.0, 1.0], i_color: [1.0, 1.0, 1.0, 1.0]}
	}
}

impl From<Instance> for PerInstance {
	fn from(instance: Instance) -> Self {
		Self {i_translation: instance.transform.translation, i_z_theta: [0.5, instance.transform.rotation], i_scale: instance.transform.scale, i_color: instance.color}
	}
}