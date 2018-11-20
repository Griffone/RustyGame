// Module that encopases all accelerated graphical presentation.

//mod texture;

use config::Configuration;
use {FONT_PREFIX, SHADER_PREFIX};

use glium::index::BufferCreationError as IndexBufferCreationError;
use glium::vertex::BufferCreationError as VertexBufferCreationError;
use glium::{Display, IndexBuffer, Surface, VertexBuffer};

use glium::program::{Program, ProgramCreationError};

use std::f32::consts::PI;
use std::io::Error as IoError;
use std::path::Path;

const INSTANCE_COLUMNS: usize = 50;
const INSTANCE_ROWS: usize = 50;
const INSTANCE_COUNT: usize = INSTANCE_COLUMNS * INSTANCE_ROWS;

#[derive(Copy, Clone)]
pub struct Vertex {
	position: [f32; 2],
	tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

// Graphical context
//
// The workhorse of this module. Abstracts away specifics of underlying 3D library calls for the purposes of the project.
pub struct Graphics {
	display: Display, // takes ownership of the glium display object
	program: Program, // shaders used to draw objects
	// more will be created later

	// temporary variables for testing
	verts: VertexBuffer<Vertex>,
	indcs: IndexBuffer<u16>,
	instances: VertexBuffer<PerInstance>,
	texture: glium::texture::CompressedSrgbTexture2d,
}

impl Graphics {
	pub fn new(display: Display, config: &Configuration) -> Result<Self, GraphicsCreationError> {
		let program = load_program(&display, &config.vert_shader, &config.frag_shader)?;

		let (verts, indcs) = generate_quad(&display)?;

		let instances = generate_instances(&display)?;

		let texture = image::open(std::path::Path::new("data/textures/test.png"))
			.unwrap()
			.to_rgba();
		let texture_size = texture.dimensions();
		let texture =
			glium::texture::RawImage2d::from_raw_rgba_reversed(&texture.into_raw(), texture_size);
		let texture = glium::texture::CompressedSrgbTexture2d::new(&display, texture).unwrap();

		Ok(Graphics {
			display: display,
			program: program,
			verts: verts,
			indcs: indcs,
			instances: instances,
			texture: texture,
		})
	}

	pub fn draw(&self) {
		let params = glium::DrawParameters {
			blend: glium::Blend::alpha_blending(),
			..Default::default()
		};

		let mut width_to_height = 1.0f32;
		if let Some(size) = self.display.gl_window().get_inner_size() {
			width_to_height = size.width as f32 / size.height as f32;
		}

		let uniforms = uniform! {
			u_scale: [1.0, width_to_height],
			u_texture: &self.texture,
		};

		let mut target = self.display.draw();
		target.clear_color(0.0, 0.0, 1.0, 1.0);
		target.draw((&self.verts, self.instances.per_instance().unwrap()), &self.indcs, &self.program, &uniforms, &params).unwrap();
		target.finish().unwrap();
	}

	pub fn window(&self) -> core::cell::Ref<glium::glutin::GlWindow> {
		self.display.gl_window()
	}
}

#[derive(Debug)]
pub enum GraphicsCreationError {
	Io(IoError),                   // Something went wrong trying to load shader files
	Program(ProgramCreationError), // Something went wrong trying to compile shaders
	VertexBuffer(VertexBufferCreationError), // Something went wrong trying to generate vertices for the quad
	IndexBuffer(IndexBufferCreationError), // Something went wrong trying to generate indices for the quad
}

impl std::fmt::Display for GraphicsCreationError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			GraphicsCreationError::Io(error) => {
				write!(f, "(IO)");
				error.fmt(f)
			}
			GraphicsCreationError::Program(error) => {
				write!(f, "(Program)");
				error.fmt(f)
			}
			GraphicsCreationError::VertexBuffer(error) => {
				write!(f, "(VertexBuffer)");
				error.fmt(f)
			}
			GraphicsCreationError::IndexBuffer(error) => {
				write!(f, "(IndexBuffer)");
				error.fmt(f)
			}
		}
	}
}

impl std::error::Error for GraphicsCreationError {
	fn description(&self) -> &str {
		"Failed to create a Graphics object."
	}

	fn cause(&self) -> Option<&std::error::Error> {
		match self {
			GraphicsCreationError::Io(error) => Some(error),
			GraphicsCreationError::Program(error) => Some(error),
			GraphicsCreationError::VertexBuffer(error) => Some(error),
			GraphicsCreationError::IndexBuffer(error) => Some(error),
		}
	}
}

impl From<IoError> for GraphicsCreationError {
	fn from(error: IoError) -> Self {
		GraphicsCreationError::Io(error)
	}
}

