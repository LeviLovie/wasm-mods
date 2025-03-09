use anyhow::{Context, Result};
use cargo_metadata::{Metadata, MetadataCommand};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashSet;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};

macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo::warning={}", format!($($tokens)*))
    }
}

fn main() -> Result<()> {
    // Always rerun
    println!("cargo:rerun-if-changed=build.rs");
    if env::var("WATCH_MODS").is_ok() && env::var("CARGO_CMD").unwrap_or_default() == "run" {
        println!("cargo:rerun-if-changed=mods");
        println!("cargo:rerun-if-changed=crates/common");
        println!("cargo:rerun-if-changed=wit");

        // Build mods first
        build_all_mods()?;

        // Only set up watchers during development
        if env::var("CARGO_TASK").unwrap_or_default() == "build" {
            setup_mod_watcher()?;
        }
    } else {
        // Always rebuild if any mod or common code changes
        println!("cargo:rerun-if-changed=mods");
        println!("cargo:rerun-if-changed=crates/common");
        println!("cargo:rerun-if-changed=wit");
        build_all_mods()?;
    }

    Ok(())
}

fn build_all_mods() -> Result<()> {
    let metadata = get_workspace_metadata()?;
    let mod_packages = find_mod_packages(&metadata)?;

    let mut mods_to_build = Vec::new();
    for package_id in &mod_packages {
        let package = metadata
            .packages
            .iter()
            .find(|p| p.id == *package_id)
            .context("Package not found in metadata")?;
        mods_to_build.push(package.name.clone());
    }
    p!("Building mods: {:?}", mods_to_build);

    for package_id in mod_packages {
        let package = metadata
            .packages
            .iter()
            .find(|p| p.id == package_id)
            .context("Package not found in metadata")?;

        let status = Command::new("cargo")
            .args([
                "build",
                "--target",
                "wasm32-wasip1",
                "--release",
                "--package",
                &package.name,
            ])
            .status()
            .context("Failed to execute cargo build")?;
        if !status.success() {
            anyhow::bail!("Failed to build mod: {}", package.name);
        }
        let build_dir_path = env::current_dir()
            .unwrap()
            .join("target")
            .join("wasm32-wasip1")
            .join("release")
            .join(package.name.replace("-", "_"));
        let build_file_path = format!("{}.wasm", build_dir_path.to_str().unwrap());
        let component_file_path = format!("{}_comp.wasm", build_dir_path.to_str().unwrap());
        let status = Command::new("wasm-tools")
            .args([
                "component",
                "new",
                &build_file_path,
                "--output",
                &component_file_path,
            ])
            .status()
            .context("Failed to execute wasm-tools strip")?;
        if !status.success() {
            anyhow::bail!("Failed to strip mod: {}", package.name);
        }

        // Copy the built WASM file to the target/wasm directory
        let target_dir = metadata.target_directory.join("wasm");
        std::fs::create_dir_all(&target_dir)?;

        let wasm_file = Path::new(&component_file_path);

        if wasm_file.exists() {
            let profile = env::var("PROFILE").unwrap();
            let dest = target_dir
                .parent()
                .unwrap()
                .join(profile)
                .join("wasm")
                .join(format!("{}.wasm", package.name));
            std::fs::create_dir_all(dest.parent().unwrap())?;
            std::fs::copy(&wasm_file, &dest)?;
        } else {
            p!("Could not find built WASM file at {:?}", wasm_file);
        }
    }

    Ok(())
}

