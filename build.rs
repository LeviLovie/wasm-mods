use std::{env, fs, path::Path, process::Command};

macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo::warning={}", format!($($tokens)*))
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=mods/");
    println!("cargo:rerun-if-changed=wit/");

    let mods = find_mod_packages();
    for mod_ in mods {
        build_mod(mod_.0, mod_.1);
    }
}

fn build_mod(name: String, path: String) {
    p!("Building \"{}\"", &name);
    let build_script_path = Path::new(&path).join("build.sh");
    let output = Command::new("sh")
        .arg(build_script_path.to_str().unwrap().to_string())
        .current_dir(&path)
        .output()
        .expect("Failed to run build script");
    if !output.status.success() {
        p!(
            "Failed to build \"{}\": {}. Run \"cd {} && sh build.sh\" to debug",
            &name,
            output.status,
            &path
        );
        return;
    }

    let profile = env::var("PROFILE").unwrap();
    let dest = Path::new("target").join(profile).join("wasm");
    let source = Path::new(&path).join("mod.wasm");
    if !source.exists() {
        p!(
            "Skipping `{}` because `{}` does not exist",
            &name,
            source.to_str().unwrap()
        );
        return;
    }

    p!(
        "Copying {} to {:?}",
        source.to_str().unwrap(),
        dest.to_str().unwrap()
    );
    fs::create_dir_all(&dest).expect("Failed to create target directory");
    fs::copy(&source, dest.join(format!("{}.wasm", &name))).expect("Failed to copy wasm file");

    p!("Built {}", &name);
}

fn find_mod_packages() -> Vec<(String, String)> {
    let mods_dir = env::current_dir()
        .expect("Failed to get current directory")
        .join("mods");
    let dirs = get_folders_in_directory(mods_dir.to_str().unwrap());
    let mut packages = Vec::new();

    for dir in dirs {
        if !dir.ends_with("_mod") {
            continue;
        }
        packages.push((
            dir.clone(),
            mods_dir.join(dir.clone()).to_str().unwrap().to_string(),
        ));
    }

    packages
}

fn get_folders_in_directory(dir: &str) -> Vec<String> {
    let mut folders = Vec::new();

    for entry in fs::read_dir(dir).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        if path.is_dir() {
            if let Some(folder_name) = path.file_name().and_then(|name| name.to_str()) {
                folders.push(folder_name.to_string());
            }
        }
    }

    folders
}
