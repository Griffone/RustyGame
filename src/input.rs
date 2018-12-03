// Input handler module
pub use glium::glutin::ScanCode;

use std::collections::HashMap as Map;

/// A bit-field for key modifiers
///
/// Use MODIFIER_* for individual values
pub type KeyModifiers = u8;

pub type MouseButton = glium::glutin::MouseButton;

// Modifier masks
pub const MODIFIER_NONE: KeyModifiers = 0x0;
pub const MODIFIER_SHIFT: KeyModifiers = 0x1;
pub const MODIFIER_ALT: KeyModifiers = 0x2;
pub const MODIFIER_CTRL: KeyModifiers = 0x4;
pub const MODIFIER_LOGO: KeyModifiers = 0x8;

// Ordinary scancodes (will get added as necessary)
pub const SCANCODE_F11: ScanCode = 0x57;

/// An action identifier that can be caused by Input
#[derive(Clone, Copy, Debug)]
pub enum Action {
	None, // Action invariant, no action actually needs to be performed

	ToggleFullscreen,
}

/// Action executed on mouse wheel movement
///
/// These are different from a simple Action as they have a corresponding wheel delta value
#[derive(Clone, Copy, Debug)]
pub enum WheelAction {
	None,

	ChangeViewSharpness,
	ChangeSceneSize,
	ChangeViewSize,
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

/// A button identifier
///
/// Similar to a key, but a mouse button instead
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Button {
	pub modifiers: KeyModifiers,
	pub button: MouseButton,
}

struct ButtonInfo {
	pressed: bool,
	on_down: Action,
	on_up: Action,
}

/// Holds input context
///
/// Does minor pre-processing of input, operates on scenes and stuff.
pub struct Input {
	keys: Map<Key, KeyInfo>,
	buttons: Map<Button, ButtonInfo>,
	wheel_deltas: Map<KeyModifiers, WheelAction>,

	mouse_position: (f32, f32),
	mouse_wheel: f32,
	viewport_size: (f32, f32),
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
		Self {
			scancode: event.scancode,
			modifiers,
		}
	}
}

impl Default for KeyInfo {
	fn default() -> Self {
		Self {
			pressed: false,
			on_up: Action::None,
			on_down: Action::None,
		}
	}
}

impl Default for ButtonInfo {
	fn default() -> Self {
		Self {
			pressed: false,
			on_up: Action::None,
			on_down: Action::None,
		}
	}
}

pub trait KeyModifier {
	fn shift(&self) -> bool;
	fn alt(&self) -> bool;
	fn ctrl(&self) -> bool;
	fn logo(&self) -> bool;

	fn from_state(&glium::glutin::ModifiersState) -> Self;
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

	fn from_state(state: &glium::glutin::ModifiersState) -> Self {
		let mut value = 0;
		if state.shift {
			value |= MODIFIER_SHIFT;
		};
		if state.ctrl {
			value |= MODIFIER_CTRL;
		};
		if state.alt {
			value |= MODIFIER_ALT;
		};
		if state.logo {
			value |= MODIFIER_LOGO;
		};
		value
	}
}

impl Default for Input {
	fn default() -> Self {
		Self {
			keys: Map::new(),
			buttons: Map::new(),
			wheel_deltas: Map::new(),

			mouse_position: (0.0, 0.0),
			mouse_wheel: 0.0,
			viewport_size: (0.0, 0.0),
		}
	}
}

impl Input {
	pub fn load_from_file(path: &std::path::Path) -> Self {
		panic!("Input loading not implemented!");
	}

	pub fn with_default_actions() -> Self {
		let mut input = Self::default();

		input.set_on_key_up(
			Key {
				scancode: SCANCODE_F11,
				modifiers: MODIFIER_NONE,
			},
			Action::ToggleFullscreen,
		);

		input.set_on_wheel_delta(MODIFIER_NONE, WheelAction::ChangeViewSize);
		input.set_on_wheel_delta(MODIFIER_SHIFT, WheelAction::ChangeViewSharpness);
		input.set_on_wheel_delta(MODIFIER_ALT, WheelAction::ChangeSceneSize);

		input
	}

	pub fn set_on_key_down(&mut self, key: Key, action: Action) {
		if self.keys.contains_key(&key) {
			if let Some(info) = self.keys.get_mut(&key) {
				info.on_down = action;
			}
		} else {
			self.keys.insert(
				key,
				KeyInfo {
					on_down: action,
					..KeyInfo::default()
				},
			);
		}
	}