fn setup_mod_watcher() -> Result<()> {
    // Set up a file watcher that will rebuild mods when they change
    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(
        move |res| {
            if let Ok(event) = res {
                tx.send(event).unwrap_or(());
            }
        },
        Config::default(),
    )?;

    // Watch the mods directory for changes
    watcher.watch(Path::new("mods"), RecursiveMode::Recursive)?;

    // Also watch the common crate as changes there affect mods
    watcher.watch(Path::new("crates/common"), RecursiveMode::Recursive)?;

    let metadata = get_workspace_metadata()?;

    // Keep track of the last time we built each mod to avoid rebuilding too frequently
    let mut last_builds: HashSet<PathBuf> = HashSet::new();
    let debounce_time = Duration::from_secs(2);
    let mut last_build_time = Instant::now() - debounce_time;

    // Event loop
    loop {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(Event { paths, .. }) => {
                // Skip if we just built and it's within the debounce period
                if last_build_time.elapsed() < debounce_time {
                    continue;
                }

                // Determine which mod was changed
                let mut mods_to_rebuild = HashSet::new();

                for path in &paths {
                    // Skip temporary files, target directories, etc.
                    if should_ignore_path(path) {
                        continue;
                    }

                    // Determine which mod this file belongs to
                    if let Some(mod_path) = get_mod_path(path) {
                        if !last_builds.contains(&mod_path) {
                            mods_to_rebuild.insert(mod_path.clone());
                            last_builds.insert(mod_path);
                        }
                    } else if path.starts_with("crates/common") {
                        // If the common crate changed, rebuild all mods
                        println!("Common crate changed, rebuilding all mods");
                        build_all_mods()?;
                        break;
                    }
                }

                // Rebuild the specific mods that changed
                for mod_path in mods_to_rebuild {
                    let mod_name = mod_path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown");

                    println!("Rebuilding mod: {}", mod_name);

                    let status = Command::new("cargo")
                        .current_dir(&mod_path)
                        .args(["build", "--target", "wasm32-unknown-unknown", "--release"])
                        .status();

                    if let Ok(status) = status {
                        if status.success() {
                            // Copy the built WASM file to the target directory
                            let target_dir = metadata.target_directory.join("wasm");
                            std::fs::create_dir_all(&target_dir)?;

                            let wasm_file = metadata
                                .target_directory
                                .join("wasm32-unknown-unknown/release")
                                .join(format!("{}.wasm", mod_name.replace('-', "_")));

                            if wasm_file.exists() {
                                let dest = target_dir.join(format!("{}.wasm", mod_name));
                                std::fs::copy(&wasm_file, &dest)?;
                                println!("Rebuilt mod: {}", mod_name);
                            }
                        } else {
                            println!("Failed to rebuild mod: {}", mod_name);
                        }
                    }
                }

                last_build_time = Instant::now();
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                // No events, check if we should clear the last_builds cache
                if last_build_time.elapsed() > Duration::from_secs(30) {
                    last_builds.clear();
                }
            }
            Err(e) => {
                println!("Watch error: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}

fn get_workspace_metadata() -> Result<Metadata> {
    MetadataCommand::new()
        .exec()
        .context("Failed to get cargo metadata")
}

fn find_mod_packages(metadata: &Metadata) -> Result<Vec<cargo_metadata::PackageId>> {
    let mut mod_packages = Vec::new();
    let mods_dir = env::current_dir()?.join("mods");
    let mods_dir_str = mods_dir.to_str().unwrap();

    for package in &metadata.packages {
        if package.manifest_path.starts_with(mods_dir_str) {
            // Don't include the virtual package
            if package.name != "mods" {
                mod_packages.push(package.id.clone());
            }
        }
        //// Check if package is in a subdirectory of "mods" and not the mods workspace itself
        //if package.manifest_path.as_std_path().starts_with("mods/")
        //    && !package
        //        .manifest_path
        //        .as_std_path()
        //        .ends_with("mods/Cargo.toml")
        //{
        //    // Don't include the virtual package
        //    if package.name != "mods-virtual" {
        //        mod_packages.push(package.id.clone());
        //    }
        //}
    }

    Ok(mod_packages)
}

fn should_ignore_path(path: &Path) -> bool {
    path.to_str()
        .map(|s| {
            s.contains("/target/")
                || s.contains("\\.git\\")
                || s.ends_with("~")
                || s.ends_with(".swp")
                || s.ends_with(".tmp")
        })
        .unwrap_or(false)
}

fn get_mod_path(path: &Path) -> Option<PathBuf> {
    let path_str = path.to_str()?;
    if !path_str.starts_with("mods/") {
        return None;
    }

    // Extract mod name from path (e.g., "mods/mod-one/...")
    let parts: Vec<&str> = path_str.splitn(3, '/').collect();
    if parts.len() < 2 {
        return None;
    }

    Some(PathBuf::from(format!("{}/{}", parts[0], parts[1])))
}
