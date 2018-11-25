// Module that encopases all accelerated graphical presentation.

pub use graphics::graphics::Graphics;	// Huh?

//mod texture;
pub mod graphics;	// Graphical context, core module
pub mod math;		// Helper functions
pub mod instance;	// A drawable object instance
pub mod transform;	// Transformation of a drawable instance
pub mod scene;		// A renderable scene

pub const INSTANCED_SHADER: &str = "instanced";
pub const VERTEX_SHADER_EXTENSHION: &str = ".vert";
pub const FRAGMENT_SHADER_EXTENSHION: &str = ".frag";