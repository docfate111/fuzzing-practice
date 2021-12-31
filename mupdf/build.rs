use std::env;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/main.rs");
    let cwd = env::current_dir().unwrap().to_string_lossy().to_string();
    let mupdf_dir = format!("{}/mupdf", cwd);
    // make clean; remove any leftover gunk from prior builds
    Command::new("make")
            .arg("clean")
            .current_dir(mupdf_dir.clone())
            .status()
            .expect("Couldn't clean mupdf directory");
    // export LLVM_CONFIG=llvm-config-11
    env::set_var("LLVM_CONFIG", "llvm-config-11");
    // configure with afl-clang-fast and set install directory to ./mupdf/install
    /*no configure script Command::new("./configure")
            .arg(&format!("--prefix={}/install", mupdf_dir))
            .env("CC", "/usr/local/bin/afl-clang-fast")
            .current_dir(mupdf_dir.clone())
            .status()
            .expect("Couldn't configure to build mupdf with afl-clang-fast");
    */
    // make && make install
    Command::new("make")
            .current_dir(mupdf_dir.clone())
            .status()
            .expect("Couldn't make install")
            .arg("install")
            .current_dir(mupdf_dir)
            .status()
            .expect("Couldn't install");

}
