use std::process::Command;
use std::env;

fn main() {
    let build_enabled = env::var("BUILD_ENABLED")
        .map(|v| v == "1")
        .unwrap_or(true); // run by default
    let directory = env::var("CARGO_MANIFEST_DIR").unwrap();
    let target_dir = "target/aarch64-unknown-none/debug/";
    let binary   = format!("{}/{}{}", directory, target_dir, "shiny_salmon");
    let out_bin  = format!("{}/{}{}", directory, target_dir, "shiny_salmon.o");
    println!("{}\r\n{}", binary, out_bin);
    env::set_current_dir(directory).unwrap();
    if build_enabled {
        env::set_var("BUILD_ENABLED", "0");
        Command::new("cargo").args(&["clean"])
                            .status().unwrap();
        Command::new("cargo").args(&["build"])
                            .status().unwrap();
        Command::new("rust-objcopy").args(&["-O", "binary", &binary, &out_bin]).status().unwrap();
    }
}
