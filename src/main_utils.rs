#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]

use crate::chart_values_ini::*;
use crate::chart_values_toml::*;
use crate::localui::*;
use crate::spiderchart::*;
use crate::spiderchart_draw::*;
use crate::tables::*;
use crate::tables::*;
use crate::user_int::*;
use crate::user_int_ex::*;

use hide_console::hide_console;
use ini::Ini;
//use native_dialog::MessageDialog; //0.7.0
use native_dialog::{DialogBuilder, MessageLevel}; //0.9.0
use rfd::FileDialog;
use std::{
	ffi::CString,
	fs,
	os::raw::{c_char, c_int, c_void},
	path::Path,
	ptr::{self, null_mut},
};

use windows::{
	Win32::{
		Foundation::HWND,
		Graphics::Gdi::*,
		UI::WindowsAndMessaging::{
			GWL_EXSTYLE, GetWindowLongPtrW, SetWindowLongPtrW, WS_EX_APPWINDOW,
		},
	},
	core::*,
};

pub fn classify_value(value: f64, iqi_type: usize, param: usize) -> usize {
	use THRESHOLDS;
	let thresholds = THRESHOLDS
		.get(iqi_type)
		.and_then(|row| row.get(param))
		.expect("Invalid IQI type or parameter index");

	if thresholds.len() < 2 {
		return 0;
	}

	// Detect order: descending or ascending
	let descending = thresholds[0] > thresholds[thresholds.len() - 1];

	if descending {
		for (class, &threshold) in thresholds.iter().enumerate() {
			if value == threshold {
				return class;
			}
			if value > threshold {
				if class == 0 {
					return class;
				} else {
					return class - 1;
				}
			}
		}
	} else {
		for (class, &threshold) in thresholds.iter().enumerate() {
			if value == threshold {
				return class;
			}
			if value < threshold {
				if class == 0 {
					return class;
				} else {
					return class - 1;
				}
			}
		}
	}
	// If loop completes without returning, you may want to return a default
	thresholds.len().min(25).max(0)
}

pub fn load_cui_font() {
	unsafe {
		let font_path =
			widestring::U16CString::from_str("./fonts/PoppinsUI-Regular.ttf")
				.unwrap();
		let fonts_added = AddFontResourceExW(
			PCWSTR(font_path.as_ptr()),
			FR_PRIVATE,
			Some(null_mut()),
		);

		if fonts_added > 0 {
			println!("Font loaded successfully.");
		} else {
			eprintln!("Failed to load font.");
		}
	}
}

pub fn set_window_style(hpanel: i32) {
	let mut hwnd: HWND = HWND(std::ptr::null_mut()); // Dummy HWND

	// Modify the extended window style
	unsafe {
		GetPanelAttributeAnsi(
			hpanel,
			ATTR_SYSTEM_WINDOW_HANDLE as i32,
			&mut hwnd as *mut _ as *mut c_void,
		);
		let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
		SetWindowLongPtrW(
			hwnd,
			GWL_EXSTYLE,
			ex_style | WS_EX_APPWINDOW.0 as isize,
		);
	}
}

pub fn format_float(value: f64) -> String {
	let s = format!("{:.3}", value); // Format with 3 decimal places
	let s = s.trim_end_matches('0'); // Remove trailing zeroes
	let s = s.trim_end_matches('.'); // Remove trailing dot if no decimals remain
	s.to_string()
}

use windows::Win32::UI::WindowsAndMessaging::{
	HWND_TOP, HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW,
	SetForegroundWindow, SetWindowPos,
};

pub fn bring_fp_to_front() {
	unsafe {
		let hwnd = GetCVIWindowHandle();

		let _ = SetWindowPos(
			HWND(hwnd as *mut c_void),
			Some(HWND_TOP),
			0,
			0,
			0,
			0,
			SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
		);
	}
}

pub fn populate_listbox_with_toml_files(panel_handle: i32, list_box_ctrl: u32) {
	let favorites_path = Path::new("Favorites");

	if let Ok(entries) = fs::read_dir(favorites_path) {
		unsafe {
			ClearListCtrl(panel_handle as i32, list_box_ctrl as i32);
		}
		for (index, entry) in entries.flatten().enumerate() {
			let path = entry.path();
			if let Some(ext) = path.extension() {
				if ext == "toml" {
					if let Some(file_name) =
						path.file_name().and_then(|n| n.to_str())
					{
						let c_string = CString::new(file_name).unwrap();
						unsafe {
							println!(
								"Inserting item: {}",
								c_string.to_string_lossy()
							);
							InsertListItemAnsi(
								panel_handle,
								list_box_ctrl as i32,
								index as i32,
								c_string.as_ptr(),
								c_string.as_ptr(),
							);
						}
					}
				}
			}
		}
	} else {
		eprintln!("Could not read Favorites directory.");
	}
}

