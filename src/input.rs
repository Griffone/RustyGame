// Input handler module
pub use glium::glutin::ScanCode;

use std::collections::HashMap as Map;

/// A bit-field for key modifiers
/// 
/// Use MODIFIER_* for individual values
pub type KeyModifiers = u8;

// Modifier masks
pub const MODIFIER_NONE: KeyModifiers = 0x0;
pub const MODIFIER_SHIFT: KeyModifiers = 0x1;
pub const MODIFIER_ALT: KeyModifiers = 0x2;
pub const MODIFIER_CTRL: KeyModifiers = 0x4;
pub const MODIFIER_LOGO: KeyModifiers = 0x8;

// Ordinary scancodes (will get added as necessary)
pub const SCANCODE_F11: ScanCode = 0x57;

/// An action that can be caused by input
#[derive(Clone, Copy, Debug)]
pub enum Action {
	None,

	ToggleFullscreen,
}

/// A key identifier
/// 
/// Unique modifier-scancode pair,
/// The game uses scancodes to be language agnostic.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Key {
	pub modifiers: KeyModifiers,
	pub scancode: ScanCode,
}

/// Info that is stored in Input class
struct KeyInfo {
	pressed: bool,
	on_down: Action,
	on_up: Action,
}

/// Holds input context
/// 
/// Does minor pre-processing of input, operates on scenes and stuff.
pub struct Input {
	keys: Map<Key, KeyInfo>,

	pub call_callbacks: bool,
}

impl<'a> From<&'a glium::glutin::KeyboardInput> for Key {
	fn from(event: &glium::glutin::KeyboardInput) -> Self {
		let mut modifiers: KeyModifiers = 0;
		if event.modifiers.shift {
			modifiers |= MODIFIER_SHIFT;
		}
		if event.modifiers.ctrl {
			modifiers |= MODIFIER_CTRL;
		}
		if event.modifiers.alt {
			modifiers |= MODIFIER_ALT;
		}
		if event.modifiers.logo {
			modifiers |= MODIFIER_LOGO;
		}
		Self { scancode: event.scancode, modifiers }
	}
}

pub trait KeyModifier {
	fn shift(&self) -> bool;
	fn alt(&self) -> bool;
	fn ctrl(&self) -> bool;
	fn logo(&self) -> bool;
}

impl KeyModifier for KeyModifiers {
	fn shift(&self) -> bool {
		self & MODIFIER_SHIFT == MODIFIER_SHIFT
	}

	fn alt(&self) -> bool {
		self & MODIFIER_ALT == MODIFIER_ALT
	}

	fn ctrl(&self) -> bool {
		self & MODIFIER_CTRL == MODIFIER_CTRL
	}

	fn logo(&self) -> bool {
		self & MODIFIER_LOGO == MODIFIER_LOGO
	}
}

impl Default for Input {
	fn default() -> Self {
		Self { call_callbacks: false, keys: Map::new()}
	}
}

impl Input {
	pub fn load_from_file(path: &std::path::Path) -> Self {
		panic!("Input loading not implemented!");
	}

	pub fn set_on_down(&mut self, key: Key, action: Action) {
		// FIXME: cleanup once rust compiler realizes self.keys are not borrowed for the whole scope...
		let mut found = false;
		{
			if let Some(info) = self.keys.get_mut(&key) {
				info.on_down = action;
				found = true;
			}
		}
		if !found {
			self.keys.insert(key, KeyInfo {pressed: false, on_down: action, on_up: Action::None});
		}
	}

	pub fn set_on_up(&mut self, key: Key, action: Action) {
		// FIXME: cleanup once rust compiler realizes self.keys are not borrowed for the whole scope...
		let mut found = false;
		{
			if let Some(info) = self.keys.get_mut(&key) {
				info.on_up = action;
				found = true;
			}
		}
		if !found {
			self.keys.insert(key, KeyInfo {pressed: false, on_down: Action::None, on_up: action});
		}
	}

	pub fn clear(&mut self, key: Key) {
		self.keys.remove(&key);
	}

	pub fn process_key(&mut self, event: &glium::glutin::KeyboardInput) -> Action {
		let key = event.into();
		let mut action = Action::None;

		if let Some(info) = self.keys.get_mut(&key) {
			let pressed;
			match event.state {
				glium::glutin::ElementState::Pressed => pressed = true,
				glium::glutin::ElementState::Released => pressed = false,
			}
			if pressed != info.pressed {
				if pressed {
					action = info.on_down;
				} else {
					action = info.on_up;
				}
				info.pressed = pressed;
			}
		}
		
		action
	}
}

fn do_nothing(_: &Key) {}