// Configuration file save-and-loading utility.

// A single structure that is able to hold all necessary configurations
// All variables have default, so that user can easily reset single setting by just deleting them
#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
	#[serde(default)]
	pub fullscreen: bool,
	//#[serde(default)]
	//pub fullscreen_monitor: u32,

	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub window_position: Option<(f64, f64)>,

	#[serde(default)]
	pub debug_mode: bool,


	#[serde(skip)]
	pub changed: bool,
}



impl Configuration {
	// Create a new defaulted Configuration
	pub fn default() -> Self {
		Self {
			fullscreen: false,
			//fullscreen_monitor: 0,
			window_position: None,
			debug_mode: false,
			changed: false,
		}
	}

	pub fn load_or_default<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Box<std::error::Error>> {
		match std::fs::File::open(path) {
			Ok(file) => {
				let config = serde_yaml::from_reader(file)?;

				Ok(config)
			},
			Err(error) => match error.kind() {
				std::io::ErrorKind::NotFound => Ok(Self{changed: true, ..Self::default()}),
				_ => Err(Box::new(error))
			}
		}
	}

	pub fn set_fullscreen(&mut self, enabled: bool) {
		self.changed = self.changed || self.fullscreen != enabled;
		self.fullscreen = enabled;
	}

	pub fn set_debug_mode(&mut self, enabled: bool) {
		self.changed = self.changed || self.debug_mode != enabled;
		self.debug_mode = enabled;
	}

	pub fn set_window_position(&mut self, window_position: (f64, f64)) {
		self.changed = self.changed || self.window_position != Some(window_position);
		self.window_position = Some(window_position);
	}

	pub fn reset_window_position(&mut self) {
		self.changed = self.changed || self.window_position != None;
		self.window_position = None;
	}

	pub fn save_as<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), Box<std::error::Error>> {
		if self.changed {
			let string = serde_yaml::to_string(self)?;

			std::fs::write(path, string)?;
		}
		
		Ok(())
	}
}