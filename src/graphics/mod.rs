// Module that encopases all accelerated graphical presentation.

use {FONT_PREFIX, SHADER_PREFIX};
use config::Configuration;

use glium::{Display, IndexBuffer, VertexBuffer, Surface};
use glium::index::BufferCreationError as IndexBufferCreationError;
use glium::vertex::BufferCreationError as VertexBufferCreationError;

use glium::program::{Program, ProgramCreationError};

use std::path::Path;
use std::io::Error as IoError;

#[derive(Copy, Clone)]
pub struct Vertex {
	position: [f32; 2],
//	tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position); //, tex_coords);


// Graphical context
//
// The workhorse of this module. Abstracts away specifics of underlying 3D library calls for the purposes of the project.
pub struct Graphics {
	display: Display,	// takes ownership of the glium display object
	program: Program,	// shaders used to draw objects
	// more will be created later

	// temporary variables for testing
	verts: VertexBuffer<Vertex>,
	indcs: IndexBuffer<u16>,
}

impl Graphics {
	pub fn new(display: Display, config: &Configuration) -> Result<Self, GraphicsCreationError> {
		let program = load_program(&display, &config.vert_shader, &config.frag_shader)?;

		let (verts, indcs) = generate_quad(&display)?;

		Ok(Graphics {display: display, program: program, verts: verts, indcs: indcs})
	}

	pub fn draw(&self) {
		let mut width_to_height = 1.0f32;
		if let Some(size) = self.display.gl_window().get_inner_size() {
			width_to_height = size.width as f32 / size.height as f32;
		}

		let uniforms = uniform! {
			u_translate: [0.0, 0.0f32],
			u_z_theta: [0.0, 0.0f32],
			u_scale: [1.0, width_to_height],

			u_color: [1.0, 1.0, 0.0, 1.0f32],
		};

		let mut target = self.display.draw();
		target.clear_color(0.0, 0.0, 1.0, 1.0);
		target.draw(&self.verts, &self.indcs, &self.program, &uniforms, &Default::default()).unwrap();
		target.finish().unwrap();
	}

	pub fn window(&self) -> core::cell::Ref<glium::glutin::GlWindow> {
		self.display.gl_window()
	}
}


#[derive(Debug)]
pub enum GraphicsCreationError {
	Io(IoError),					// Something went wrong trying to load shader files
	Program(ProgramCreationError),	// Something went wrong trying to compile shaders
	VertexBuffer(VertexBufferCreationError),	// Something went wrong trying to generate vertices for the quad
	IndexBuffer(IndexBufferCreationError),		// Something went wrong trying to generate indices for the quad
}

impl std::fmt::Display for GraphicsCreationError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			GraphicsCreationError::Io(error) => { write!(f, "(IO)"); error.fmt(f) }
			GraphicsCreationError::Program(error) => { write!(f, "(Program)"); error.fmt(f) }
			GraphicsCreationError::VertexBuffer(error) => { write!(f, "(VertexBuffer)"); error.fmt(f) }
			GraphicsCreationError::IndexBuffer(error) => { write!(f, "(IndexBuffer)"); error.fmt(f) }
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
			Vertex { position: [-0.5,  0.5] },
			Vertex { position: [-0.5, -0.5] },
			Vertex { position: [ 0.5,  0.5] },
			Vertex { position: [ 0.5, -0.5] },
//			Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0], },
//			Vertex { position: [-0.5,  0.5], tex_coords: [0.0, 1.0], },
//			Vertex { position: [ 0.5,  0.5], tex_coords: [1.0, 1.0], },
//			Vertex { position: [ 0.5, -0.5], tex_coords: [1.0, 0.0], },
		],
	)?;
	let index_buffer = IndexBuffer::new(
		facade,
		glium::index::PrimitiveType::TrianglesList,
		&[0, 1, 2,  1, 2, 3]
	)?;

	Ok((vertex_buffer, index_buffer))
}
