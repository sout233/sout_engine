use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let profile = env::var("PROFILE").unwrap();

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let target_dir = out_dir
        .ancestors()
        .nth(3)
        .expect("Failed to get target dir");

    let dll_src = PathBuf::from("lua53.dll");
    let dll_dst = target_dir.join("lua53.dll");

    if profile == "debug" {
        println!("cargo:rerun-if-changed=lua53.dll");

        match fs::copy(&dll_src, &dll_dst) {
            Ok(_) => println!("Copied lua53.dll to {:?}", dll_dst),
            Err(e) => {
                println!("cargo:warning=Failed to copy lua53.dll: {}", e);
            }
        }
    }
}