	pub fn set_on_button_down(&mut self, button: Button, action: Action) {
		if self.buttons.contains_key(&button) {
			if let Some(info) = self.buttons.get_mut(&button) {
				info.on_down = action;
			}
		} else {
			self.buttons.insert(
				button,
				ButtonInfo {
					on_down: action,
					..ButtonInfo::default()
				},
			);
		}
	}

	pub fn set_on_key_up(&mut self, key: Key, action: Action) {
		if self.keys.contains_key(&key) {
			if let Some(info) = self.keys.get_mut(&key) {
				info.on_up = action;
			}
		} else {
			self.keys.insert(
				key,
				KeyInfo {
					on_up: action,
					..KeyInfo::default()
				},
			);
		}
	}

	pub fn set_on_button_up(&mut self, button: Button, action: Action) {
		if self.buttons.contains_key(&button) {
			if let Some(info) = self.buttons.get_mut(&button) {
				info.on_up = action;
			}
		} else {
			self.buttons.insert(
				button,
				ButtonInfo {
					on_up: action,
					..ButtonInfo::default()
				},
			);
		}
	}

	pub fn set_on_wheel_delta(&mut self, modifiers: KeyModifiers, action: WheelAction) {
		self.wheel_deltas.insert(modifiers, action);
	}

	pub fn clear_key(&mut self, key: Key) {
		self.keys.remove(&key);
	}

	pub fn clear_button(&mut self, button: Button) {
		self.buttons.remove(&button);
	}

	pub fn clear_wheel_delta(&mut self, modifiers: KeyModifiers) {
		self.wheel_deltas.remove(&modifiers);
	}

	/// Process a KeyboardInput event
	///
	/// Will return corresponding Some(Action) action identifier if one needs to be processed
	pub fn process_key(&mut self, event: &glium::glutin::KeyboardInput) -> Option<Action> {
		let key = event.into();

		if let Some(info) = self.keys.get_mut(&key) {
			let pressed;
			match event.state {
				glium::glutin::ElementState::Pressed => pressed = true,
				glium::glutin::ElementState::Released => pressed = false,
			}
			if pressed != info.pressed {
				info.pressed = pressed;
				if pressed {
					return Some(info.on_down);
				} else {
					return Some(info.on_up);
				}
			}
		}

		None
	}

	pub fn process_button(
		&mut self,
		state: &glium::glutin::ElementState,
		button: &MouseButton,
		modifiers: &glium::glutin::ModifiersState,
	) -> Option<Action> {
		let button = Button {
			modifiers: KeyModifiers::from_state(modifiers),
			button: button.clone(),
		};

		let mut action = Action::None;

		if let Some(info) = self.buttons.get_mut(&button) {
			let pressed;
			match state {
				glium::glutin::ElementState::Pressed => pressed = true,
				glium::glutin::ElementState::Released => pressed = false,
			}
			if pressed != info.pressed {
				info.pressed = pressed;
				if pressed {
					action = info.on_down;
				} else {
					action = info.on_up;
				}
			}
		}

		if let Action::None = action {
			None
		} else {
			Some(action)
		}
	}

	pub fn process_mouse_wheel(
		&mut self,
		modifiers: &glium::glutin::ModifiersState,
	) -> Option<WheelAction> {
		let modifiers = KeyModifiers::from_state(modifiers);

		let mut action = WheelAction::None;

		if let Some(wheel_action) = self.wheel_deltas.get(&modifiers) {
			action = wheel_action.clone();
		}

		if let WheelAction::None = action {
			None
		} else {
			Some(action)
		}
	}

	pub fn update_mouse_position(&mut self, mouse_position: (f32, f32)) {
		self.mouse_position = mouse_position;
	}

	pub fn update_viewport_size(&mut self, viewport_size: (f32, f32)) {
		self.viewport_size = viewport_size;
	}

	/// Get mouse position in window space
	pub fn absolute_mouse_position(&self) -> (f32, f32) {
		self.mouse_position
	}

	/// Get normalized mouse coordinates in window-space
	pub fn relative_mouse_position(&self) -> [f32; 2] {
		[
			self.mouse_position.0 / self.viewport_size.0,
			self.mouse_position.1 / self.viewport_size.1,
		]
	}

	/// Get mouse wheel coordinate
	pub fn mouse_wheel(&self) -> f32 {
		self.mouse_wheel
	}
}
