use std::{env, fs, path::PathBuf};

fn main() {
    let dist = PathBuf::from("dist");
    let _ = fs::create_dir_all(&dist);

    if let Ok(entries) = fs::read_dir(&dist) {
        for entry in entries.flatten() {
            let _ = fs::remove_file(entry.path());
        }
    }

    let profile = env::var("PROFILE").unwrap();
    let exe = if cfg!(windows) { "ferrum.exe" } else { "ferrum" };
    let pdb = "ferrum.pdb";

    let src = PathBuf::from("target").join(&profile).join(exe);
    let _ = fs::copy(&src, dist.join(exe));

    let src_pdb = PathBuf::from("target").join(&profile).join(pdb);
    if src_pdb.exists() {
        let _ = fs::copy(&src_pdb, dist.join(pdb));
    }

    println!("cargo:rustc-env=FERRUM_WRAPPER={}", PathBuf::from("run-wrapper.bat").display());
}
