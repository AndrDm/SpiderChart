use native_dialog::{DialogBuilder, MessageLevel}; //0.9.0

use std::{
	ffi::{CStr, CString},
	fs,
	os::raw::{c_int, c_void},
	path::{Path, PathBuf},
	ptr::{self},
};

use crate::chart_values_toml::*;
use crate::main_utils::*;
use crate::spiderchart::*;
use crate::spiderchart_draw::*;
use crate::user_int::*;
use crate::user_int_ex::*;

//==============================================================================
// Callbacks functions (pub)
//

#[unsafe(no_mangle)]
pub extern "C" fn on_change_callback(
	panel: c_int,
	_control: c_int,
	event: c_int,
	_callback_data: *mut c_void,
	_event_data1: c_int,
	_event_data2: c_int,
) -> c_int {
	if event == EVENT_COMMIT as i32 {
		draw_spider_chart(panel);
	}
	0
}

#[unsafe(no_mangle)]
pub extern "C" fn on_lang_change_callback(
	panel: c_int,
	_control: c_int,
	event: c_int,
	_callback_data: *mut c_void,
	_event_data1: c_int,
	_event_data2: c_int,
) -> c_int {
	if event == EVENT_COMMIT as i32 {
		reset_menu_bar();
		change_language();
		draw_spider_chart(panel); // Redraw the chart to apply language changes
	}
	0
}

#[unsafe(no_mangle)]
pub extern "C" fn main_panel_callback(
	panel: c_int,
	event: c_int,
	_callback_data: *mut c_void,
	_event_data1: c_int,
	_event_data2: c_int,
) -> c_int {
	if event == EVENT_CLOSE as i32 {
		quit_user_interface();
	}
	if event == EVENT_FILESDROPPED as i32 {
		//#define EVENT_FILESDROPPED 1005 toolbox.h
		// Prepare the arguments
		let mut file_names: *mut *mut i8 = std::ptr::null_mut();
		let file_names_ptr: *mut *mut *mut i8 = &mut file_names;
		let mut mouse_point = Point { x: 0, y: 0 };

		unsafe {
			let result =
				GetDragAndDropDataAnsi(file_names_ptr, &mut mouse_point);
			println!(
				"C function returned: {}, mouse point: {:?}",
				result, mouse_point
			);

			// Extract first file name
			if result == 0 && !file_names.is_null() {
				let first_file_ptr = *file_names; // Dereference to get char*
				println!("dereference");
				if !first_file_ptr.is_null() {
					let c_str = CStr::from_ptr(first_file_ptr);
					let file_name = c_str.to_string_lossy().into_owned();
					println!("First dropped file: {}", file_name);

					if file_name.to_lowercase().ends_with(".toml") {
						let path_buf = PathBuf::from(file_name);
						let path = path_buf.to_string_lossy().to_string();

						let values = ChartValuesToml::load_from_toml(&path);
						values.set_controls(panel);
						draw_spider_chart(panel);
						if mouse_point.x > 22
							&& mouse_point.x < 279
							&& mouse_point.y > 382
							&& mouse_point.y < 534
						{
							// Handle mouse click within the specified control (better to obtain coords later)
							// Define the favorites folder
							let favorites_dir = Path::new("favorites");

							// Create the folder if it doesn't exist
							if !favorites_dir.exists() {
								if let Err(e) =
									fs::create_dir_all(favorites_dir)
								{
									eprintln!(
										"Failed to create favorites directory: {}",
										e
									);
								}
							}

							// Copy the file into the favorites folder
							if let Some(file_name_only) = path_buf.file_name() {
								let dest_path =
									favorites_dir.join(file_name_only);
								if let Err(e) = fs::copy(&path_buf, &dest_path)
								{
									eprintln!(
										"Failed to copy file to favorites: {}",
										e
									);
								}
								populate_listbox_with_toml_files(
									PANEL as i32,
									PANEL_LISTBOX,
								);
							}
						}
					} else {
						println!("Skipped file: not a .toml file");
					}
				}
			}
			println!("file dropped");
		}
	}
	0
}

