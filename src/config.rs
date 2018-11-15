// Configuration file save-and-loading utility.

// A single structure that is able to hold all necessary configurations
#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
	#[serde(default)]
	pub fullscreen_enabled: bool,
	//#[serde(default)]
	//pub fullscreen_monitor: u32,

	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub window_position: Option<(i32, i32)>,

	#[serde(default)]
	pub debug_enabled: bool,


	#[serde(skip)]
	pub changed: bool,
}



impl Configuration {
	// Create a new defaulted Configuration
	pub fn default() -> Self {
		Self {
			fullscreen_enabled: false,
			//fullscreen_monitor: 0,
			window_position: None,
			debug_enabled: false,
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

	pub fn set_fullscreen_enabled(&mut self, enable: bool) {
		self.changed = self.changed || self.fullscreen_enabled != enable;
		println!("Config is changed: {}", self.changed);
		self.fullscreen_enabled = enable;
	}

	pub fn set_debug_enabled(&mut self, enable: bool) {
		self.changed = self.changed || self.debug_enabled != enable;
		self.debug_enabled = enable;
	}

	pub fn set_window_position(&mut self, window_position: (i32, i32)) {
		self.changed = self.changed || self.window_position != Some(window_position);
		self.window_position = Some(window_position);
	}

	pub fn save_as<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), Box<std::error::Error>> {
		if self.changed {
			let string = serde_yaml::to_string(self)?;

			std::fs::write(path, string)?;
		}
		
		Ok(())
	}
}