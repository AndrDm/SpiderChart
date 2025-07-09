//==============================================================================
//
// Title:		SpiderChart Draw
// Purpose:		Drawing of the Spider Chart Diagram (Net Summary Plot)
//
//==============================================================================
use crate::{
	chart_values_toml::*, main_utils::*, spiderchart::*, user_int::*,
	user_int_ex::*,
};
use std::{
	ffi::{CStr, CString},
	os::raw::{c_char, c_int},
	path::PathBuf,
};

//==============================================================================
// DRAW
//

pub fn draw_spider_chart(hpanel: c_int) {
	let compare = get_numeric_value_i32(hpanel, PANEL_COMPARESWITCH);
	if compare == 1 {
		// If compare mode is enabled, draw the comparison chart
		//draw_spider_chart_compare(hpanel);
		let mut total_checked = 0;
		let mut total_items = 0;
		unsafe {
			GetNumCheckedItems(
				hpanel,
				PANEL_LISTBOX as i32,
				&mut total_checked as *mut i32,
			);

			GetNumListItems(
				hpanel,
				PANEL_LISTBOX as i32,
				&mut total_items as *mut i32,
			);
			let theme = get_numeric_value_i32(hpanel, PANEL_THEME);
			if theme == 1 || theme == 3 {
				clear_canvas(hpanel, PANEL_CANVAS as i32, 0x1c1c1c); //Dark
				set_pen_color(hpanel, PANEL_CANVAS, VAL_WHITE);
			} else {
				clear_canvas(hpanel, PANEL_CANVAS as i32, 0xFFFFFF);
				set_pen_color(hpanel, PANEL_CANVAS, 0x013025); //VAL_BLACK
			}

			if total_checked == 0 {
				let font_name = CString::new("Poppins UI").unwrap();
				create_meta_font(
					font_name.as_ptr(),
					font_name.as_ptr(),
					24, // point size
					0,  // bold
					0,  // italic
					0,  // underline
					0,  // strikeout
				);

				canvas_draw_text(
					hpanel,
					PANEL_CANVAS,
					CString::new(get_string_value(hpanel, PANEL_SELECTMSG))
						.unwrap()
						.as_ptr(),
					//CString::new("Top Left").unwrap().as_ptr(),
					font_name.as_ptr(),
					Rect { left: 20, top: 325, height: 36, width: 930 }, //630
					VAL_CENTER,
				);

				return; // Exit if no items are checked
			}

			let mut counter = 0;
			let mut max_class = 0;

			for i in 0..total_items {
				//First pass for autoscale
				let mut item_text: [c_char; 256] = [0; 256];

				let mut item_checked = 0;

				IsListItemChecked(
					hpanel,
					PANEL_LISTBOX as i32,
					i,
					&mut item_checked as *mut c_int,
				);
				if item_checked == 0 {
					continue; // Skip unchecked items
				}

				GetValueFromIndexUtf8(
					hpanel,
					PANEL_LISTBOX as i32,
					i,
					item_text.as_mut_ptr() as *mut std::ffi::c_void,
				);

				// Load the values from the TOML file
				// Assuming ChartValuesToml::load_from_toml is a function that loads
				let item_text_str =
					CStr::from_ptr(item_text.as_ptr()).to_string_lossy();
				let path_buf =
					PathBuf::from("favorites").join(item_text_str.as_ref());

				let path_str = path_buf.to_string_lossy();
				let values = ChartValuesToml::load_from_toml(&path_str);

				let iqi_type = values.iqi() as i32; // Use the public getter method for IQI type

				let isrb = values.isrb();
				let iso_mtl = values.mtl();
				let smtr = values.smtr();
				let snrn = values.snrn();
				let lag = values.lag();
				let csa = values.csa();

				let isrb_class = classify_value(isrb, iqi_type as usize, 0); // 0 = ISRB
				let csa_class = classify_value(csa, iqi_type as usize, 1); // 1 = CSA
				let lag_class = classify_value(lag, iqi_type as usize, 2); // 2 = LAG
				let snrn_class = classify_value(snrn, iqi_type as usize, 3); // 3 = SNRN
				let smtr_class = classify_value(smtr, iqi_type as usize, 4); // 4 = SMTR
				let iso_mtl_class =
					classify_value(iso_mtl, iqi_type as usize, 5); // 5 = ISO_MTL

				let classes: [usize; 6] = [
					isrb_class,
					csa_class,
					lag_class,
					snrn_class,
					smtr_class,
					iso_mtl_class,
				];

				let max_value = *classes.iter().max().unwrap();
				let auto_scale =
					get_numeric_value_i32(hpanel, PANEL_AUTOSCALESWITCH);

				let rounded_max = if auto_scale == 1 {
					round_up_to_nearest(max_value)
				} else {
					25
				};

				if max_class < rounded_max {
					max_class = rounded_max;
				}

				counter += 1;
			}

			counter = 0; // Reset counter for the next loop

			for i in 0..total_items {
				// Second pass for drawing
				let mut item_text: [c_char; 256] = [0; 256];

				let mut item_checked = 0;

				IsListItemChecked(
					hpanel,
					PANEL_LISTBOX as i32,
					i,
					&mut item_checked as *mut c_int,
				);
				if item_checked == 0 {
					continue; // Skip unchecked items
				}

				GetValueFromIndexUtf8(
					hpanel,
					PANEL_LISTBOX as i32,
					i,
					item_text.as_mut_ptr() as *mut std::ffi::c_void,
				);

				// Load the values from the TOML file
				// Assuming ChartValuesToml::load_from_toml is a function that loads
				let item_text_str =
					CStr::from_ptr(item_text.as_ptr()).to_string_lossy();
				let path_buf =
					PathBuf::from("favorites").join(item_text_str.as_ref());
				let path_str = path_buf.to_string_lossy();
				let values = ChartValuesToml::load_from_toml(&path_str);

				draw_spider_chart_compare(
					hpanel,
					values,
					counter as usize,
					item_text_str.as_ref(),
					max_class,
				);
				counter += 1;
			}
		}
	} else {
		// Otherwise, draw the single spider chart
		draw_spider_chart_single(hpanel);
	}
	//draw_spider_chart_single(hpanel);
}