#[unsafe(no_mangle)]
pub extern "C" fn menu_help_callback(
	_menu_bar: i32,
	_menu_item: u32,
	_callback_data: *mut c_void,
	panel: i32,
) {
	println!("Menu Help Callback");
	let lang = get_numeric_value_i32(panel, PANEL_LANG);
	let file = if lang == 0 {
		"bin/SpiderChart-EN.chm"
	} else {
		"bin/SpiderChart-DE.chm"
	};
	let command: u32 = 0; // Replace with actual command like HH_DISPLAY_TOPIC
	let data: *mut std::ffi::c_void = ptr::null_mut(); // Or a valid pointer if needed

	// Create the ADS path by appending :Zone.Identifier
	let ads_path = format!("{}:Zone.Identifier", file);

	match fs::remove_file(&ads_path) {
		//unlock the file
		Ok(_) => println!("Zone.Identifier removed, file unblocked."),
		Err(e) => println!(
			"Failed to remove (or already removed) Zone.Identifier: {}",
			e
		),
	}
	show_html_help(file, command, data);
}

#[unsafe(no_mangle)]
pub extern "C" fn menu_callback(
	_menu_bar: i32,
	menu_item: u32,
	_callback_data: *mut c_void,
	panel: i32,
) {
	match menu_item {
		MENUBAR_FILE_QUIT => {
			println!("Quit Menu Callback");
			quit_user_interface();
		}
		MENUBAR_FILE_EXPORT => {
			export_as_png(panel);
			bring_fp_to_front();
		}
		MENUBAR_FILE_SAVE => {
			//save_as_ini(panel);
			save_as_toml(panel);
			bring_fp_to_front();
		}
		MENUBAR_FILE_OPEN => {
			//load_from_ini(panel);
			load_from_toml(panel);
			bring_fp_to_front();
			// Add open logic here if needed
		}

		MENUBAR_FILE_PRINT => unsafe {
			PrintCtrlAnsi(
				panel,
				PANEL_CANVAS as i32,
				0 as *const i8,
				VAL_INTEGRAL_SCALE as i32,
				1,
			);
		},

		MENUBAR_EDIT_COPY => unsafe {
			println!("Copying Spider Chart to Clipboard...");
			let mut bitmap: i32 = 0;
			let ctrl = PANEL_CANVAS as i32;
			let include_label = 0;
			GetCtrlDisplayBitmap(
				panel,
				ctrl,
				include_label,
				&mut bitmap as *mut i32,
			);
			if bitmap != 0 {
				ClipboardPutBitmap(bitmap);
				// balloon();
				DiscardBitmap(bitmap);
			} else {
				eprintln!("Failed to get bitmap from control.");
			}
		},

		MENUBAR_EDIT_RESET => {
			let confirmed = DialogBuilder::message()
				.set_level(MessageLevel::Info)
				.set_title(&get_string_value(PANEL as i32, PANEL_RESETTITLEMSG))
				.set_text(&get_string_value(PANEL as i32, PANEL_RESETTEXTMSG))
				.confirm()
				.show()
				.unwrap_or(false);

			if confirmed {
				set_ctrl_val_i32(panel, PANEL_IQI_TYPE, 0);
				set_ctrl_val_i32(panel, PANEL_THEME, 0);
				set_ctrl_val_str(
					panel,
					PANEL_DETECTOR_STRING,
					"Detector XYZ, Mode zyx",
				);
				set_ctrl_val_f64(panel, PANEL_ISRB, 240.0);
				set_ctrl_val_f64(panel, PANEL_CSA, 0.05);
				set_ctrl_val_f64(panel, PANEL_LAG, 0.3);
				set_ctrl_val_f64(panel, PANEL_SNRN, 820.0);
				set_ctrl_val_f64(panel, PANEL_SMTR, 125.0);
				set_ctrl_val_f64(panel, PANEL_ISOMTL, 90.0);
				set_ctrl_val_str(panel, PANEL_DETECTOR_TIMING, "");
				set_ctrl_val_str(panel, PANEL_DETECTOR_GAIN, "");
				set_ctrl_val_i32(panel, PANEL_AUTOSCALESWITCH, 0);

				draw_spider_chart(panel);
			}
		}

		MENUBAR_HELP_ABOUT => {
			let g_habout_panel = load_panel(
				0,
				CString::new("bin/SpiderChart.uir").unwrap().as_ptr(),
				PANELABOUT,
			);
			if g_habout_panel < 0 {
				return;
			}
			change_language();
			let version = env!("CARGO_PKG_VERSION");
			let version_str = format!("v.{}", version);
			set_ctrl_val_str(
				PANELABOUT as i32,
				PANELABOUT_ABOUTVER,
				&version_str,
			);
			// display_panel(g_habout_panel); // this works, but not modal
			unsafe {
				// NI CVI Recommended way to display a panel
				InstallPopup(g_habout_panel);
				let mut panel_data: i32 = 0;
				let mut control_data: i32 = 0;
				GetUserEvent(
					1,
					&mut panel_data as *mut i32,
					&mut control_data as *mut i32,
				);
				RemovePopup(g_habout_panel);
				DiscardPanel(g_habout_panel as i32);
			}
		}
		_ => {}
	}
}

