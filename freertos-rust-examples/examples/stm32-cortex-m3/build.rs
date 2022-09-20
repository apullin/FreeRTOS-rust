use std::env;
use std::fs;
use std::fs::copy;
use std::path::PathBuf;

fn main() {
    // Allows to show relevant environment variables for debugging purpose
    print_env();

    let target = env::var("TARGET").unwrap_or_default();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    let target_family = env::var("CARGO_CFG_TARGET_FAMILY").unwrap_or_default();
    let out_dir = env::var("OUT_DIR").unwrap();

    let mut b = freertos_cargo_build::Builder::new();

    // Set path to FreeRTOS kernel sources
    b.freertos("../../FreeRTOS-Kernel" );

    // Set include path for FreeRTOSConfig.h
    let configHdr = fs::canonicalize(PathBuf::from("./FreeRTOSConfig.h") )
        .expect("Cannot determine path for FreeRTOSConfig.h");

    let configPath = configHdr.parent().unwrap();

    b.freertos_config(configPath );

    // Make a hardlink of the linker script in the output build dir, so cc linking will pick it up
    // TODO: This should use an absolute path, instead of creating a hardlink
    copy(
        "memory.x",
        PathBuf::from(out_dir.as_str()).join("memory.x"),
    ).unwrap();

    b.compile().unwrap_or_else(|e| { panic!(e.to_string()) });
}

/// Print relevant environment variables
fn print_env() {
    let env_keys = ["TARGET", "OUT_DIR", "HOST"];
    env::vars().for_each(|(key, val)| {
        if key.starts_with("CARGO") {
            println!("cargo:warning={}={}", key, val);
        } else if env_keys.contains(&key.as_str()) {
            println!("cargo:warning={}={}", key, val);
        } else {
            // println!("cargo:warning={}={}", key, val);
        }
    });
}