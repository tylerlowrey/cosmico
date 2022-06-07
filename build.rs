use std::env;
use std::path::Path;
use fs_extra::{dir, dir::CopyOptions};

const ASSETS_FOLDER_LOCATION: &str = "assets";

fn main() {
    println!("cargo:rerun-if-changed={}", ASSETS_FOLDER_LOCATION);
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let target_dir = Path::new(&manifest_dir_string).join("target").join(build_type);
    println!("cargo:warning=TARGET_DIR is {:?}", target_dir);

    let mut copy_options = CopyOptions::new();
    copy_options.overwrite = true;
    dir::copy(
        Path::new(ASSETS_FOLDER_LOCATION),
       target_dir,
        &copy_options
    ).expect("Unable to copy assets folder");
}