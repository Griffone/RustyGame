// Module that encopases all accelerated graphical presentation.

pub use self::graphics::Graphics;
pub use self::texture::TextureCollection;

pub mod graphics;	// Graphical context, core module
pub mod math;		// Helper functions
pub mod texture;	// Smart texture wrapping above glium to allow instancing with different textures
pub mod instance;	// A drawable object instance
pub mod transform;	// Transformation of a drawable instance
pub mod scene;		// A renderable scene

pub const INSTANCED_SHADER: &str = "instanced";
pub const VERTEX_SHADER_EXTENSHION: &str = ".vert";
pub const FRAGMENT_SHADER_EXTENSHION: &str = ".frag";