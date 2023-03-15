use std::{env, fs, os};
use std::path::Path;

#[test]
fn test() {
    println!("{}",env::var("OUT_DIR").unwrap());
    let path = Path::new("./");
    println!("{:?}", path);
    let path_buf = path.canonicalize().unwrap();
    println!("{:?}", path_buf);
    let path_str = path_buf.to_str().unwrap();
    println!("{:?}", path_str);
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("{:?}", dir);
    println!("{}", "G:\\clion\\rust-syntax\\rust-syntax-stable");
    println!("{:}", "G:\\clion\\rust-syntax\\rust-syntax-stable");
    let p = path.display();
    println!("{:?}", p);
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("----------------------------------");
    let path = Path::new(&dir).join("lib");
    println!("cargo:rustc-link-search={}", path.display());
    let path = Path::new("./lib").canonicalize().unwrap();
    println!("cargo:rustc-link-search={}", path.display());
    println!("{:?}", env::current_dir().unwrap());
    println!("{:?}", Path::new(env!("OUT_DIR")).join("../../..").canonicalize().unwrap());
    let build_path = Path::new(&env::var("OUT_DIR").unwrap()).join("../../..").canonicalize().unwrap();
    let paths = fs::read_dir("./lib").unwrap();
    for path in paths {
        let p = path.unwrap().path();
        println!("{:?}", p);
        println!("{:?}", p.file_name().unwrap());
    }
}

pub fn test_symlink() {
    println!("{:}", env!("OUT_DIR"));
    // let build_path = Path::new("./target/debug").canonicalize().unwrap();
    let build_path = Path::new(env!("OUT_DIR")).join("../../..").canonicalize().unwrap();
    println!("{:?}",build_path);
    // let path = Path::new("./lib").canonicalize().unwrap();
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("lib").canonicalize().unwrap();
    println!("{:?}", path);
    // let files = fs::read_dir("./lib").unwrap();
    let files = fs::read_dir(&path).unwrap();
    for file in files {
        let file_path = file.unwrap().path();
        let filename = file_path.file_name().unwrap();
        let link = build_path.join(filename);
        if link.exists() {
            fs::remove_file(&link).expect("TODO: panic message");
        }
        println!("{:?}", path.join(filename));
        println!("{:?}", link);
        os::windows::fs::symlink_file(path.join(filename), link).unwrap();
    }
}