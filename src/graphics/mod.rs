// Module that encopases all accelerated graphical presentation.

use glium::program::Program;
use glium::{IndexBuffer, VertexBuffer};
use std::path::Path;
use {FONT_PREFIX, SHADER_PREFIX};

#[derive(Copy, Clone)]
pub struct Vertex {
	position: [f32; 2],
//	tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position); //, tex_coords);

pub fn load_font<'a>(name: &String) -> Result<rusttype::Font<'a>, Box<std::error::Error>> {
	let path = String::from(FONT_PREFIX) + name;
	let path = Path::new(&path);

	let file = std::fs::read(path)?;
	let font = rusttype::Font::from_bytes(file)?;

	Ok(font)
}

pub fn load_program<F>(
	facade: &F,
	vertex_shader_name: &String,
	fragment_shader_name: &String,
) -> Result<Program, Box<std::error::Error>>
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

pub fn generate_quad<F>(
	facade: &F,
) -> Result<(VertexBuffer<Vertex>, IndexBuffer<u16>), Box<std::error::Error>>
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
