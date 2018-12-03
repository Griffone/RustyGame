// Texture handling for drawn objects
//
// Mildly complicated by the need to combine different textures into single draw call

use TEXTURE_PREFIX;

use super::Graphics;
use super::math::Rect;

use image::{ImageError, DynamicImage, GenericImage};

use glium::texture::TextureCreationError;
pub use glium::texture::CompressedSrgbTexture2d as GLTexture;
use glium::texture::CompressedSrgbFormat as GLTextureFormat;

use std::io::Error as IoError;
use std::path::Path;
use std::collections::HashMap as Map;

pub type TextureID = String;

const DEFAULT_FORMAT: GLTextureFormat = GLTextureFormat::S3tcDxt1Alpha;
const USED_MIPMAP: glium::texture::CompressedMipmapsOption = glium::texture::CompressedMipmapsOption::NoMipmap;

// A single texture
#[derive(Copy, Clone, Debug)]
pub struct Texture {
	pub area: Rect,
}

/// A collection of multiple textures.
/// 
/// Internally stored as an atlas to enable instancing with different textures from the same collection.
#[derive(Debug)]
pub struct TextureCollection {
	textures: Map<TextureID, Texture>,
	pub texture: GLTexture,
}

#[derive(Debug)]
pub enum TextureCollectionCreationError {
	Io(IoError),       // Something went wrong trying to load a texture file
	Image(ImageError), // Something went wrong trying to load image
	Texture(TextureCreationError),	// Failed to generate a texture array or upload it to the GPU
}

impl From<IoError> for TextureCollectionCreationError {
	fn from(error: IoError) -> Self {
		TextureCollectionCreationError::Io(error)
	}
}

impl From<ImageError> for TextureCollectionCreationError {
	fn from(error: ImageError) -> Self {
		// Don't nest IoErrors, unwrap one here
		if let ImageError::IoError(error) = error {
			TextureCollectionCreationError::Io(error)
		} else {
			TextureCollectionCreationError::Image(error)
		}
	}
}

impl From<TextureCreationError> for TextureCollectionCreationError {
	fn from(error: TextureCreationError) -> Self {
		TextureCollectionCreationError::Texture(error)
	}
}

impl TextureCollection {
	// Attempt to generate a TextureCollection from a vector of filenames to load from.
	// Eager to fail, meaning failing at any point will return Err
	pub fn new(
		graphics: &Graphics,
		texture_filenames: &Vec<&str>,
	) -> Result<TextureCollection, TextureCollectionCreationError>
	{
		let expected_length = texture_filenames.len();
		let mut images = Vec::with_capacity(expected_length);
		let mut width = 0;
		let mut height = 0;

		for name in texture_filenames {
			let path = String::from(TEXTURE_PREFIX) + name;
			let path = Path::new(&path);

			let image = image::open(path)?;
			let image = image.to_rgba();
			let (image_width, image_height) = image.dimensions();

			width += image_width;
			if image_height > height {
				height = image_height;
			}
			images.push(image);
		}

		let mut pos_x = 0;
		let mut texture = DynamicImage::new_rgba8(width, height);
		let width = width as f32;
		let height = height as f32;

		let mut textures = Map::with_capacity(expected_length);

		for (name, image) in texture_filenames.iter().zip(images.iter()) {
			let (image_width, image_height) = image.dimensions();
			let rect = Rect::new([pos_x as f32 / width, 1.0 - image_height as f32 / height], [(pos_x + image_width) as f32 / width, 1.0]);
			textures.insert(String::from(*name), Texture {area: rect});
			texture.copy_from(image, pos_x, 0);
			pos_x += image_width;
		}

		let texture = texture.to_rgba();
		let texture_dimensions = texture.dimensions();
		let texture = glium::texture::RawImage2d::from_raw_rgba_reversed(&texture, texture_dimensions);
		let texture = GLTexture::new(&graphics.display, texture)?;

		Ok(TextureCollection {textures: textures, texture: texture})
	}

	pub fn get(&self, id: &TextureID) -> Option<Texture> {
		match self.textures.get(id) {
			Some(texture) => Some(texture.clone()),
			None => None
		}
	}
}