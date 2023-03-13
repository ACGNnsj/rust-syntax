
use ::std::{env, fs, os, path::{Path, PathBuf}};

fn main() {
   /* let out_dir = env::var("OUT_DIR").unwrap();
    let work_dir = vec!["../../..", "../../../deps"];
    work_dir.iter().for_each(|dir_path| symbolic_link_dll(&Path::new(&out_dir[..]).join(dir_path).canonicalize().unwrap()));*/
    // let build_path = Path::new(&env::var("OUT_DIR").unwrap()).join("../../..").canonicalize().unwrap();
    let path = Path::new("./lib").canonicalize().unwrap();
    // let files = fs::read_dir("./lib").unwrap();
    // for file in files {
    //     let file_path = file.unwrap().path();
    //     let filename = file_path.file_name().unwrap();
    //     let link = build_path.join(filename);
    //     if link.exists() {
    //         fs::remove_file(&link).expect("TODO: panic message");
    //     }
    //     os::windows::fs::symlink_file(path.join(filename), link).expect("TODO: panic message");
    // }
    // let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    // let path = Path::new(&dir).join("lib");
    println!("cargo:rustc-link-search={}", path.display());
}

#[cfg(windows)]
fn symbolic_link_dll(exe_dir: &PathBuf) {
    const DLL_FILE: &str = "auxiliaries_native.dll";
    let mut dll_origin = env::current_dir().unwrap();
    dll_origin.push("assets");
    dll_origin.push(DLL_FILE);
    if dll_origin.exists() {
        let dll_symbol = exe_dir.join(DLL_FILE);
        if dll_symbol.exists() {
            fs::remove_file(dll_symbol.clone()).unwrap();
        }
        os::windows::fs::symlink_file(dll_origin.clone(), dll_symbol.clone()).unwrap();
    }
}

#[cfg(not(windows))]
fn symbolic_link_dll(exe_dir: &PathBuf) {
    unreachable!("xxx");
}