impl From<ProgramCreationError> for GraphicsCreationError {
	fn from(error: ProgramCreationError) -> Self {
		GraphicsCreationError::Program(error)
	}
}

impl From<VertexBufferCreationError> for GraphicsCreationError {
	fn from(error: VertexBufferCreationError) -> Self {
		GraphicsCreationError::VertexBuffer(error)
	}
}

impl From<IndexBufferCreationError> for GraphicsCreationError {
	fn from(error: IndexBufferCreationError) -> Self {
		GraphicsCreationError::IndexBuffer(error)
	}
}

fn load_font<'a>(name: &String) -> Result<rusttype::Font<'a>, Box<std::error::Error>> {
	let path = String::from(FONT_PREFIX) + name;
	let path = Path::new(&path);

	let file = std::fs::read(path)?;
	let font = rusttype::Font::from_bytes(file)?;

	Ok(font)
}

fn load_program<F>(
	facade: &F,
	vertex_shader_name: &String,
	fragment_shader_name: &String,
) -> Result<Program, GraphicsCreationError>
where
	F: glium::backend::Facade,
{
	let path = String::from(SHADER_PREFIX) + vertex_shader_name;
	let path = Path::new(&path);

	let vertex_shader = std::fs::read_to_string(path)?;

	let path = String::from(SHADER_PREFIX) + fragment_shader_name;
	let path = Path::new(&path);

	let fragment_shader = std::fs::read_to_string(path)?;

	let program = Program::from_source(facade, &vertex_shader, &fragment_shader, None)?;
	Ok(program)
}

fn generate_quad<F>(
	facade: &F,
) -> Result<(VertexBuffer<Vertex>, IndexBuffer<u16>), GraphicsCreationError>
where
	F: glium::backend::Facade,
{
	let vertex_buffer = VertexBuffer::new(
		facade,
		&[
			Vertex {
				position: [-0.5, 0.5],
				tex_coords: [0.0, 1.0],
			},
			Vertex {
				position: [-0.5, -0.5],
				tex_coords: [0.0, 0.0],
			},
			Vertex {
				position: [0.5, 0.5],
				tex_coords: [1.0, 1.0],
			},
			Vertex {
				position: [0.5, -0.5],
				tex_coords: [1.0, 0.0],
			},
		],
	)?;
	let index_buffer = IndexBuffer::new(
		facade,
		glium::index::PrimitiveType::TrianglesList,
		&[0, 1, 2, 1, 2, 3],
	)?;

	Ok((vertex_buffer, index_buffer))
}

fn generate_instances<F: glium::backend::Facade>(facade: &F) -> Result<VertexBuffer<PerInstance>, VertexBufferCreationError> {
	let mut instances: Vec<PerInstance> = Vec::with_capacity(INSTANCE_COUNT);

	let x_step = 2.0 / INSTANCE_COLUMNS as f32;
	let y_step = 2.0 / INSTANCE_ROWS as f32;

	for x in 1..INSTANCE_COLUMNS {
		for y in 1..INSTANCE_ROWS {
			instances.push(PerInstance {
				i_translation: [-1.0 + x_step * x as f32, -1.0 + y_step * y as f32],
				i_z_theta: [0.5, 0.0],
				i_scale: [x_step, x_step],
				i_color: [1.0, 1.0, 1.0, 1.0],
			});
		}
	}

	VertexBuffer::dynamic(facade, &instances)
}

fn short_angle_distance(a: f32, b: f32) -> f32 {
	let max = PI * 2.0;
	let delta = (b - a) % max;

	(2.0 * delta) % max - delta
}

pub fn lerp_angle(a: f32, b: f32, t: f32) -> f32 {
	a + short_angle_distance(a, b) * t
}

#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
	a * (1.0 - t) + b * t
}

#[inline]
pub fn lerp_vec2(a: &[f32; 2], b: &[f32; 2], t: f32) -> [f32; 2] {
	[lerp(a[0], b[0], t), lerp(a[1], b[1], t)]
}

#[inline]
pub fn lerp_vec4(a: &[f32; 4], b: &[f32; 4], t: f32) -> [f32; 4] {
	[
		lerp(a[0], b[0], t),
		lerp(a[1], b[1], t),
		lerp(a[2], b[2], t),
		lerp(a[3], b[3], t),
	]
}

// A PerInstance structure that used similar to a uniform
#[derive(Copy, Clone, Default)]
struct PerInstance {
	i_translation: [f32; 2],	// Translation in world space
	i_z_theta: [f32; 2],		// Z-order and angle of rotation around origin in radians
	i_scale: [f32; 2],			// Scaling of the object in world space
	i_color: [f32; 4],			// The color of the object
}
implement_vertex!(PerInstance, i_translation, i_z_theta, i_scale, i_color);