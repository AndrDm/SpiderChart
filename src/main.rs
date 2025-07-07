//==============================================================================
//
// Title:		SpiderChart
// Purpose:		Drawing of the Spider Chart Diagram (Net Summary Plot)
//
// Created on:	25.06.2025 at 08:57:26 by AD.
//
//==============================================================================

mod chart_values_ini;
mod chart_values_toml;
mod localui;
mod main_callbacks;
mod main_utils;
mod spiderchart;
mod spiderchart_draw;
mod tables;
mod user_int;
mod user_int_ex;

use crate::{
	chart_values_ini::*, main_utils::*, spiderchart::*, spiderchart_draw::*,
	user_int_ex::*,
};
use hide_console::hide_console;
use std::{ffi::CString, os::raw::c_char};

//==============================================================================
// main function, entry point, cleaned by copilot
//
fn main() {
	println!("Hello, ASTM Spider Chart!");
	hide_console();
	load_cui_font();

	let c_argv = build_c_argv();

	if !init_runtime(&c_argv) {
		eprintln!("Failed to initialize CVIRTE.");
		return;
	}
	set_localized_decimal_symbol(true);

	let hpanel = match load_main_panel() {
		Some(panel) => panel,
		None => {
			eprintln!("Failed to load User Interface.");
			return;
		}
	};

	setup_panel(hpanel);

	let path = "SpiderChart.ini";
	let (ini_values, mut conf) = ChartValues::load_from_ini(path);
	ini_values.print();
	ini_values.set_controls(hpanel);

	change_language();
	populate_listbox_with_toml_files(hpanel, PANEL_LISTBOX);
	display_panel(hpanel);
	draw_spider_chart(hpanel);

	run_user_interface();

	let current_values = ChartValues::from_controls(hpanel);
	current_values.save_to_ini(&mut conf, path);

	discard_panel(hpanel);
	close_cvi_rte();
}

fn build_c_argv() -> Vec<*const c_char> {
	std::env::args()
		.map(|arg| CString::new(arg).unwrap())
		.map(|cstr| cstr.into_raw() as *const c_char)
		.collect()
}

fn init_runtime(c_argv: &[*const c_char]) -> bool {
	init_cvi_rte(0, c_argv.as_ptr(), 0) != 0
}

fn load_main_panel() -> Option<i32> {
	let panel = load_panel(
		0,
		CString::new("bin/SpiderChart.uir").unwrap().as_ptr(),
		PANEL,
	);
	change_language();
	if panel < 0 { None } else { Some(panel) }
}

fn setup_panel(hpanel: i32) {
	set_window_style(hpanel);
	enable_drag_and_drop(hpanel);
}

#[test]
fn test_classes() {
	assert_eq!(6, classify_value(240.0, 0, 0)); //iSRb 240.0 == Class 6
	assert_eq!(18, classify_value(0.05, 0, 1)); //CSa why 17?
	assert_eq!(10, classify_value(0.3, 0, 2)); //Lag
	assert_eq!(11, classify_value(820.0, 0, 3)); //SNRn
	assert_eq!(20, classify_value(125.0, 0, 4)); //SMTR
	assert_eq!(13, classify_value(90.0, 0, 5)); //MTL
}
