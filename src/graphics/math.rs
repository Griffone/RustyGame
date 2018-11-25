// Helper functions used in graphics module
pub use std::f32::consts::PI;

pub const MAX_ROTATION: f32 = PI * 2.0;

type Point = [f32; 2];
type Bounds = Vec<Point>;

/// Linearly interpolateable
pub trait Lerp {
	fn lerp(a: &Self, b: &Self, t: f32) -> Self;
}

/// Can be converted to a set of Points
pub trait Boundable {
	fn bounds(&self) -> Bounds;
}

#[derive(Copy, Clone, Debug)]
pub struct Rect {
	min: Point,
	max: Point,
}

pub fn short_angle_distance(a: f32, b: f32) -> f32 {
	let delta = (b - a) % MAX_ROTATION;

	(2.0 * delta) % MAX_ROTATION - delta
}

pub fn lerp_angle(a: f32, b: f32, t: f32) -> f32 {
	a + short_angle_distance(a, b) * t
}

impl Rect {
	/// Create a new rectangle from two points, no checking is done to make sure min is actually less than max
	/// 
	/// To generate a rectangle that contains two arbitrary points use bounds()
	pub fn new(min: Point, max: Point) -> Self {
		Self {min: min, max: max}
	}

	/// Create a smallest rectangle that includes given points
	pub fn from_bounds(points: &Bounds) -> Self {
		let mut min = [std::f32::MAX, std::f32::MAX];
		let mut max = [std::f32::MIN, std::f32::MIN];

		for point in points {
			if min[0] > point[0] {
				min[0] = point[0];
			}
			if min[1] > point[1] {
				min[1] = point[1];
			}

			if max[0] < point[0] {
				max[0] = point[0];
			}
			if max[1] < point[1] {
				max[1] = point[1];
			}
		}

		Self {min: min, max: max}
	}

	pub fn grow_to_fit(&mut self, point: &Point) {
		if self.min[0] > point[0] {
			self.min[0] = point[0];
		}
		if self.min[1] > point[1] {
			self.min[1] = point[1];
		}

		if self.max[0] < point[0] {
			self.max[0] = point[0];
		}
		if self.max[1] < point[1] {
			self.max[1] = point[1];
		}
	}

	pub fn min(&self) -> [f32; 2] {
		self.min
	}

	pub fn min_x(&self) -> f32 {
		self.min[0]
	}

	pub fn min_y(&self) -> f32 {
		self.min[1]
	}

	pub fn max(&self) -> [f32; 2] {
		self.max
	}

	pub fn max_x(&self) -> f32 {
		self.max[0]
	}

	pub fn max_y(&self) -> f32 {
		self.max[1]
	}

	pub fn width(&self) -> f32 {
		self.max[0] - self.min[0]
	}

	pub fn height(&self) -> f32 {
		self.max[1] - self.min[1]
	}

	pub fn center(&self) -> Point {
		[
			(self.min[0] + self.max[0]) / 2.0,
			(self.min[1] + self.max[1]) / 2.0,
		]
	}

	/// Returns a point where x = width and y = height
	pub fn size(&self) -> Point {
		[self.max[0] - self.min[0], self.max[1] - self.min[1]]
	}
}

impl Boundable for Rect {
	fn bounds(&self) -> Bounds {
		vec![self.min, self.max]
	}
}

impl Lerp for Rect {
	fn lerp(a: &Self, b: &Self, t: f32) -> Self {
		Self {
			min: Lerp::lerp(&a.min, &b.min, t),
			max: Lerp::lerp(&a.max, &b.max, t),
		}
	}
}

impl Lerp for f32 {
	fn lerp(a: &Self, b: &Self, t: f32) -> Self {
		let mt = 1.0 - t;
		a * mt + b * t
	}
}

impl Lerp for [f32; 2] {
	fn lerp(a: &Self, b: &Self, t: f32) -> Self {
		let mt = 1.0 - t;
		[a[0] * mt + b[0] * t, a[1] * mt + b[1] * t]
	}
}

impl Lerp for [f32; 3] {
	fn lerp(a: &Self, b: &Self, t: f32) -> Self {
		let mt = 1.0 - t;
		[
			a[0] * mt + b[0] * t,
			a[1] * mt + b[1] * t,
			a[2] * mt + b[2] * t,
		]
	}
}

impl Lerp for [f32; 4] {
	fn lerp(a: &Self, b: &Self, t: f32) -> Self {
		let mt = 1.0 - t;
		[
			a[0] * mt + b[0] * t,
			a[1] * mt + b[1] * t,
			a[2] * mt + b[2] * t,
			a[3] * mt + b[3] * t,
		]
	}
}