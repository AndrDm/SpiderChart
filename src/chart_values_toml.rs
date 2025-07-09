use serde::{Deserialize, Serialize};
use std::{fs, os::raw::c_int, path::Path};

use crate::spiderchart::*;
use crate::user_int_ex::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartValuesToml {
	iqi: i32,
	detector: String,
	isrb: f64,
	csa: f64,
	lag: f64,
	snrn: f64,
	smtr: f64,
	mtl: f64,
	#[serde(default)]
	// Default to empty string if not set - nice feature of serde
	timing: String,
	#[serde(default)]
	gain: String,
}

impl ChartValuesToml {
	pub fn defaults() -> Self {
		Self {
			iqi: 0,
			detector: String::from("Detector XYZ, Model zyx"),
			isrb: 240.0,
			csa: 0.05,
			lag: 0.30,
			snrn: 820.0,
			smtr: 125.0,
			mtl: 90.0,
			timing: String::from(""),
			gain: String::from(""),
		}
	}

	pub fn iqi(&self) -> i32 {
		self.iqi
	}
	//pub fn detector(&self) -> &str {
	//	&self.detector
	//}
	pub fn isrb(&self) -> f64 {
		self.isrb
	}
	pub fn csa(&self) -> f64 {
		self.csa
	}
	pub fn lag(&self) -> f64 {
		self.lag
	}
	pub fn snrn(&self) -> f64 {
		self.snrn
	}
	pub fn smtr(&self) -> f64 {
		self.smtr
	}
	pub fn mtl(&self) -> f64 {
		self.mtl
	}
	/*
	pub fn timing(&self) -> &str {
		&self.timing
	}
	pub fn gain(&self) -> &str {
		&self.gain
	}
	*/

	pub fn load_from_toml(path: &str) -> Self {
		if Path::new(path).exists() {
			match fs::read_to_string(path) {
				Ok(content) => toml::from_str(&content).unwrap_or_else(|_| {
					println!("OK to parse TOML from {}.", path);
					let defaults = Self::defaults();
					//defaults.save_to_toml(path);
					defaults
				}),
				Err(_) => {
					println!(
						"Failed to read from {}. Using default values.",
						path
					);
					let defaults = Self::defaults();
					//defaults.save_to_toml(path);
					defaults
				}
			}
		} else {
			println!("File {} does not exist. Using default values.", path);
			let defaults = Self::defaults();
			//defaults.save_to_toml(path);
			defaults
		}
	}

	pub fn save_to_toml(&self, path: &str) {
		let toml_str = toml::to_string_pretty(self).unwrap();
		fs::write(path, toml_str).unwrap();
	}

	pub fn set_controls(&self, hpanel: c_int) {
		set_ctrl_val_i32(hpanel, PANEL_IQI_TYPE, self.iqi);
		set_ctrl_val_str(hpanel, PANEL_DETECTOR_STRING, &self.detector);
		set_ctrl_val_f64(hpanel, PANEL_ISRB, self.isrb);
		set_ctrl_val_f64(hpanel, PANEL_CSA, self.csa);
		set_ctrl_val_f64(hpanel, PANEL_LAG, self.lag);
		set_ctrl_val_f64(hpanel, PANEL_SNRN, self.snrn);
		set_ctrl_val_f64(hpanel, PANEL_SMTR, self.smtr);
		set_ctrl_val_f64(hpanel, PANEL_ISOMTL, self.mtl);
		set_ctrl_val_str(hpanel, PANEL_DETECTOR_TIMING, &self.timing);
		set_ctrl_val_str(hpanel, PANEL_DETECTOR_GAIN, &self.gain);
	}

	pub fn from_controls(hpanel: c_int) -> Self {
		Self {
			iqi: get_numeric_value_i32(hpanel, PANEL_IQI_TYPE),
			detector: get_string_value(hpanel, PANEL_DETECTOR_STRING),
			isrb: get_numeric_value(hpanel, PANEL_ISRB),
			csa: get_numeric_value(hpanel, PANEL_CSA),
			lag: get_numeric_value(hpanel, PANEL_LAG),
			snrn: get_numeric_value(hpanel, PANEL_SNRN),
			smtr: get_numeric_value(hpanel, PANEL_SMTR),
			mtl: get_numeric_value(hpanel, PANEL_ISOMTL),
			timing: get_string_value(hpanel, PANEL_DETECTOR_TIMING),
			gain: get_string_value(hpanel, PANEL_DETECTOR_GAIN),
		}
	}

	#[allow(dead_code)]
	pub fn print(&self) {
		println!(
			"TOML values:\n  iqi: {}\n  detector: \"{}\"\n  isrb: {}\n  csa: {}\n  lag: {}\n  snrn: {}\n  smtr: {}\n  mtl: {}\n  timing: \"{}\"\n  gain: \"{}\"",
			self.iqi,
			self.detector,
			self.isrb,
			self.csa,
			self.lag,
			self.snrn,
			self.smtr,
			self.mtl,
			self.timing,
			self.gain
		);
	}
}
