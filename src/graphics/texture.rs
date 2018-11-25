// Texture handling for drawn objects
//
// Mildly complicated by the need to combine different textures into single draw call

use TEXTURE_PREFIX;

use super::Graphics;

use image::ImageError;
use glium::texture::TextureCreationError;

use std::io::Error as IoError;
use std::path::Path;

// A single texture
pub struct Texture {}

// A collection of multiple textures.
// Currently is a fancy wrapper for an ArrayTexture
pub struct TextureCollection {}

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

impl TextureCollection {
	// Attempt to generate a TextureCollection from a vector of filenames to load from.
	// Eager to fail, meaning failing at any point will return Err
	pub fn new<F>(
		facade: F,
		texture_filenames: Vec<String>,
	) -> Result<TextureCollection, TextureCollectionCreationError>
	where
		F: glium::backend::Facade,
	{
		// First texture will define parameters for the whole collection
		let path = String::from(TEXTURE_PREFIX) + &texture_filenames[0];
		let path = Path::new(&path);

		let image = image::open(path)?;
		let image = image.to_rgba();
		let dimensions = image.dimensions();



		Ok(TextureCollection {})
	}
}