pub fn reset_menu_bar() {
	unsafe {
		let theme = get_numeric_value_i32(PANEL as i32, PANEL_THEME);

		SetMenuBarAttributeAnsi(
			MENUBAR_FILE as i32,
			MENUBAR_VIEW_THEME_LIGHT as i32,
			ATTR_CHECKED as i32,
			0,
		);
		SetMenuBarAttributeAnsi(
			MENUBAR_FILE as i32,
			MENUBAR_VIEW_THEME_DARK as i32,
			ATTR_CHECKED as i32,
			0,
		);
		SetMenuBarAttributeAnsi(
			MENUBAR_FILE as i32,
			MENUBAR_VIEW_THEME_LIGHT_AXIS as i32,
			ATTR_CHECKED as i32,
			0,
		);
		SetMenuBarAttributeAnsi(
			MENUBAR_FILE as i32,
			MENUBAR_VIEW_THEME_DARK_AXIS as i32,
			ATTR_CHECKED as i32,
			0,
		);
		SetMenuBarAttributeAnsi(
			MENUBAR_FILE as i32,
			MENUBAR_VIEW_THEME_PRIDE as i32,
			ATTR_CHECKED as i32,
			0,
		);
		match theme {
			0 => {
				SetMenuBarAttributeAnsi(
					MENUBAR_FILE as i32,
					MENUBAR_VIEW_THEME_LIGHT as i32,
					ATTR_CHECKED as i32,
					1,
				);
			}
			1 => {
				SetMenuBarAttributeAnsi(
					MENUBAR_FILE as i32,
					MENUBAR_VIEW_THEME_DARK as i32,
					ATTR_CHECKED as i32,
					1,
				);
			}
			2 => {
				SetMenuBarAttributeAnsi(
					MENUBAR_FILE as i32,
					MENUBAR_VIEW_THEME_LIGHT_AXIS as i32,
					ATTR_CHECKED as i32,
					1,
				);
			}
			3 => {
				SetMenuBarAttributeAnsi(
					MENUBAR_FILE as i32,
					MENUBAR_VIEW_THEME_DARK_AXIS as i32,
					ATTR_CHECKED as i32,
					1,
				);
			}
			4 => {
				SetMenuBarAttributeAnsi(
					MENUBAR_FILE as i32,
					MENUBAR_VIEW_THEME_PRIDE as i32,
					ATTR_CHECKED as i32,
					1,
				);
			}
			_ => {}
		}
		let lang = get_numeric_value_i32(PANEL as i32, PANEL_LANG);
		SetMenuBarAttributeAnsi(
			MENUBAR_FILE as i32,
			MENUBAR_VIEW_LANGUAGE_ENGLISH as i32,
			ATTR_CHECKED as i32,
			0,
		);
		SetMenuBarAttributeAnsi(
			MENUBAR_FILE as i32,
			MENUBAR_VIEW_LANGUAGE_GERMAN as i32,
			ATTR_CHECKED as i32,
			0,
		);

		match lang {
			0 => {
				SetMenuBarAttributeAnsi(
					MENUBAR_FILE as i32,
					MENUBAR_VIEW_LANGUAGE_ENGLISH as i32,
					ATTR_CHECKED as i32,
					1,
				);
			}
			1 => {
				SetMenuBarAttributeAnsi(
					MENUBAR_FILE as i32,
					MENUBAR_VIEW_LANGUAGE_GERMAN as i32,
					ATTR_CHECKED as i32,
					1,
				);
			}

			_ => {}
		}
	}
}

pub fn change_language() {
	unsafe {
		let lang = get_numeric_value_i32(PANEL as i32, PANEL_LANG);
		let panels = [PANEL, PANEL_2, PANELABOUT];

		for &panel in &panels {
			let hpanel = panel as i32;

			let path = if lang == 0 {
				if cfg!(debug_assertions) {
					"target/debug/bin/SpiderChart-EN.lwl"
				} else {
					"bin/SpiderChart-EN.lwl"
				}
			} else {
				if cfg!(debug_assertions) {
					"target/debug/bin/SpiderChart-DE.lwl"
				} else {
					"bin/SpiderChart-DE.lwl"
				}
			};

			LocalizePanelUtf8(hpanel, CString::new(path).unwrap().as_ptr());
		}
	}
}

//==============================================================================
// Table helper functions
//
pub const NUM_ROWS: i32 = 26;
pub const NUM_COLS: i32 = 24;