pub fn clear_canvas(hpanel: c_int, _panel: c_int, background_color: u32) {
	let canvas_width = 950;
	let canvas_height = 720; // Adjusted canvas size 700

	let solid_default: [u8; 8] =
		[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];

	set_pattern(hpanel, PANEL_CANVAS, ATTR_PEN_PATTERN, &solid_default);

	set_attribute_u32(hpanel, PANEL_CANVAS, ATTR_PEN_MODE, VAL_COPY_MODE);

	canvas_clear(
		hpanel,
		PANEL_CANVAS,
		Rect {
			left: 0,
			top: 0,
			height: canvas_height,
			width: canvas_width + 25,
		},
	);

	set_attribute_u32(hpanel, PANEL_CANVAS, ATTR_PEN_MODE, VAL_COPY_MODE);

	set_attribute_u32(
		hpanel,
		PANEL_CANVAS,
		ATTR_PEN_FILL_COLOR,
		background_color,
	);

	set_attribute_u32(hpanel, PANEL_CANVAS, ATTR_PEN_COLOR, background_color);

	canvas_draw_rect(
		hpanel,
		PANEL_CANVAS,
		Rect {
			left: 0,
			top: 0,
			height: canvas_height,
			width: canvas_width + 25,
		},
		VAL_DRAW_INTERIOR,
	);
}

fn round_up_to_nearest(value: usize) -> usize {
	for &threshold in &[10, 15, 20, 25] {
		if value <= threshold {
			return threshold;
		}
	}
	value // If value > 25, return as is or handle differently
}

