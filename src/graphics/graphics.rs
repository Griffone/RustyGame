// Graphical context handling

use config::Configuration;
use SHADER_PREFIX;

use super::instance::PerInstance;
use super::scene::Scene;
use super::{FRAGMENT_SHADER_EXTENSHION, INSTANCED_SHADER, VERTEX_SHADER_EXTENSHION};

use glium::index::BufferCreationError as IndexBufferCreationError;
use glium::program::ProgramCreationError;
use glium::vertex::BufferCreationError as VertexBufferCreationError;
use glium::{Display, IndexBuffer, Program, Surface, VertexBuffer};

use std::io::Error as IoError;
use std::path::Path;

// Graphical context
pub struct Graphics {
	pub display: Display, // takes ownership of the glium display object
	program: Program, // shaders used to draw objects

	quad_vertices: VertexBuffer<Vertex>,
	quad_indices: IndexBuffer<u16>,

	instance_buffer: VertexBuffer<PerInstance>,
	batch_size: usize,
}

impl Graphics {
	pub fn new(display: Display, config: &Configuration) -> Result<Self, GraphicsCreationError> {
		let program = load_program(&display, &String::from(INSTANCED_SHADER))?;

		let (verts, indcs) = generate_quad(&display)?;

		let batch_size = config.batch_size;
		let instances = generate_instance_buffer(&display, batch_size)?;

		Ok(Graphics {
			display: display,
			program: program,
			quad_vertices: verts,
			quad_indices: indcs,
			instance_buffer: instances,
			batch_size: batch_size,
		})
	}

	pub fn draw<T: Scene>(&mut self, scene: &T) {
		let params = glium::DrawParameters {
			depth: glium::Depth {
				test: glium::DepthTest::IfLess,
				write: true,
				..Default::default()
			},
			blend: glium::Blend::alpha_blending(),
			..Default::default()
		};

		// Preserve aspect ratio of the world-space
		let mut width_to_height = 1.0f32;
		if let Some(size) = self.display.gl_window().get_inner_size() {
			width_to_height = size.width as f32 / size.height as f32;
		}

		let view_rect = scene.view_rect();

		let scale = if scene.preserve_ratio() {
			// Calculate the necessary scaling
			let mut scaling = 2.0 / view_rect.width();
			let y_scaling = 2.0 / width_to_height / view_rect.height();

			if scaling > y_scaling {
				scaling = y_scaling;
			}

			[scaling, width_to_height * scaling]
		} else {
			[2.0 / view_rect.width(), 2.0 / view_rect.height()]
		};

		let uniforms = uniform! {
			u_scale: scale,
			u_translation: view_rect.center(),

			u_view_origin: scene.view_origin(),
			u_view_distance: scene.view_distance(),

			u_texture: scene.texture(),
		};

		let mut target = self.display.draw();
		target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

		// Pass 0: objects
		{
			for chunk in scene.object_instances().chunks(self.batch_size) {
				{
					let mut mapping = self.instance_buffer.map();
					for (object, instance) in chunk.iter().zip(mapping.iter_mut()) {
						*instance = object.clone().into();
					}
				}
				target
					.draw(
						(
							&self.quad_vertices,
							self.instance_buffer
								.slice(..chunk.len())
								.unwrap()
								.per_instance()
								.unwrap(),
						),
						&self.quad_indices,
						&self.program,
						&uniforms,
						&params,
					).unwrap();
			}
		}

		target.finish().unwrap();
	}

	pub fn window(&self) -> core::cell::Ref<glium::glutin::GlWindow> {
		self.display.gl_window()
	}

	pub fn screen_to_world<T: Scene>(&self, normalized_screen: super::math::Point, scene: &T) -> super::math::Point {
		let mut width_to_height = 1.0f32;
		if let Some(size) = self.display.gl_window().get_inner_size() {
			width_to_height = size.width as f32 / size.height as f32;
		}
		let view_rect = scene.view_rect();
		let scale = if scene.preserve_ratio() {
			// Calculate the necessary scaling
			let mut scaling = 1.0 / view_rect.width();
			let y_scaling = 1.0 / width_to_height / view_rect.height();

			if scaling > y_scaling {
				scaling = y_scaling;
			}

			[scaling, width_to_height * scaling]
		} else {
			[1.0 / view_rect.width(), 1.0 / view_rect.height()]
		};

		[normalized_screen[0] / scale[0] + view_rect.min_x(), (1.0 - normalized_screen[1]) / scale[1] + view_rect.min_y()]
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

#[derive(Copy, Clone)]
pub struct Vertex {
	position: [f32; 2],
	tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

fn load_program<F>(facade: &F, shader_name: &str) -> Result<Program, GraphicsCreationError>
where
	F: glium::backend::Facade,
{
	let mut path = String::from(SHADER_PREFIX);
	path.push_str(shader_name);
	path.push_str(VERTEX_SHADER_EXTENSHION);
	let path = Path::new(&path);

	let vertex_shader = std::fs::read_to_string(path)?;

	let mut path = String::from(SHADER_PREFIX);
	path.push_str(shader_name);
	path.push_str(FRAGMENT_SHADER_EXTENSHION);
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

fn generate_instance_buffer<F: glium::backend::Facade>(
	facade: &F,
	size: usize,
) -> Result<VertexBuffer<PerInstance>, VertexBufferCreationError> {
	assert!(size > 0, "batch size should be at least 1!");

	let mut instances: Vec<PerInstance> = Vec::with_capacity(size);
	for _ in 0..size {
		instances.push(Default::default());
	}

	VertexBuffer::dynamic(facade, &instances)
}
