use std::{
    env, fs,
    io::Read,
    path::PathBuf,
    process::{Command, Stdio},
};

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

    // Input buffers
    let mut branch_buf = String::new();
    let mut head_buf = String::new();

    let _ = Command::new("git")
        .arg("branch")
        .arg("--show-current")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Couldn't get the current branch")
        .stdout
        .unwrap()
        .read_to_string(&mut branch_buf);
    let _ = Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Couldn't get the current Git HEAD")
        .stdout
        .unwrap()
        .read_to_string(&mut head_buf);

    branch_buf.remove(branch_buf.len() - 1);
    head_buf.remove(head_buf.len() - 1);

    println!("cargo:rerun-if-changed=Cargo.toml");

    fs::write(
        &dest_path,
        format!("{} on {} with {}", version_str, branch_buf, head_buf),
    )
    .unwrap();

    Ok(())
}