#[unsafe(no_mangle)]
pub extern "C" fn language_callback(
	menu_bar: i32,
	menu_item: u32,
	_callback_data: *mut c_void,
	panel: i32,
) {
	let mut en = 0;
	let mut de = 0;
	unsafe {
		match menu_item {
			MENUBAR_VIEW_LANGUAGE_ENGLISH => {
				en = 1;
				de = 0;
			}
			MENUBAR_VIEW_LANGUAGE_GERMAN => {
				en = 0;
				de = 1;
				println!("Localizing to German");
			}

			_ => {}
		}
		SetMenuBarAttributeAnsi(
			menu_bar,
			MENUBAR_VIEW_LANGUAGE_ENGLISH as i32,
			ATTR_CHECKED as i32,
			en,
		);

		SetMenuBarAttributeAnsi(
			menu_bar,
			MENUBAR_VIEW_LANGUAGE_GERMAN as i32,
			ATTR_CHECKED as i32,
			de,
		);
		set_ctrl_val_i32(panel, PANEL_LANG, de); // 0 = English, 1 = German its OK as long as we have two languages only
		change_language();
	};
}

#[unsafe(no_mangle)]
pub extern "C" fn menu_theme_callback(
	menu_bar: i32,
	menu_item: u32,
	_callback_data: *mut c_void,
	panel: i32,
) {
	unsafe {
		SetMenuBarAttributeAnsi(
			menu_bar,
			MENUBAR_VIEW_THEME_LIGHT as i32,
			ATTR_CHECKED as i32,
			0,
		);

		SetMenuBarAttributeAnsi(
			menu_bar,
			MENUBAR_VIEW_THEME_DARK as i32,
			ATTR_CHECKED as i32,
			0,
		);

		SetMenuBarAttributeAnsi(
			menu_bar,
			MENUBAR_VIEW_THEME_LIGHT_AXIS as i32,
			ATTR_CHECKED as i32,
			0,
		);

		SetMenuBarAttributeAnsi(
			menu_bar,
			MENUBAR_VIEW_THEME_DARK_AXIS as i32,
			ATTR_CHECKED as i32,
			0,
		);

		SetMenuBarAttributeAnsi(
			menu_bar,
			MENUBAR_VIEW_THEME_PRIDE as i32,
			ATTR_CHECKED as i32,
			0,
		);
		SetMenuBarAttributeAnsi(
			menu_bar,
			menu_item as i32,
			ATTR_CHECKED as i32,
			1, // Set the selected theme as checked
		);
	}
	match menu_item {
		MENUBAR_VIEW_THEME_LIGHT => {
			set_ctrl_val_i32(panel, PANEL_THEME, 0);
			draw_spider_chart(panel);
		}
		MENUBAR_VIEW_THEME_DARK => {
			set_ctrl_val_i32(panel, PANEL_THEME, 1);
			draw_spider_chart(panel);
		}
		MENUBAR_VIEW_THEME_LIGHT_AXIS => {
			set_ctrl_val_i32(panel, PANEL_THEME, 2);
			draw_spider_chart(panel);
		}
		MENUBAR_VIEW_THEME_DARK_AXIS => {
			set_ctrl_val_i32(panel, PANEL_THEME, 3);
			draw_spider_chart(panel);
		}
		MENUBAR_VIEW_THEME_PRIDE => {
			set_ctrl_val_i32(panel, PANEL_THEME, 4);
			draw_spider_chart(panel);
		}

		_ => {}
	}
}

#[unsafe(no_mangle)]
pub extern "C" fn export_callback(
	panel: c_int,
	_control: c_int,
	event: c_int,
	_callback_data: *mut c_void,
	_event_data1: c_int,
	_event_data2: c_int,
) -> c_int {
	if event == EVENT_COMMIT as i32 {
		export_as_png(panel);
		bring_fp_to_front();
	}
	0
}