pub fn draw_spider_chart_single(hpanel: c_int) {
	// Translate the values to class 0 to 25
	let iqi_type = get_numeric_value_i32(hpanel, PANEL_IQI_TYPE);

	let isrb = get_numeric_value(hpanel, PANEL_ISRB);
	let iso_mtl = get_numeric_value(hpanel, PANEL_ISOMTL);
	let smtr = get_numeric_value(hpanel, PANEL_SMTR);
	let snrn = get_numeric_value(hpanel, PANEL_SNRN);
	let lag = get_numeric_value(hpanel, PANEL_LAG);
	let csa = get_numeric_value(hpanel, PANEL_CSA);

	let isrb_class = classify_value(isrb, iqi_type as usize, 0); // 0 = ISRB
	let csa_class = classify_value(csa, iqi_type as usize, 1); // 1 = CSA
	let lag_class = classify_value(lag, iqi_type as usize, 2); // 2 = LAG
	let snrn_class = classify_value(snrn, iqi_type as usize, 3); // 3 = SNRN
	let smtr_class = classify_value(smtr, iqi_type as usize, 4); // 4 = SMTR
	let iso_mtl_class = classify_value(iso_mtl, iqi_type as usize, 5); // 5 = ISO_MTL

	// Set control values as strings
	set_ctrl_val_str(hpanel, PANEL_ISRB_CLASS, &isrb_class.to_string());
	set_ctrl_val_str(hpanel, PANEL_CSA_CLASS, &csa_class.to_string());
	set_ctrl_val_str(hpanel, PANEL_LAG_CLASS, &lag_class.to_string());
	set_ctrl_val_str(hpanel, PANEL_SNRN_CLASS, &snrn_class.to_string());
	set_ctrl_val_str(hpanel, PANEL_SMTR_CLASS, &smtr_class.to_string());
	set_ctrl_val_str(hpanel, PANEL_ISOMTL_CLASS, &iso_mtl_class.to_string());

	let classes: [usize; 6] = [
		isrb_class,
		csa_class,
		lag_class,
		snrn_class,
		smtr_class,
		iso_mtl_class,
	];

	let max_value = *classes.iter().max().unwrap();
	let auto_scale = get_numeric_value_i32(hpanel, PANEL_AUTOSCALESWITCH);

	let rounded_max =
		if auto_scale == 1 { round_up_to_nearest(max_value) } else { 25 };

	let max_value = rounded_max as f64; // Use the rounded max value

	fn hex_corners(center_x: f64, center_y: f64, size: f64) -> [(f64, f64); 6] {
		let mut corners = [(0.0, 0.0); 6];
		for i in 0..6 {
			let angle = std::f64::consts::PI / 3.0 * i as f64
				+ std::f64::consts::PI / 6.0;
			corners[i] =
				(center_x + size * angle.cos(), center_y + size * angle.sin());
		}
		corners
	}
	reset_menu_bar();
	let theme = get_numeric_value_i32(hpanel, PANEL_THEME);

	let (
		background_color,
		text_color,
		text_accent_color,
		spider_color,
		axis,
		pride,
	): (u32, u32, u32, u32, bool, bool) = match theme {
		1 => (
			//Dark Teal Theme
			0x1c1c1c, // Dark background
			0xFFFFFF, // Whitetext
			0xFFFFFF, // Dark Teal accent
			0xf39200, // Orange spider
			false, false,
		),
		2 => (
			// Light with Axis
			0xFFFFFF, // Light background
			0x013025, // Dark Teal text
			0x013025, // Dark Teal accent
			0x009fda, // Cyan spider
			true, false,
		),
		3 => (
			0x1c1c1c, // Dark background
			0xFFFFFF, // Whitetext
			0xFFFFFF, // Dark Teal accent
			0xf39200, // Orange spider
			true, false,
		),
		4 => (
			0xFFFFFF, // Light background
			0x013025, // Dark Teal text
			0x013025, // Dark Teal accent
			0x009fda, // Cyan spider
			true, true,
		),

		_ => (
			//Light (default)
			0xFFFFFF, // Light background
			0x013025, // Dark Teal text
			0x013025, // Dark Teal accent
			0x009fda, // Cyan spider
			false, false,
		),
	};

	let canvas_width = 950;
	let canvas_height = 720; // Adjusted canvas size 700
	let center_x = canvas_width as f64 / 2.0;
	let center_y = canvas_height as f64 / 2.0;

	clear_canvas(hpanel, PANEL_CANVAS as i32, background_color);

	// Set drawing attributes once
	set_attribute_u32(hpanel, PANEL_CANVAS, ATTR_PEN_WIDTH, 2);
	set_attribute_u32(hpanel, PANEL_CANVAS, ATTR_ENABLE_ANTI_ALIASING, 1);

	// Draw nested hexagons
	for i in 0..5 {
		let hex_height = 500.0 - i as f64 * 100.0;
		let size = hex_height / 2.0;
		let corners = hex_corners(center_x, center_y, size);

		let mut point_array = [make_point_i32(0, 0); 6];
		for (j, &(x, y)) in corners.iter().enumerate() {
			point_array[j] = make_point_f64(x, y);
		}

		set_pen_color(hpanel, PANEL_CANVAS, spider_color);

		canvas_draw_poly(
			hpanel as i32,
			PANEL_CANVAS,
			6,
			point_array.as_ptr(),
			1, // wrap
			VAL_DRAW_FRAME,
		);
	}

	// Your normalized values (0 to 25), starting from top and going counter-clockwise
	// let values: [f64; 6] = [5.0, 18.0, 10.0, 11.0, 20.0, 13.0]; // example values

	fn rotate_left(values: [usize; 6], n: usize) -> [f64; 6] {
		let mut rotated = [0.0; 6];
		for i in 0..6 {
			rotated[i] = values[(i + n) % 6] as f64; // Convert usize to f64
		}
		rotated
	}

	let rotated = rotate_left(classes, 2); // rotate 2 times, our iSRb is  topmost

	let max_radius = 250.0; // half of 500 pixels

	// Center of the canvas
	let center_x = canvas_width as f64 / 2.0;
	let center_y = canvas_height as f64 / 2.0;

	// Compute spider chart points
	let mut spider_points = [make_point_i32(0, 0); 6];
	for i in 0..6 {
		let angle =
			std::f64::consts::PI / 3.0 * i as f64 + std::f64::consts::PI / 6.0;
		let radius = (rotated[i] / max_value) * max_radius;
		let x = center_x + radius * angle.cos();
		let y = center_y + radius * angle.sin();
		spider_points[i] = make_point_f64(x, y);
	}

	// Set drawing attributes for the spider chart
	set_pen_color(hpanel, PANEL_CANVAS, spider_color);
	set_pen_width(hpanel, PANEL_CANVAS, 2);

	set_ctrl_attribute_color(
		hpanel,
		PANEL_CANVAS,
		ATTR_PEN_FILL_COLOR,
		spider_color,
	);

	let pattern = get_numeric_value_i32(hpanel, PANEL_PATTERNSWITCH);

	// If pattern is enabled, draw with a checkerboard pattern
	let checker_pat: [u8; 8] = if pattern == 1 {
		[0xEF, 0xEF, 0xEF, 0xEF, 0xFE, 0xFE, 0xFE, 0xFE]
	} else {
		[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]
	};

	let mut complement_pat = [0u8; 8];
	for i in 0..8 {
		complement_pat[i] = !checker_pat[i];
	}

	set_pen_width(hpanel, PANEL_CANVAS, 1);

	set_pen_pattern(hpanel, PANEL_CANVAS, &checker_pat);

	// First pass: draw with complemented pattern and white fill
	set_pen_pattern(hpanel, PANEL_CANVAS, &complement_pat);

	set_pen_mode(hpanel, PANEL_CANVAS, VAL_COPY_MODE);

	set_ctrl_attribute_color(
		hpanel,
		PANEL_CANVAS,
		ATTR_PEN_FILL_COLOR,
		VAL_WHITE,
	);
	canvas_draw_poly(
		hpanel,
		PANEL_CANVAS,
		6,
		spider_points.as_ptr(),
		1,
		VAL_DRAW_FRAME_AND_INTERIOR,
	);

	// Second pass: draw with original pattern and desired color
	set_pen_pattern(hpanel, PANEL_CANVAS, &checker_pat);

	set_pen_mode(hpanel, PANEL_CANVAS, VAL_OR_MODE);

	set_ctrl_attribute_color(
		hpanel,
		PANEL_CANVAS,
		ATTR_PEN_FILL_COLOR,
		spider_color,
	);
	canvas_draw_poly(
		hpanel,
		PANEL_CANVAS,
		6,
		spider_points.as_ptr(),
		1,
		VAL_DRAW_FRAME_AND_INTERIOR,
	);

	// Center of the canvas
	let center_x = canvas_width as f64 / 2.0;
	let center_y = canvas_height as f64 / 2.0;

	// Outer hexagon size (500 height → 250 radius)
	let outer_radius = 500.0 / 2.0;

	// Get corners of the outer hexagon
	let outer_corners = hex_corners(center_x, center_y, outer_radius);

	// Set gray color for axis lines
	set_ctrl_attribute_color(hpanel, PANEL_CANVAS, ATTR_PEN_COLOR, VAL_GRAY);
	set_ctrl_attribute_color(hpanel, PANEL_CANVAS, ATTR_PEN_WIDTH, 2);

	// Draw lines from center to each corner
	for &(x, y) in &outer_corners {
		let point_val1 = make_point_f64(center_x, center_y);
		let point_val2 = make_point_f64(x, y);
		canvas_draw_line(hpanel, PANEL_CANVAS, point_val1, point_val2);
	}

	// Draw nested hexagons over the axis lines
	if axis {
		for i in 0..5 {
			let hex_height = 500.0 - i as f64 * 100.0;
			let size = hex_height / 2.0;
			let corners = hex_corners(center_x, center_y, size);

			let mut point_array = [make_point_i32(0, 0); 6];
			for (j, &(x, y)) in corners.iter().enumerate() {
				point_array[j] = make_point_f64(x, y);
			}

			if pride {
				// Optional: change color per hexagon
				let color = match i {
					0 => 0x00cc66, // green
					1 => 0x009fda, // cyan
					2 => 0xffcc00, // yellow
					3 => 0xff6600, // orange
					_ => 0xcc0033, // red
				};

				set_ctrl_attribute_color(
					hpanel,
					PANEL_CANVAS,
					ATTR_PEN_COLOR,
					color,
				);
			} else {
				set_ctrl_attribute_color(
					hpanel,
					PANEL_CANVAS,
					ATTR_PEN_COLOR,
					0x007f7f, //spider_color,
				);
			}

			canvas_draw_poly(
				hpanel,
				PANEL_CANVAS,
				6,
				point_array.as_ptr(),
				1, // wrap
				VAL_DRAW_FRAME,
			);
		}
	}

	// Draw labels for each axis
	set_ctrl_attribute_color(
		hpanel,
		PANEL_CANVAS,
		ATTR_PEN_FILL_COLOR,
		VAL_TRANSPARENT, //background_color as i32,
	);

	let radii = [0, 50, 100, 150, 200, 250];

	let divider = match rounded_max {
		25 => 10,
		20 => 12,
		15 => 16,
		10 => 25,
		_ => 25, // Default fallback
	};

	for &r in &radii {
		let label = CString::new(format!("{}", r / divider)).unwrap();

		// Define a small rectangle around the label position
		let rect = Rect {
			left: (center_x - 58.0) as i32,
			top: (center_y - r as f64 - 16.0) as i32,
			height: 32,
			width: 32,
		};

		let font_name = CString::new("Poppins UI").unwrap();
		create_meta_font(
			font_name.as_ptr(),
			font_name.as_ptr(),
			22, // point size
			0,  // bold
			0,  // italic
			0,  // underline
			0,  // strikeout
		);
		// https://www.ni.com/docs/de-DE/bundle/labwindows-cvi/page/cvi/uiref/cvicreatemetafont.htm

		set_pen_color(
			hpanel,
			PANEL_CANVAS,
			text_color, //VAL_BLACK,
		);

		canvas_draw_text(
			hpanel,
			PANEL_CANVAS,
			label.as_ptr(),
			font_name.as_ptr(), //VAL_APP_META_FONT,
			rect,
			VAL_CENTER_RIGHT,
		);

		//https://www.ni.com/docs/de-DE/bundle/labwindows-cvi/page/cvi/uiref/cvicanvasdrawtext.htm
	}

	// Draw labels for each axis
	let font_name = CString::new("Poppins UI").unwrap();
	create_meta_font(
		font_name.as_ptr(),
		font_name.as_ptr(),
		24, // point size
		0,  // bold
		0,  // italic
		0,  // underline
		0,  // strikeout
	);
	// https://www.ni.com/docs/de-DE/bundle/labwindows-cvi/page/cvi/uiref/cvicreatemetafont.htm

	let font_name_small = CString::new("Poppins UI").unwrap();

	// Draw labels for each axis
	set_pen_color(
		hpanel,
		PANEL_CANVAS,
		text_color, //VAL_BLACK,
	);

	/*
	// #ToDo: Implement localized decimal separator
	use std::ffi::OsString;
	use std::os::windows::ffi::OsStringExt;
	use std::ptr;
	use winapi::um::winnls::{GetLocaleInfoW, LOCALE_USER_DEFAULT, LOCALE_SDECIMAL};

	fn get_decimal_separator() -> String {
		let mut buffer = [0u16; 4]; // Enough for "." or ","
		unsafe {
			let len = GetLocaleInfoW(
				LOCALE_USER_DEFAULT,
				LOCALE_SDECIMAL,
				buffer.as_mut_ptr(),
				buffer.len() as i32,
			);
			if len > 0 {
				OsString::from_wide(&buffer[..(len - 1)]).to_string_lossy().into_owned()
			} else {
				".".to_string() // fallback
			}
		}
	}

	fn format_localized_float(value: f64) -> String {
		let separator = get_decimal_separator();
		let formatted = format!("{:.2}", value);
		if separator != "." {
			formatted.replace(".", &separator)
		} else {
			formatted
		}
	}
	// ---
	let cSA_value = get_numeric_value(hpanel, PANEL_CSA as i32);
	let formatted_value = format_localized_float(cSA_value);
	let cSA_label = CString::new(format!("CSa = {} %", formatted_value)).unwrap();
	*/

	set_pen_fill_color(hpanel, PANEL_CANVAS, background_color);

	let isrb_val = get_numeric_value(hpanel, PANEL_ISRB);
	let isrb_label = CString::new(format!(
		"iSR              = {} µm",
		format_float(isrb_val)
	))
	.unwrap();

	// Define a small rectangle around the label position
	canvas_draw_text(
		hpanel,
		PANEL_CANVAS,
		isrb_label.as_ptr(),
		font_name.as_ptr(),
		Rect { left: 316, top: 60, height: 32, width: 316 },
		VAL_CENTER,
	);

	let mut isrb_text_length: i32 = 0;
	let mut isrb_text_height: i32 = 0;

	get_text_display_size(
		isrb_label.as_ptr(),
		font_name.as_ptr(),
		&mut isrb_text_height,
		&mut isrb_text_length,
	);

	create_meta_font(
		font_name_small.as_ptr(),
		font_name_small.as_ptr(),
		16, // point size
		0,  // bold
		0,  // italic
		0,  // underline
		0,  // strikeout
	);

	canvas_draw_text(
		hpanel,
		PANEL_CANVAS,
		CString::new("b").unwrap().as_ptr(),
		font_name_small.as_ptr(),
		Rect {
			left: 518 - (isrb_text_length / 2) - 10, //move to opposite direction 211 base length then 15 pix per char
			top: isrb_text_height + 42,
			height: 32,
			width: 11,
		},
		VAL_CENTER,
	);

	canvas_draw_text(
		hpanel,
		PANEL_CANVAS,
		CString::new("detector").unwrap().as_ptr(),
		font_name_small.as_ptr(),
		Rect {
			left: 518 - (isrb_text_length / 2) - 20 + 20, //move to opposite direction 211 base length then 15 pix per char
			top: isrb_text_height + 24,
			height: 32,
			width: 70,
		},
		VAL_CENTER,
	);

	create_meta_font(
		font_name.as_ptr(),
		font_name.as_ptr(),
		24, // point size
		0,  // bold
		0,  // italic
		0,  // underline
		0,  // strikeout
	);

	let iso_mtl_value = get_numeric_value(hpanel, PANEL_ISOMTL);
	let iso_mtl_label =
		CString::new(format!("ISO MTL = {} mm", format_float(iso_mtl_value)))
			.unwrap();

	canvas_draw_text(
		hpanel,
		PANEL_CANVAS,
		iso_mtl_label.as_ptr(),
		font_name.as_ptr(),
		Rect { left: 10, top: 210, height: 32, width: 240 },
		VAL_CENTER_RIGHT,
	);

	let smtr_value = get_numeric_value(hpanel, PANEL_SMTR);
	let smtr_label =
		CString::new(format!("SMTR = {} mm", format_float(smtr_value)))
			.unwrap();
	canvas_draw_text(
		hpanel,
		PANEL_CANVAS,
		smtr_label.as_ptr(),
		font_name.as_ptr(),
		Rect { left: 10, top: 470, height: 32, width: 240 },
		VAL_CENTER_RIGHT,
	);

	let csa_value = get_numeric_value(hpanel, PANEL_CSA);
	let csa_label =
		CString::new(format!("CSa = {} %", format_float(csa_value))).unwrap();

	canvas_draw_text(
		hpanel,
		PANEL_CANVAS,
		csa_label.as_ptr(),
		font_name.as_ptr(),
		Rect { left: 700, top: 210, height: 32, width: 260 },
		VAL_CENTER_LEFT,
	);

	let lag_value = get_numeric_value(hpanel, PANEL_LAG);
	let lag_label =
		CString::new(format!("Image Lag = {} %", format_float(lag_value)))
			.unwrap();

	canvas_draw_text(
		hpanel,
		PANEL_CANVAS,
		lag_label.as_ptr(),
		font_name.as_ptr(),
		Rect { left: 700, top: 470, height: 32, width: 260 },
		VAL_CENTER_LEFT,
	);

	let snrn_value = get_numeric_value(hpanel, PANEL_SNRN);
	let snrn_label =
		CString::new(format!("SNR  = {} @ 1 mGy", format_float(snrn_value)))
			.unwrap();
	canvas_draw_text(
		hpanel,
		PANEL_CANVAS,
		snrn_label.as_ptr(),
		font_name.as_ptr(),
		Rect { left: 316, top: 620, height: 32, width: 316 },
		VAL_CENTER,
	);
	let mut snr_text_length: i32 = 0;
	let mut snr_text_height: i32 = 0;

	get_text_display_size(
		snrn_label.as_ptr(),
		font_name.as_ptr(),
		&mut snr_text_height,
		&mut snr_text_length,
	);
	// https://www.ni.com/docs/de-DE/bundle/labwindows-cvi/page/cvi/uiref/cvigettextdisplaysize.htm

	create_meta_font(
		font_name_small.as_ptr(),
		font_name_small.as_ptr(),
		16, // point size
		0,  // bold
		0,  // italic
		0,  // underline
		0,  // strikeout
	);

	canvas_draw_text(
		hpanel,
		PANEL_CANVAS,
		CString::new("N").unwrap().as_ptr(),
		font_name_small.as_ptr(),
		Rect {
			left: 518 - (snr_text_length / 2), //move to opposite direction
			top: snr_text_height + 600,
			height: 32,
			width: 11,
		},
		VAL_CENTER,
	);

	create_meta_font(
		font_name.as_ptr(),
		font_name.as_ptr(),
		24, // point size
		0,  // bold
		0,  // italic
		0,  // underline
		0,  // strikeout
	);

	set_pen_color(hpanel, PANEL_CANVAS, text_accent_color);

	let detector_value = get_string_value(hpanel, PANEL_DETECTOR_STRING);
	let detector_string =
		CString::new(format!("DDA: {}", detector_value)).unwrap();
	create_meta_font(
		font_name.as_ptr(),
		font_name.as_ptr(),
		24, // point size
		1,  // bold
		0,  // italic
		0,  // underline
		0,  // strikeout
	);

	canvas_draw_text(
		hpanel,
		PANEL_CANVAS,
		detector_string.as_ptr(),
		font_name.as_ptr(),
		Rect { left: 20, top: 10, height: 36, width: 630 },
		VAL_CENTER_LEFT,
	);

	canvas_draw_text(
		hpanel,
		PANEL_CANVAS,
		CString::new(match get_numeric_value_i32(hpanel, PANEL_IQI_TYPE) {
			0 => "Aluminium 6061/7022",
			1 => "Stainless Steel 316L",
			2 => "Inconel 719",
			3 => "Titanium Ti-6Al-4V",
			_ => "IQI Type: Unknown",
		})
		.unwrap()
		.as_ptr(),
		font_name.as_ptr(),
		Rect { left: 640, top: 10, height: 36, width: 300 },
		VAL_CENTER_RIGHT,
	);

	let timing_str = get_string_value(hpanel, PANEL_DETECTOR_TIMING);
	if !timing_str.trim().is_empty() {
		let timing_text = format!("Timing: {}", timing_str);
		canvas_draw_text(
			hpanel,
			PANEL_CANVAS,
			CString::new(timing_text).unwrap().as_ptr(),
			font_name.as_ptr(),
			Rect { left: 20, top: 660, height: 36, width: canvas_width / 2 },
			VAL_CENTER_LEFT,
		);
	}

	let gain_str = get_string_value(hpanel, PANEL_DETECTOR_GAIN);
	if !gain_str.trim().is_empty() {
		let gain_text = format!("Gain: {}", gain_str);
		canvas_draw_text(
			hpanel,
			PANEL_CANVAS,
			CString::new(gain_text).unwrap().as_ptr(),
			font_name.as_ptr(),
			Rect {
				left: canvas_width / 2,
				top: 660,
				height: 36,
				width: canvas_width / 2,
			},
			VAL_CENTER_RIGHT,
		);
	}
}

