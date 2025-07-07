//use copy_to_output::copy_to_output;
//use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs, path::Path};

fn main() -> std::io::Result<()> {
	// Ensure build script reruns if the source file changes
	println!("cargo:rerun-if-changed=cvi/SpiderChart.uir");

	// Get the output directory (e.g., target/debug or target/release)
	let out_dir = env::var("OUT_DIR").unwrap();
	// OUT_DIR is something like .../target/debug/build/yourcrate-xxxxxx/out
	// To get to target/debug or target/release:
	let _profile = env::var("PROFILE").unwrap();
	let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
	let target_dir = Path::new(&out_dir)
		.ancestors()
		.nth(3) // Go up 3 levels: out -> build -> debug/release -> target
		.expect("Failed to determine target directory");

	// Construct the destination path: target/debug/bin/subfolder/SpiderChart.uir
	//let dest_dir = target_dir.join(&profile).join("bin").join("subfolder");
	let dest_dir = target_dir.join("bin");
	fs::create_dir_all(&dest_dir)
		.expect("Failed to create destination directory");

	let src_file = Path::new(&manifest_dir).join("cvi/SpiderChart.uir");
	let dest_file = dest_dir.join("SpiderChart.uir");
	fs::copy(&src_file, &dest_file).expect("Could not copy uir file");
	println!("src {:?}, dst {:?}", src_file, dest_file);
	println!("cargo:rerun-if-changed=cvi/SpiderChart.uir");
	//copy_to_output("cvi/SpiderChart.uir", &env::var("PROFILE").unwrap())
	//	.expect("Could not copy uir file");

	println!("cargo:rerun-if-changed=cvi/SpiderChart-EN.lwl");
	let src_file_en = Path::new(&manifest_dir).join("cvi/SpiderChart-EN.lwl");
	let dest_file_en = dest_dir.join("SpiderChart-EN.lwl");
	fs::copy(&src_file_en, &dest_file_en).expect("Could not copy uir file");

	//	copy_to_output("cvi/SpiderChart-EN.lwl", &env::var("PROFILE").unwrap())
	//		.expect("Could not copy lwl EN file");

	println!("cargo:rerun-if-changed=cvi/SpiderChart-DE.lwl");
	let src_file_de = Path::new(&manifest_dir).join("cvi/SpiderChart-DE.lwl");
	let dest_file_de = dest_dir.join("SpiderChart-DE.lwl");
	fs::copy(&src_file_de, &dest_file_de).expect("Could not copy uir file");

	//	copy_to_output("cvi/SpiderChart-DE.lwl", &env::var("PROFILE").unwrap())
	//		.expect("Could not copy lwl DE file");

	println!("cargo:rerun-if-changed=hlp/SpiderChart-EN.chm");
	let src_file_en = Path::new(&manifest_dir).join("hlp/SpiderChart-EN.chm");
	let dest_file_en = dest_dir.join("SpiderChart-EN.chm");
	fs::copy(&src_file_en, &dest_file_en).expect("Could not copy en chm file");

	//	copy_to_output("cvi/SpiderChart-EN.lwl", &env::var("PROFILE").unwrap())
	//		.expect("Could not copy lwl EN file");

	println!("cargo:rerun-if-changed=hlp/SpiderChart-DE.chm");
	let src_file_de = Path::new(&manifest_dir).join("hlp/SpiderChart-DE.chm");
	let dest_file_de = dest_dir.join("SpiderChart-DE.chm");
	fs::copy(&src_file_de, &dest_file_de).expect("Could not copy de chm file");

	let fav_dir = target_dir.join("favorites");
	fs::create_dir_all(&fav_dir).expect("Failed to create favorites directory");
	let src_toml =
		Path::new(&manifest_dir).join("Default ASTM E2597 Example.toml");
	let dst_toml = fav_dir.join("Default ASTM E2597 Example.toml");
	fs::copy(&src_toml, &dst_toml).expect("Could not copy default toml file");

	println!("cargo:rerun-if-changed=res/SpiderChart.res");

	let _out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

	// Compile the .rc file to .res
	let status = Command::new("windres")
		.args([
			"res/SpiderChart.rc",
			"-O",
			"coff",
			"-o",
			&format!("{}/SpiderChart.res", _out_dir.display()),
		])
		.status()
		.expect("Failed to run windres");

	if !status.success() {
		panic!("windres failed");
	}

	// Link the .res file
	println!(
		"cargo:rustc-link-arg={}",
		_out_dir.join("SpiderChart.res").display()
	);

	println!("cargo:rustc-link-arg=/NODEFAULTLIB:library");
	//println!("cargo:rustc-link-arg=SpiderChart.res");

	println!("cargo:rerun-if-changed=cvi/SpiderChart.h");

	let input_path = "cvi/SpiderChart.h";
	let output_path = "cvi/SpiderChartBind.h";

	// Handle the Result properly here
	let input_file = File::open(input_path)?;
	let reader = BufReader::new(input_file);

	let mut output_file = File::create(output_path)?;

	for line in reader.lines() {
		let line = line?;
		if line.trim_start().starts_with("#include")
			|| line.trim_start().starts_with("int  CVICALLBACK")
			|| line.trim_start().starts_with("void CVICALLBACK")
		{
			continue;
		}
		writeln!(output_file, "{}", line)?;
	}

	println!("Filtered content written to {}", output_path);

	let bindings = bindgen::Builder::default()
		.header("cvi/SpiderChartBind.h")
		.generate()
		.expect("Unable to generate bindings");

	let bindings_out_path = "src/spiderchart.rs";
	bindings
		.write_to_file(bindings_out_path)
		.expect("Couldn't write bindings!");

	// Read the file, prepend the attribute, and write it back
	let mut contents = std::fs::read_to_string(bindings_out_path)?;
	contents = format!("#![allow(dead_code)]\n{}", contents);
	std::fs::write(bindings_out_path, contents)?;

	println!("Updated file with #![allow(dead_code)] at the top.");
	// official ext support
	println!("cargo:rustc-link-lib=lib\\cvirt");
	println!("cargo:rustc-link-lib=lib\\cvisupp");
	println!("cargo:rustc-link-lib=lib\\localui");

	println!("cargo:rustc-link-arg=cvi/SpiderChart.obj");

	//println!("cargo:rustc-link-arg=lib\\ImageControl.obj");
	println!("cargo:rustc-link-arg=lib\\hyperlinkctrl.obj");
	println!("cargo:rustc-link-arg=lib\\inifile.obj");
	println!("cargo:rustc-link-arg=lib\\toolbox.obj");
	//println!("cargo:rustc-link-arg=lib\\asynctmr.obj");
	println!("cargo:rustc-link-lib=lib\\cviauto");
	println!("cargo:rustc-link-lib=lib\\cvi");
	println!("cargo:rustc-link-lib=lib\\cvistart");
	println!("cargo:rustc-link-lib=lib\\instrsup_start");

	//println!("cargo:rustc-link-lib=lib\\nivision");

	println!("cargo:rustc-link-lib=kernel32");
	println!("cargo:rustc-link-lib=user32");
	println!("cargo:rustc-link-lib=advapi32");
	println!("cargo:rustc-link-lib=gdi32");

	Ok(())
}
