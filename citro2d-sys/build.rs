use std::env;

fn main() {
    let dkp_path = env::var("DEVKITPRO").unwrap();
    let debug_symbols = "0";
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=DEVKITPRO");
    println!("cargo:rustc-link-search=native={dkp_path}/libctru/lib");
    println!(
        "cargo:rustc-link-lib=static={}",
        match debug_symbols {
            // Based on valid values described in
            // https://doc.rust-lang.org/cargo/reference/profiles.html#debug
            "0" | "false" => "citro2d",
            _ => "citro2dd",
        }
    );
}