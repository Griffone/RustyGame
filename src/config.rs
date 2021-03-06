// Configuration file save-and-loading utility.

// A single structure that is able to hold all necessary configurations
// All variables have default, so that user can easily reset single setting by just deleting them
#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
	#[serde(default)]
	pub fullscreen: bool,

	#[serde(default)]
	pub vsync: bool,

	#[serde(default)]
	pub font: String,
	#[serde(default = "default_batch_size")]
	pub batch_size: usize,

	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub window_position: Option<(f64, f64)>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub window_size: Option<(f64, f64)>,

	#[serde(default)]
	pub debug_mode: bool,

	#[serde(skip)]
	pub changed: bool,
}

fn default_batch_size() -> usize {
	1024
}

impl Default for Configuration {
	// Create a new defaulted Configuration
	fn default() -> Self {
		Self {
			fullscreen: false,
			vsync: true,
			font: String::from("arimo.ttf"),
			batch_size: default_batch_size(),
			window_position: None,
			window_size: None,
			debug_mode: false,
			changed: true,
		}
	}
}

impl Configuration {
	pub fn load_or_default(path: &std::path::Path) -> Self {
		match std::fs::File::open(path) {
			Ok(file) => {
				match serde_yaml::from_reader(file) {
					Ok(config) => return config, // please note, "changed" field will be false, as thats the defaut bool behavior, but this is at all not obvious
					Err(error) => {
						println!("Error interpreting {:#?}:", path);
						println!("{}", error);
						let new_path = path.with_file_name("config_old.yml");
						match std::fs::rename(path, &new_path) {
							Ok(_) => println!("Renamed {:#?} to {:#?}", path, new_path),
							Err(error) => {
								println!("Error renaming {:#?}:", path);
								println!("{}", error);
								panic!("failed to rename uninterpreted configuration, please consider renaming {:#?} manually!", path);
							}
						}
					}
				}
			}
			Err(error) => {
				println!("Error reading {:#?}:", path);
				println!("{}", error);
			}
		}
		println!("Using default configuration");
		Self::default()
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

	pub fn set_window_size(&mut self, window_size: (f64, f64)) {
		self.changed = self.changed || self.window_size != Some(window_size);
		self.window_size = Some(window_size);
	}

	pub fn reset_window_size(&mut self) {
		self.changed = self.changed || self.window_size != None;
		self.window_size = None;
	}

	pub fn save_as<P: AsRef<std::path::Path>>(
		&self,
		path: P,
	) -> Result<(), Box<std::error::Error>> {
		if self.changed {
			let string = serde_yaml::to_string(self)?;

			std::fs::write(path, string)?;
		}

		Ok(())
	}
}
