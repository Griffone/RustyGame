// An instance of a renderable object

use super::math::Lerp;
use super::texture::Texture;
use super::transform::Transform;

#[derive(Clone)]
pub struct Instance {
	pub transform: Transform,
	pub color_lit: [f32; 4],
	pub color_unlit: [f32; 4],
	pub texture_lit: Texture,
	pub texture_unlit: Texture,
}

impl Instance {
	fn new(
		transform: Transform,
		color_lit: [f32; 4],
		color_unlit: [f32; 4],
		texture_lit: &Texture,
		texture_unlit: &Texture,
	) -> Self {
		Self {
			transform,
			color_lit,
			color_unlit,
			texture_lit: texture_lit.clone(),
			texture_unlit: texture_unlit.clone(),
		}
	}

	fn unlit(transform: Transform, color: [f32; 4], texture: &Texture) -> Self {
		Self {
			transform,
			color_lit: color,
			color_unlit: color,
			texture_lit: texture.clone(),
			texture_unlit: texture.clone(),
		}
	}
}

impl Lerp for Instance {
	fn lerp(a: &Self, b: &Self, t: f32) -> Self {
		let texture_lit;
		let texture_unlit;
		if t <= 0.5 {
			texture_lit = a.texture_lit.clone();
			texture_unlit = a.texture_unlit.clone();
		} else {
			texture_lit = b.texture_lit.clone();
			texture_unlit = b.texture_unlit.clone();
		}
		Self {
			transform: Lerp::lerp(&a.transform, &b.transform, t),
			color_lit: Lerp::lerp(&a.color_lit, &b.color_lit, t),
			color_unlit: Lerp::lerp(&a.color_unlit, &b.color_unlit, t),
			texture_lit,
			texture_unlit,
		}
	}
}

// Data structure that is passed to shaders for each instance
#[derive(Copy, Clone)]
pub struct PerInstance {
	pub i_translation: [f32; 2],	// Translation in world space
	pub i_z_theta: [f32; 2],		// Z-order and angle of rotation around origin in radians
	pub i_scale: [f32; 2],			// Scaling of the object in world space
	pub i_color_lit: [f32; 4],		// The color of the object when within vision range
	pub i_color_unlit: [f32; 4],	// The color of the object when not within vision range
	pub i_texture_lit: [f32; 4],	// Offsets to texture used when in vision range
	pub i_texture_unlit: [f32; 4],	// Offsets to texture used when not within vision range
}
implement_vertex!(PerInstance, i_translation, i_z_theta, i_scale, i_color_lit, i_color_unlit, i_texture_lit, i_texture_unlit);

impl Default for PerInstance {
	fn default() -> Self {
		Self {
			i_translation: [0.0, 0.0],
			i_z_theta: [0.5, 0.0],
			i_scale: [1.0, 1.0],
			i_color_lit: [1.0, 1.0, 1.0, 1.0],
			i_color_unlit: [0.5, 0.5, 0.5, 1.0],
			i_texture_lit: [0.0, 0.0, 0.0, 0.0],
			i_texture_unlit: [0.0, 0.0, 0.0, 0.0],
		}
	}
}

impl From<Instance> for PerInstance {
	fn from(instance: Instance) -> Self {
		Self {
			i_translation: instance.transform.translation,
			i_z_theta: [0.5, instance.transform.rotation],
			i_scale: instance.transform.scale,
			i_color_lit: instance.color_lit,
			i_color_unlit: instance.color_unlit,
			i_texture_lit: instance.texture_lit.area.get_vec4(),
			i_texture_unlit: instance.texture_unlit.area.get_vec4(),
		}
	}
}
