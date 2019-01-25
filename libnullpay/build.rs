use std::env;
use std::path::Path;

fn main() {
    let target = env::var("TARGET").unwrap();
    println!("target={}", target);
    if target.find("-windows-").is_some() {
        println!("cargo:rustc-link-lib=indy.dll");

        let profile = env::var("PROFILE").unwrap();
        println!("profile={}", profile);

        let indy_dir = env::var("INDY_DIR").unwrap_or(format!("..\\libindy\\target\\{}", profile));
        println!("indy_dir={}", indy_dir);
        let indy_dir = Path::new(indy_dir.as_str());

        println!("cargo:rustc-flags=-L {}", indy_dir.as_os_str().to_str().unwrap());
    } else if env::var("INDY_DIR").ok().is_some() {
        let indy_dir = env::var("INDY_DIR").unwrap();

        if target.find("aarch64-").is_some() {
            println!("cargo:rustc-link-search=native={}/libindy_arm64/lib/libindy.so", indy_dir)
        } else if target.find("i686-").is_some() {
            println!("cargo:rustc-link-search=native={}/libindy_x86/lib/libindy.so", indy_dir)
        } else if target.find("x86_x64-").is_some() {
            println!("cargo:rustc-link-search=native={}/libindy_x86_x64/libindy.so", indy_dir)
        }
    } else {
        println!("cargo:rustc-link-lib=indy")
    }
}