pub fn set_table_labels(panel: i32, table_ctrl: i32) {
	let col_labels = ["iSRB", "CSa", "Lag", "SNRn", "SMTR", "ISO MTL"];
	unsafe {
		SetTableRowAttributeAnsi(
			panel,
			table_ctrl,
			-1,
			ATTR_USE_LABEL_TEXT as i32,
			1,
		);

		// Set row labels: "0" to "25"
		for row in 0..NUM_ROWS {
			let label = row.to_string();
			let c_label = std::ffi::CString::new(label).unwrap();
			SetTableRowAttributeAnsi(
				panel,
				table_ctrl,
				row + 1,
				ATTR_LABEL_TEXT as i32,
				c_label.as_ptr(),
				//row,
			);
			SetTableRowAttributeAnsi(
				panel,
				table_ctrl,
				row + 1,
				ATTR_SIZE_MODE as i32,
				VAL_USE_EXPLICIT_SIZE as i32,
				//row,
			);

			SetTableRowAttributeAnsi(
				panel,
				table_ctrl,
				row + 1,
				ATTR_ROW_HEIGHT as i32,
				21,
				//row,
			);
		}

		SetTableColumnAttributeAnsi(
			panel,
			table_ctrl,
			-1,
			ATTR_USE_LABEL_TEXT as i32,
			1,
		);
		// Set column labels: repeat col_labels[] 4 times
		for col in 0..NUM_COLS {
			let label = col_labels[(col % col_labels.len() as i32) as usize];
			let c_label = std::ffi::CString::new(label).unwrap();
			SetTableColumnAttributeUtf8(
				panel,
				table_ctrl,
				col + 1,
				ATTR_LABEL_TEXT as i32,
				c_label.as_ptr(),
			);

			SetTableColumnAttributeUtf8(
				panel,
				table_ctrl,
				col + 1,
				ATTR_COLUMN_WIDTH as i32,
				50,
				//row,
			);
		}
	}
}

pub fn set_table_header_labels(panel: i32, table_ctrl: i32) {
	let col_labels = [
		"Aluminium 6061/7022",
		"Stainless Steel 316L",
		"Inconel 719",
		"Titanium Ti-6Al-4V",
	];
	unsafe {
		SetTableColumnAttributeAnsi(
			panel,
			table_ctrl,
			-1,
			ATTR_USE_LABEL_TEXT as i32,
			1,
		);

		// Set column labels dynamically based on col_labels length
		for col in 0..col_labels.len() {
			let label = col_labels[col];
			let c_label = std::ffi::CString::new(label).unwrap();

			SetTableColumnAttributeUtf8(
				panel,
				table_ctrl,
				col as i32 + 1,
				ATTR_LABEL_TEXT as i32,
				c_label.as_ptr(),
			);

			SetTableColumnAttributeUtf8(
				panel,
				table_ctrl,
				col as i32 + 1,
				ATTR_COLUMN_WIDTH as i32,
				300,
			);
		}
	}
}

pub fn fill_table_with_thresholds(panel: i32, table_ctrl: i32) {
	// Each array is a column, each value is a row
	let columns: [&[f64; 26]; 24] = [
		&ISRB_THRESHOLDS,
		&AL_CSA_THR,
		&AL_LAG_THR,
		&AL_SNRN_THR,
		&AL_SMTR_THR,
		&AL_ISO_MTL_THR,
		&SS_ISRB_THR,
		&SS_CSA_THR,
		&SS_LAG_THR,
		&SS_SNRN_THR,
		&SS_SMTR_THR,
		&SS_ISO_MTL_THR,
		&IN_ISRB_THR,
		&IN_CSA_THR,
		&IN_LAG_THR,
		&IN_SNRN_THR,
		&IN_SMTR_THR,
		&IN_ISO_MTL_THR,
		&TI_ISRB_THR,
		&TI_CSA_THR,
		&TI_LAG_THR,
		&TI_SNRN_THR,
		&TI_SMTR_THR,
		&TI_ISO_MTL_THR,
	];

	for (col, arr) in columns.iter().enumerate() {
		for (row, &val) in arr.iter().enumerate() {
			unsafe {
				// Table indices are usually 1-based in LabWindows/CVI
				// https://www.ni.com/docs/de-DE/bundle/labwindows-cvi/page/cvi/uiref/cvisettablecellval.htm
				let cell = MakePoint((col + 1) as i32, (row + 1) as i32);
				SetTableCellValAnsi(panel, table_ctrl, cell, val);
			}
		}
	}
}

pub fn export_as_png(panel: i32) -> i32 {
	let file = FileDialog::new()
		//.set_title("Save Spider Chart As PNG Image...")
		.set_title(get_string_value(panel, PANEL_EXPORTPNGMSG))
		.add_filter("PNG Image", &["png"])
		.save_file();
	if let Some(path_buf) = file {
		let path = path_buf.to_string_lossy().to_string();
		let _ = save_canvas_bitmap(panel, PANEL_CANVAS, &path);
	}
	0
}

pub fn save_as_toml(panel: i32) -> i32 {
	let file = FileDialog::new()
		.set_title(get_string_value(panel, PANEL_SAVETOMLMSG))
		.add_filter("TOML File", &["toml"])
		.save_file();
	if let Some(path_buf) = file {
		let path = path_buf.to_string_lossy().to_string();
		let current_values = ChartValuesToml::from_controls(panel);
		current_values.save_to_toml(&path);
	}
	0
}

pub fn load_from_toml(panel: i32) -> i32 {
	let file = FileDialog::new()
		.set_title(get_string_value(panel, PANEL_LOADTOMLMSG))
		.add_filter("TOML File", &["toml"])
		.pick_file();
	if let Some(path_buf) = file {
		let path = path_buf.to_string_lossy().to_string();
		let values = ChartValuesToml::load_from_toml(&path);
		values.set_controls(panel);
		draw_spider_chart(panel);
	}
	0
}