pub fn draw_spider_chart_compare(
	hpanel: c_int,
	values: ChartValuesToml,
	index: usize,
	item_text: &str,
	max_class: usize,
) {
	// Get IQI type from values (if present), otherwise fallback to control if needed
	// Replace with the correct field name for IQI type in ChartValuesToml, e.g. values.iqi_type_value or fallback to a default
	let iqi_type = values.iqi() as i32; // Use the public getter method for IQI type

	let isrb = values.isrb();
	let iso_mtl = values.mtl();
	let smtr = values.smtr();
	let snrn = values.snrn();
	let lag = values.lag();
	let csa = values.csa();

	let isrb_class = classify_value(isrb, iqi_type as usize, 0); // 0 = ISRB
	let csa_class = classify_value(csa, iqi_type as usize, 1); // 1 = CSA
	let lag_class = classify_value(lag, iqi_type as usize, 2); // 2 = LAG
	let snrn_class = classify_value(snrn, iqi_type as usize, 3); // 3 = SNRN
	let smtr_class = classify_value(smtr, iqi_type as usize, 4); // 4 = SMTR
	let iso_mtl_class = classify_value(iso_mtl, iqi_type as usize, 5); // 5 = ISO_MTL

	let classes: [usize; 6] = [
		isrb_class,
		csa_class,
		lag_class,
		snrn_class,
		smtr_class,
		iso_mtl_class,
	];
	/*
		fn round_up_to_nearest(value: usize) -> usize {
			for &threshold in &[10, 15, 20, 25] {
				if value <= threshold {
					return threshold;
				}
			}
			value // If value > 25, return as is or handle differently
		}

		let max_value = *classes.iter().max().unwrap();
		//let auto_scale = get_numeric_value_i32(hpanel, PANEL_AUTOSCALESWITCH);
		let auto_scale = 0; // For comparison, we can set this to 0 to disable auto-scaling
		let rounded_max =
			if auto_scale == 1 { round_up_to_nearest(max_value) } else { 25 };
	*/
	let max_value = max_class as f64; // Use the rounded max value
	let rounded_max = max_class as i32;

	fn hex_corners(center_x: f64, center_y: f64, size: f64) -> [(f64, f64); 6] {
		let mut corners = [(0.0, 0.0); 6];
		for i in 0..6 {
			let angle = std::f64::consts::PI / 3.0 * i as f64
				+ std::f64::consts::PI / 6.0;
			corners[i] =
				(center_x + size * angle.cos(), center_y + size * angle.sin());
		}
		corners
	}
	reset_menu_bar();
	let theme = get_numeric_value_i32(hpanel, PANEL_THEME);

	let (
		_background_color,
		text_color,
		_text_accent_color,
		spider_color,
		axis,
		pride,
	): (u32, u32, u32, u32, bool, bool) = match theme {
		1 => (
			//Dark Teal Theme
			0x1c1c1c, // Dark background
			0xFFFFFF, // Whitetext
			0xFFFFFF, // Dark Teal accent
			0xf39200, // Orange spider
			false, false,
		),
		2 => (
			// Light with Axis
			0xFFFFFF, // Light background
			0x013025, // Dark Teal text
			0x013025, // Dark Teal accent
			0x009fda, // Cyan spider
			true, false,
		),
		3 => (
			0x1c1c1c, // Dark background
			0xFFFFFF, // Whitetext
			0xFFFFFF, // Dark Teal accent
			0xf39200, // Orange spider
			true, false,
		),
		4 => (
			0xFFFFFF, // Light background
			0x013025, // Dark Teal text
			0x013025, // Dark Teal accent
			0x009fda, // Cyan spider
			true, true,
		),

		_ => (
			//Light (default)
			0xFFFFFF, // Light background
			0x013025, // Dark Teal text
			0x013025, // Dark Teal accent
			0x009fda, // Cyan spider
			false, false,
		),
	};

	let canvas_width = 950;
	let canvas_height = 720; // Adjusted canvas size 700
	let center_x = canvas_width as f64 / 2.0;
	let center_y = canvas_height as f64 / 2.0;

	let solid_default: [u8; 8] =
		[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];

	set_pattern(hpanel, PANEL_CANVAS, ATTR_PEN_PATTERN, &solid_default);

	set_attribute_u32(hpanel, PANEL_CANVAS, ATTR_PEN_MODE, VAL_COPY_MODE);

	// Set drawing attributes once
	set_attribute_u32(hpanel, PANEL_CANVAS, ATTR_PEN_WIDTH, 2);
	set_attribute_u32(hpanel, PANEL_CANVAS, ATTR_ENABLE_ANTI_ALIASING, 1);

	// Draw nested hexagons
	for i in 0..5 {
		let hex_height = 500.0 - i as f64 * 100.0;
		let size = hex_height / 2.0;
		let corners = hex_corners(center_x, center_y, size);

		let mut point_array = [make_point_i32(0, 0); 6];
		for (j, &(x, y)) in corners.iter().enumerate() {
			point_array[j] = make_point_f64(x, y);
		}

		set_pen_color(hpanel, PANEL_CANVAS, spider_color);
		set_pen_width(hpanel, PANEL_CANVAS, 1);
		canvas_draw_poly(
			hpanel as i32,
			PANEL_CANVAS,
			6,
			point_array.as_ptr(),
			1, // wrap
			VAL_DRAW_FRAME,
		);
	}

	// Your normalized values (0 to 25), starting from top and going counter-clockwise
	// let values: [f64; 6] = [5.0, 18.0, 10.0, 11.0, 20.0, 13.0]; // example values

	fn rotate_left(values: [usize; 6], n: usize) -> [f64; 6] {
		let mut rotated = [0.0; 6];
		for i in 0..6 {
			rotated[i] = values[(i + n) % 6] as f64; // Convert usize to f64
		}
		rotated
	}

	let rotated = rotate_left(classes, 2); // rotate 2 times, our iSRb is  topmost

	let max_radius = 250.0; // half of 500 pixels

	// Center of the canvas
	let center_x = canvas_width as f64 / 2.0;
	let center_y = canvas_height as f64 / 2.0;

	// Compute spider chart points
	let mut spider_points = [make_point_i32(0, 0); 6];
	for i in 0..6 {
		let angle =
			std::f64::consts::PI / 3.0 * i as f64 + std::f64::consts::PI / 6.0;
		let radius = (rotated[i] / max_value) * max_radius;
		let x = center_x + radius * angle.cos();
		let y = center_y + radius * angle.sin();
		spider_points[i] = make_point_f64(x, y);
	}

	// Set drawing attributes for the spider chart

	let poly_color = match index {
		0 => 0x00cc66, // green
		1 => 0x009fda, // cyan
		2 => 0xffcc00, // yellow
		3 => 0xff6600, // orange
		_ => 0xcc0033, // red
	};

	set_pen_color(hpanel, PANEL_CANVAS, poly_color); //was spider_color
	set_pen_width(hpanel, PANEL_CANVAS, 2);

	set_ctrl_attribute_color(
		hpanel,
		PANEL_CANVAS,
		ATTR_PEN_FILL_COLOR,
		spider_color,
	);

	//let pattern = get_numeric_value_i32(hpanel, PANEL_PATTERNSWITCH);
	let pattern = 0; // For comparison, we can set this to 0 to disable patterns
	// If pattern is enabled, draw with a checkerboard pattern
	let checker_pat: [u8; 8] = if pattern == 1 {
		[0xEF, 0xEF, 0xEF, 0xEF, 0xFE, 0xFE, 0xFE, 0xFE]
	} else {
		[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]
	};

	let mut complement_pat = [0u8; 8];
	for i in 0..8 {
		complement_pat[i] = !checker_pat[i];
	}

	set_pen_width(hpanel, PANEL_CANVAS, 2);

	set_pen_pattern(hpanel, PANEL_CANVAS, &checker_pat);

	// First pass: draw with complemented pattern and white fill
	set_pen_pattern(hpanel, PANEL_CANVAS, &complement_pat);

	set_pen_mode(hpanel, PANEL_CANVAS, VAL_COPY_MODE);

	set_ctrl_attribute_color(
		hpanel,
		PANEL_CANVAS,
		ATTR_PEN_FILL_COLOR,
		VAL_WHITE,
	);
	canvas_draw_poly(
		hpanel,
		PANEL_CANVAS,
		6,
		spider_points.as_ptr(),
		1,
		VAL_DRAW_FRAME,
	);

	// Center of the canvas
	let center_x = canvas_width as f64 / 2.0;
	let center_y = canvas_height as f64 / 2.0;

	// Outer hexagon size (500 height → 250 radius)
	let outer_radius = 500.0 / 2.0;

	// Get corners of the outer hexagon
	let outer_corners = hex_corners(center_x, center_y, outer_radius);

	// Set gray color for axis lines
	set_ctrl_attribute_color(hpanel, PANEL_CANVAS, ATTR_PEN_COLOR, VAL_GRAY);
	set_ctrl_attribute_color(hpanel, PANEL_CANVAS, ATTR_PEN_WIDTH, 2);
	set_pen_width(hpanel, PANEL_CANVAS, 1);
	// Draw lines from center to each corner
	for &(x, y) in &outer_corners {
		let point_val1 = make_point_f64(center_x, center_y);
		let point_val2 = make_point_f64(x, y);
		canvas_draw_line(hpanel, PANEL_CANVAS, point_val1, point_val2);
	}

	// Draw nested hexagons over the axis lines
	if axis {
		for i in 0..5 {
			let hex_height = 500.0 - i as f64 * 100.0;
			let size = hex_height / 2.0;
			let corners = hex_corners(center_x, center_y, size);

			let mut point_array = [make_point_i32(0, 0); 6];
			for (j, &(x, y)) in corners.iter().enumerate() {
				point_array[j] = make_point_f64(x, y);
			}
			set_pen_width(hpanel, PANEL_CANVAS, 1);
			if pride {
				// Optional: change color per hexagon
				let color = match i {
					0 => 0x00cc66, // green
					1 => 0x009fda, // cyan
					2 => 0xffcc00, // yellow
					3 => 0xff6600, // orange
					_ => 0xcc0033, // red
				};

				set_ctrl_attribute_color(
					hpanel,
					PANEL_CANVAS,
					ATTR_PEN_COLOR,
					color,
				);
			} else {
				set_ctrl_attribute_color(
					hpanel,
					PANEL_CANVAS,
					ATTR_PEN_COLOR,
					0x007f7f, //spider_color,
				);
			}

			canvas_draw_poly(
				hpanel,
				PANEL_CANVAS,
				6,
				point_array.as_ptr(),
				1, // wrap
				VAL_DRAW_FRAME,
			);
		}
	}

	// Draw labels for each axis
	set_ctrl_attribute_color(
		hpanel,
		PANEL_CANVAS,
		ATTR_PEN_FILL_COLOR,
		VAL_TRANSPARENT, //background_color as i32,
	);

	let radii = [0, 50, 100, 150, 200, 250];

	let divider = match rounded_max {
		25 => 10,
		20 => 12,
		15 => 16,
		10 => 25,
		_ => 25, // Default fallback
	};

	for &r in &radii {
		let label = CString::new(format!("{}", r / divider)).unwrap();

		// Define a small rectangle around the label position
		let rect = Rect {
			left: (center_x - 58.0) as i32,
			top: (center_y - r as f64 - 16.0) as i32,
			height: 32,
			width: 32,
		};

		let font_name = CString::new("Poppins UI").unwrap();
		create_meta_font(
			font_name.as_ptr(),
			font_name.as_ptr(),
			22, // point size
			0,  // bold
			0,  // italic
			0,  // underline
			0,  // strikeout
		);
		// https://www.ni.com/docs/de-DE/bundle/labwindows-cvi/page/cvi/uiref/cvicreatemetafont.htm

		set_pen_color(
			hpanel,
			PANEL_CANVAS,
			text_color, //VAL_BLACK,
		);

		canvas_draw_text(
			hpanel,
			PANEL_CANVAS,
			label.as_ptr(),
			font_name.as_ptr(), //VAL_APP_META_FONT,
			rect,
			VAL_CENTER_RIGHT,
		);

		//https://www.ni.com/docs/de-DE/bundle/labwindows-cvi/page/cvi/uiref/cvicanvasdrawtext.htm
	}

	// Draw labels for each axis
	let font_name = CString::new("Poppins UI").unwrap();
	create_meta_font(
		font_name.as_ptr(),
		font_name.as_ptr(),
		24, // point size
		0,  // bold
		0,  // italic
		0,  // underline
		0,  // strikeout
	);
	// https://www.ni.com/docs/de-DE/bundle/labwindows-cvi/page/cvi/uiref/cvicreatemetafont.htm

	// Define a small rectangle around the label position

	create_meta_font(
		font_name.as_ptr(),
		font_name.as_ptr(),
		24, // point size
		0,  // bold
		0,  // italic
		0,  // underline
		0,  // strikeout
	);

	canvas_draw_text(
		hpanel,
		PANEL_CANVAS,
		CString::new("iSR              ").unwrap().as_ptr(),
		font_name.as_ptr(),
		Rect { left: 316, top: 60, height: 32, width: 316 },
		VAL_CENTER,
	);

	let mut isrb_text_length: i32 = 0;
	let mut isrb_text_height: i32 = 0;

	let isrb_label = CString::new("iSR              ").unwrap();

	get_text_display_size(
		isrb_label.as_ptr(),
		font_name.as_ptr(),
		&mut isrb_text_height,
		&mut isrb_text_length,
	);

	let font_name_small = CString::new("Poppins UI").unwrap();

	create_meta_font(
		font_name_small.as_ptr(),
		font_name_small.as_ptr(),
		16, // point size
		0,  // bold
		0,  // italic
		0,  // underline
		0,  // strikeout
	);

	canvas_draw_text(
		hpanel,
		PANEL_CANVAS,
		CString::new("b").unwrap().as_ptr(),
		font_name_small.as_ptr(),
		Rect {
			left: 518 - (isrb_text_length / 2) - 10, //move to opposite direction 211 base length then 15 pix per char
			top: isrb_text_height + 42,
			height: 32,
			width: 11,
		},
		VAL_CENTER,
	);

	canvas_draw_text(
		hpanel,
		PANEL_CANVAS,
		CString::new("detector").unwrap().as_ptr(),
		font_name_small.as_ptr(),
		Rect {
			left: 518 - (isrb_text_length / 2) - 20 + 20, //move to opposite direction 211 base length then 15 pix per char
			top: isrb_text_height + 24,
			height: 32,
			width: 70,
		},
		VAL_CENTER,
	);

	create_meta_font(
		font_name.as_ptr(),
		font_name.as_ptr(),
		24, // point size
		0,  // bold
		0,  // italic
		0,  // underline
		0,  // strikeout
	);

	canvas_draw_text(
		hpanel,
		PANEL_CANVAS,
		CString::new("ISO MTL").unwrap().as_ptr(),
		font_name.as_ptr(),
		Rect { left: 10, top: 210, height: 32, width: 240 },
		VAL_CENTER_RIGHT,
	);

	canvas_draw_text(
		hpanel,
		PANEL_CANVAS,
		CString::new("SMTR").unwrap().as_ptr(),
		font_name.as_ptr(),
		Rect { left: 10, top: 470, height: 32, width: 240 },
		VAL_CENTER_RIGHT,
	);

	canvas_draw_text(
		hpanel,
		PANEL_CANVAS,
		CString::new("CSa").unwrap().as_ptr(),
		font_name.as_ptr(),
		Rect { left: 700, top: 210, height: 32, width: 260 },
		VAL_CENTER_LEFT,
	);

	canvas_draw_text(
		hpanel,
		PANEL_CANVAS,
		CString::new("Image Lag").unwrap().as_ptr(),
		font_name.as_ptr(),
		Rect { left: 700, top: 470, height: 32, width: 260 },
		VAL_CENTER_LEFT,
	);

	canvas_draw_text(
		hpanel,
		PANEL_CANVAS,
		CString::new("SNR").unwrap().as_ptr(),
		font_name.as_ptr(),
		Rect { left: 316, top: 620, height: 32, width: 316 },
		VAL_CENTER,
	);
	let mut snr_text_length: i32 = 0;
	let mut snr_text_height: i32 = 0;

	let snrn_label = CString::new("SNR").unwrap(); // Add spaces to align with other labels

	get_text_display_size(
		snrn_label.as_ptr(),
		font_name.as_ptr(),
		&mut snr_text_height,
		&mut snr_text_length,
	);
	// https://www.ni.com/docs/de-DE/bundle/labwindows-cvi/page/cvi/uiref/cvigettextdisplaysize.htm

	create_meta_font(
		font_name_small.as_ptr(),
		font_name_small.as_ptr(),
		16, // point size
		0,  // bold
		0,  // italic
		0,  // underline
		0,  // strikeout
	);

	canvas_draw_text(
		hpanel,
		PANEL_CANVAS,
		CString::new("N").unwrap().as_ptr(),
		font_name_small.as_ptr(),
		Rect {
			left: 518 - (snr_text_length / 2) + 1, //move to opposite direction
			top: snr_text_height + 600,
			height: 32,
			width: 11,
		},
		VAL_CENTER,
	);

	let font_name_medium = CString::new("Poppins UI").unwrap();

	create_meta_font(
		font_name_medium.as_ptr(),
		font_name_medium.as_ptr(),
		20, // point size
		0,  // bold
		0,  // italic
		0,  // underline
		0,  // strikeout
	);

	// Draw labels for each corner
	set_pen_color(
		hpanel,
		PANEL_CANVAS,
		poly_color, //VAL_BLACK,
	);

	//item_text = item_text.trim();
	let mut item_text = item_text.trim();
	if let Some(stripped) = item_text.strip_suffix(".toml") {
		item_text = stripped.trim();
	}

	match index {
		0 => canvas_draw_text(
			hpanel,
			PANEL_CANVAS,
			CString::new(item_text).unwrap().as_ptr(),
			//CString::new("Top Left").unwrap().as_ptr(),
			font_name_medium.as_ptr(),
			Rect { left: 20, top: 10, height: 36, width: 470 }, //630
			VAL_CENTER_LEFT,
		),
		1 => canvas_draw_text(
			hpanel,
			PANEL_CANVAS,
			CString::new(item_text).unwrap().as_ptr(),
			//CString::new("Top Right").unwrap().as_ptr(),
			font_name_medium.as_ptr(),
			Rect { left: 480, top: 10, height: 36, width: 470 },
			VAL_CENTER_RIGHT,
		),
		3 => canvas_draw_text(
			hpanel,
			PANEL_CANVAS,
			CString::new(item_text).unwrap().as_ptr(),
			//CString::new("Bottom Right").unwrap().as_ptr(),
			font_name_medium.as_ptr(),
			Rect { left: 480, top: 660, height: 36, width: 470 }, //640
			VAL_CENTER_RIGHT,
		),
		2 => canvas_draw_text(
			hpanel,
			PANEL_CANVAS,
			CString::new(item_text).unwrap().as_ptr(),
			//CString::new("Bottom Left").unwrap().as_ptr(),
			font_name_medium.as_ptr(),
			Rect { left: 20, top: 660, height: 36, width: 470 },
			VAL_CENTER_LEFT,
		),
		_ => {}
	}
}
