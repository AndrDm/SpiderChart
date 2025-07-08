// mod user_int;

use ini::Ini;
use std::{os::raw::c_int, path::Path};

use crate::spiderchart::*;
use crate::user_int_ex::*;

/// Struct to hold all chart values
#[derive(Debug, Clone)]
pub struct ChartValues {
	iqi: i32,
	detector: String,
	isrb: f64,
	csa: f64,
	lag: f64,
	snrn: f64,
	smtr: f64,
	mtl: f64,
	theme: i32,
	language: i32,
	pattern: i32,
	timing: String,
	gain: String,
	auto_scale: i32,
}

impl ChartValues {
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
			theme: 0,
			language: 0,
			pattern: 1,
			timing: String::from(""),
			gain: String::from(""),
			auto_scale: 0, // Default to auto scale disabled
		}
	}

	/// Load from INI file or use defaults
	pub fn load_from_ini(path: &str) -> (Self, Ini) {
		let mut conf = if Path::new(path).exists() {
			// Try to load the file, use defaults if it fails
			match Ini::load_from_file(path) {
				Ok(ini) => ini,
				Err(_) => {
					// File exists but is not valid INI, use defaults
					let mut ini = Ini::new();
					let defaults = Self::defaults();
					ini.with_section(Some("Values"))
						.set("iqi", defaults.iqi.to_string())
						.set("detector", defaults.detector.clone())
						.set("isrb", defaults.isrb.to_string())
						.set("csa", defaults.csa.to_string())
						.set("lag", defaults.lag.to_string())
						.set("snrn", defaults.snrn.to_string())
						.set("smtr", defaults.smtr.to_string())
						.set("mtl", defaults.mtl.to_string())
						.set("theme", defaults.theme.to_string())
						.set("language", defaults.language.to_string())
						.set("pattern", defaults.pattern.to_string())
						.set("timing", defaults.timing.clone())
						.set("gain", defaults.gain.clone())
						.set("auto_scale", defaults.auto_scale.to_string());
					// ini.write_to_file(path).unwrap();
					ini
				}
			}
		} else {
			// File does not exist, use defaults
			let mut ini = Ini::new();
			let defaults = Self::defaults();
			ini.with_section(Some("Values"))
				.set("iqi", defaults.iqi.to_string())
				.set("detector", defaults.detector.clone())
				.set("isrb", defaults.isrb.to_string())
				.set("csa", defaults.csa.to_string())
				.set("lag", defaults.lag.to_string())
				.set("snrn", defaults.snrn.to_string())
				.set("smtr", defaults.smtr.to_string())
				.set("mtl", defaults.mtl.to_string())
				.set("theme", defaults.theme.to_string())
				.set("pattern", defaults.pattern.to_string())
				.set("language", defaults.language.to_string())
				.set("timing", defaults.timing.clone())
				.set("gain", defaults.gain.clone())
				.set("auto_scale", defaults.auto_scale.to_string());
			ini.write_to_file(path).unwrap();
			ini
		};

		// Check if the "Values" section exists, if not, create it with defaults
		// Ensure "Values" section exists with defaults
		if conf.section(Some("Values")).is_none() {
			let defaults = ChartValues::defaults();
			conf.with_section(Some("Values"))
				.set("iqi", defaults.iqi.to_string())
				.set("detector", defaults.detector.clone())
				.set("isrb", defaults.isrb.to_string())
				.set("csa", defaults.csa.to_string())
				.set("lag", defaults.lag.to_string())
				.set("snrn", defaults.snrn.to_string())
				.set("smtr", defaults.smtr.to_string())
				.set("mtl", defaults.mtl.to_string())
				.set("theme", defaults.theme.to_string())
				.set("language", defaults.language.to_string())
				.set("pattern", defaults.pattern.to_string())
				.set("timing", defaults.timing.clone())
				.set("gain", defaults.gain.clone())
				.set("auto_scale", defaults.auto_scale.to_string());
		}

		// Now, get the section for reading
		let section = conf.section(Some("Values")).unwrap();

		let get = |key: &str, default: &str| {
			section.get(key).unwrap_or(default).to_string()
		};

		let defaults = Self::defaults();
		let values = Self {
			iqi: get("iqi", &defaults.iqi.to_string())
				.parse()
				.unwrap_or(defaults.iqi),
			detector: get("detector", &defaults.detector),
			isrb: get("isrb", &defaults.isrb.to_string())
				.parse()
				.unwrap_or(defaults.isrb),
			csa: get("csa", &defaults.csa.to_string())
				.parse()
				.unwrap_or(defaults.csa),
			lag: get("lag", &defaults.lag.to_string())
				.parse()
				.unwrap_or(defaults.lag),
			snrn: get("snrn", &defaults.snrn.to_string())
				.parse()
				.unwrap_or(defaults.snrn),
			smtr: get("smtr", &defaults.smtr.to_string())
				.parse()
				.unwrap_or(defaults.smtr),
			mtl: get("mtl", &defaults.mtl.to_string())
				.parse()
				.unwrap_or(defaults.mtl),
			theme: get("theme", &defaults.theme.to_string())
				.parse()
				.unwrap_or(defaults.theme),
			language: get("language", &defaults.language.to_string())
				.parse()
				.unwrap_or(defaults.language),
			pattern: get("pattern", &defaults.pattern.to_string())
				.parse()
				.unwrap_or(defaults.pattern),
			timing: get("timing", &defaults.timing).clone(),
			gain: get("gain", &defaults.gain).clone(),
			auto_scale: get("auto_scale", &defaults.auto_scale.to_string())
				.parse()
				.unwrap_or(defaults.auto_scale),
		};
		(values, conf)
	}

	/// Save to INI file
	pub fn save_to_ini(&self, conf: &mut Ini, path: &str) {
		conf.with_section(Some("Values"))
			.set("iqi", self.iqi.to_string())
			.set("detector", self.detector.clone())
			.set("isrb", self.isrb.to_string())
			.set("csa", self.csa.to_string())
			.set("lag", self.lag.to_string())
			.set("snrn", self.snrn.to_string())
			.set("smtr", self.smtr.to_string())
			.set("mtl", self.mtl.to_string())
			.set("theme", self.theme.to_string())
			.set("language", self.language.to_string())
			.set("pattern", self.pattern.to_string())
			.set("timing", self.timing.clone())
			.set("gain", self.gain.clone())
			.set("auto_scale", self.auto_scale.to_string());
		conf.write_to_file(path).unwrap();
	}

	/// Set all controls from values
	pub fn set_controls(&self, hpanel: c_int) {
		set_ctrl_val_i32(hpanel, PANEL_IQI_TYPE, self.iqi);
		set_ctrl_val_str(hpanel, PANEL_DETECTOR_STRING, &self.detector);
		set_ctrl_val_f64(hpanel, PANEL_ISRB, self.isrb);
		set_ctrl_val_f64(hpanel, PANEL_CSA, self.csa);
		set_ctrl_val_f64(hpanel, PANEL_LAG, self.lag);
		set_ctrl_val_f64(hpanel, PANEL_SNRN, self.snrn);
		set_ctrl_val_f64(hpanel, PANEL_SMTR, self.smtr);
		set_ctrl_val_f64(hpanel, PANEL_ISOMTL, self.mtl);
		set_ctrl_val_i32(hpanel, PANEL_THEME, self.theme);
		set_ctrl_val_i32(hpanel, PANEL_LANG, self.language);
		set_ctrl_val_i32(hpanel, PANEL_PATTERNSWITCH, self.pattern);
		set_ctrl_val_str(hpanel, PANEL_DETECTOR_TIMING, &self.timing);
		set_ctrl_val_str(hpanel, PANEL_DETECTOR_GAIN, &self.gain);
		set_ctrl_val_i32(hpanel, PANEL_AUTOSCALESWITCH, self.auto_scale);
	}

	/// Get all values from controls
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
			theme: get_numeric_value_i32(hpanel, PANEL_THEME),
			language: get_numeric_value_i32(hpanel, PANEL_LANG),
			pattern: get_numeric_value_i32(hpanel, PANEL_PATTERNSWITCH),
			timing: get_string_value(hpanel, PANEL_DETECTOR_TIMING),
			gain: get_string_value(hpanel, PANEL_DETECTOR_GAIN),
			auto_scale: get_numeric_value_i32(hpanel, PANEL_AUTOSCALESWITCH),
		}
	}

	pub fn print(&self) {
		println!(
			"INI values:\n  iqi: {}\n  detector: \"{}\"\n  isrb: {}\n  csa: {}\n  lag: {}\n  snrn: {}\n  smtr: {}\n  mtl: {} \n  theme: {}\n  language: {}\n  pattern: {} \n  timing: \"{}\"\n  gain: \"{}\"\n  auto_scale: {}",
			self.iqi,
			self.detector,
			self.isrb,
			self.csa,
			self.lag,
			self.snrn,
			self.smtr,
			self.mtl,
			self.theme,
			self.language,
			self.pattern,
			self.timing,
			self.gain,
			self.auto_scale
		);
	}
}