#[unsafe(no_mangle)]
pub extern "C" fn quit_callback(
	_panel: c_int,
	_control: c_int,
	event: c_int,
	_callback_data: *mut c_void,
	_event_data1: c_int,
	_event_data2: c_int,
) -> c_int {
	if event == EVENT_COMMIT as i32 {
		quit_user_interface();
	}
	0
}

#[unsafe(no_mangle)]
pub extern "C" fn close_about(
	_panel: c_int,
	_control: c_int,
	event: c_int,
	_callback_data: *mut c_void,
	_event_data1: c_int,
	_event_data2: c_int,
) -> c_int {
	if event == EVENT_COMMIT as i32
		|| (event == EVENT_KEYPRESS as i32
			&& _event_data1 == VAL_ESC_VKEY as i32)
	{
		unsafe {
			QueueUserEvent(
				1000,
				PANELABOUT as i32,
				PANELABOUT_CLOSEABOUT as i32,
			);
		}
	}

	0
}

#[unsafe(no_mangle)]
pub extern "C" fn close_panel_about(
	_panel: c_int,
	event: c_int,
	_callback_data: *mut c_void,
	_event_data1: c_int,
	_event_data2: c_int,
) -> c_int {
	if event == EVENT_CLOSE as i32 {
		unsafe {
			QueueUserEvent(
				1000,
				PANELABOUT as i32,
				PANELABOUT_CLOSEABOUT as i32,
			);
		}
	}
	0
}

#[unsafe(no_mangle)]
pub extern "C" fn quiality_menu_callback(
	_menu_bar: i32,
	_menu_item: i32,
	_callback_data: *mut c_void,
	_panel: i32,
) {
	let hpanel = load_panel(
		0,
		CString::new("bin/SpiderChart.uir").unwrap().as_ptr(),
		PANEL_2,
	);
	if hpanel < 0 {
		return;
	}
	change_language();
	set_table_labels(hpanel, PANEL_2_TABLE as i32);
	set_table_header_labels(hpanel, PANEL_2_TABLE_HEADERS as i32);
	fill_table_with_thresholds(hpanel, PANEL_2_TABLE as i32);

	display_panel(hpanel);
	//RunUserInterface();
}

#[unsafe(no_mangle)]
pub extern "C" fn load_callback(
	panel: c_int,
	_control: c_int,
	event: c_int,
	_callback_data: *mut c_void,
	_event_data1: c_int,
	_event_data2: c_int,
) -> c_int {
	if event == EVENT_COMMIT as i32 {
		//load_from_ini(panel);
		load_from_toml(panel);
		bring_fp_to_front();
	}
	0
}

//todo!("Implement listbox function");

#[unsafe(no_mangle)]
pub extern "C" fn listbox_callback(
	panel: c_int,
	_control: c_int,
	event: c_int,
	_callback_data: *mut c_void,
	_event_data1: c_int,
	_event_data2: c_int,
) -> c_int {
	// https://www.ni.com/docs/de-DE/bundle/labwindows-cvi/page/cvi/uiref/cviprogramming_with_list_box_controls.htm

	if event == EVENT_COMMIT as i32 {
		let selected_file = get_string_value(panel, PANEL_LISTBOX);
		let compare = get_numeric_value_i32(panel, PANEL_COMPARESWITCH);

		if !selected_file.is_empty() && compare == 0 {
			// Prepend "favorites" subfolder
			let path_buf = PathBuf::from("favorites").join(selected_file);

			let path = path_buf.to_string_lossy().to_string();

			let values = ChartValuesToml::load_from_toml(&path);
			values.set_controls(panel);
			draw_spider_chart(panel);
		}
		draw_spider_chart(panel);
	}

	0
}

