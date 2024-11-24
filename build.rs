use std::{env, fs, path::PathBuf};

fn main() -> std::io::Result<()> {
    let config = std::fs::read_to_string("Cargo.toml")?;

    // Getting the version from Cargo.toml
    let version_str: String = config
        .split("\n")
        .find(|x| x.contains("version = "))
        .unwrap()
        .split("=")
        .collect::<Vec<&str>>()[1]
        .replace("\"", "")
        .trim()
        .to_string();

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = PathBuf::from(out_dir).join("version_file");

    fs::write(&dest_path, format!("{}", version_str)).unwrap();

    println!("cargo:rerun-if-changed=Cargo.toml");

    Ok(())
}