#[unsafe(no_mangle)]
pub extern "C" fn on_compare_callback(
	panel: c_int,
	_control: c_int,
	event: c_int,
	_callback_data: *mut c_void,
	_event_data1: c_int,
	_event_data2: c_int,
) -> c_int {
	if event == EVENT_COMMIT as i32 {
		let compare = get_numeric_value_i32(panel, PANEL_COMPARESWITCH);
		set_attribute_u32(
			panel,
			PANEL_LISTBOX,
			ATTR_CHECK_MODE,
			compare as u32,
		);

		let dim_controls = [
			PANEL_DETECTOR_STRING,
			PANEL_IQI_TYPE,
			PANEL_DETECTOR_TIMING,
			PANEL_DETECTOR_GAIN,
			PANEL_ISRB,
			PANEL_CSA,
			PANEL_LAG,
			PANEL_ISRB_CLASS,
			PANEL_CSA_CLASS,
			PANEL_LAG_CLASS,
			PANEL_SNRN,
			PANEL_SMTR,
			PANEL_ISOMTL,
			PANEL_SNRN_CLASS,
			PANEL_SMTR_CLASS,
			PANEL_ISOMTL_CLASS,
			PANEL_SAVE_AS_BUTTON,
			PANEL_LOAD_BUTTON,
			PANEL_PATTERNSWITCH,
		];

		let menu_items = [
			(MENUBAR_FILE, MENUBAR_FILE_OPEN),
			(MENUBAR_FILE, MENUBAR_FILE_SAVE),
			(MENUBAR_EDIT, MENUBAR_EDIT_RESET),
		];

		for &ctrl in &dim_controls {
			set_attribute_u32(panel, ctrl, ATTR_DIMMED, compare as u32);
		}
		for &(menu_bar, item) in &menu_items {
			set_menu_bar_attribute(menu_bar, item, ATTR_DIMMED, compare as u32);
		}

		draw_spider_chart(panel);
	}
	0
}

#[unsafe(no_mangle)]
pub extern "C" fn save_as_callback(
	panel: c_int,
	_control: c_int,
	event: c_int,
	_callback_data: *mut c_void,
	_event_data1: c_int,
	_event_data2: c_int,
) -> c_int {
	if event == EVENT_COMMIT as i32 {
		//save_as_ini(panel);
		save_as_toml(panel);
		bring_fp_to_front();
	}
	0
}

#[unsafe(no_mangle)]
pub extern "C" fn quality_numbers_panel_callback(
	panel: c_int,
	event: c_int,
	_callback_data: *mut c_void,
	_event_data1: c_int,
	_event_data2: c_int,
) -> c_int {
	if event == EVENT_CLOSE as i32 {
		unsafe {
			//QuitUserInterface(panel);
			DiscardPanel(panel as i32);
		}
	}
	if event == EVENT_KEYPRESS as i32 {
		if _event_data1 == VAL_ESC_VKEY as i32 {
			unsafe {
				DiscardPanel(panel as i32);
			}
		}
	}
	0
}

// Just for fun:
#[unsafe(no_mangle)]
pub extern "C" fn scratch_pad(
	panel: c_int,
	control: c_int,
	event: c_int,
	_callback_data: *mut c_void,
	_event_data1: c_int,
	_event_data2: c_int,
) -> c_int {
	if event == EVENT_LEFT_CLICK as i32 {
		let mut mouse_x = 0;
		let mut mouse_y = 0;

		unsafe {
			SetCtrlAttributeAnsi(
				panel,
				PANEL_TIMER as i32,
				ATTR_ENABLED as i32,
				1,
			);
			GetRelativeMouseState(
				panel,
				control,
				&mut mouse_x,
				&mut mouse_y,
				ptr::null_mut(),
				ptr::null_mut(),
				ptr::null_mut(),
			);
			CanvasSetPenPosition(panel, control, MakePoint(mouse_x, mouse_y));
			SetCtrlAttributeAnsi(
				panel,
				PANEL_CANVAS as i32,
				ATTR_PEN_COLOR as i32,
				VAL_RED as i32,
			);
		}
	}
	0
}

#[unsafe(no_mangle)]
pub extern "C" fn draw_to_canvas(
	panel: c_int,
	_control: c_int, // this one referred to Timer!
	event: c_int,
	_callback_data: *mut c_void,
	_event_data1: c_int,
	_event_data2: c_int,
) -> c_int {
	if event == EVENT_TIMER_TICK as i32 {
		let mut mouse_x = 0;
		let mut mouse_y = 0;
		let mut left_down = 0;

		unsafe {
			//Canvas, not Timer!
			GetRelativeMouseState(
				panel,
				PANEL_CANVAS as i32,
				&mut mouse_x,
				&mut mouse_y,
				&mut left_down,
				ptr::null_mut(),
				ptr::null_mut(),
			);

			if left_down == 0 {
				SetCtrlAttributeAnsi(
					panel,
					PANEL_TIMER as i32,
					ATTR_ENABLED as i32,
					0,
				);
			} else {
				CanvasDrawLineTo(
					panel,
					PANEL_CANVAS as i32,
					MakePoint(mouse_x, mouse_y),
				);
			}
		}
	}
	0
}

//
// End of callback area
//==============================================================================
