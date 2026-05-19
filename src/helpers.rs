use std::fs::{DirEntry, read_dir, rename};
use std::io;
use std::path::{Path, PathBuf};

fn to_dirs(strs: &[String]) -> Result<Vec<PathBuf>, String> {
    strs.iter()
        .map(|s| {
            if Path::new(s).is_dir() {
                Ok(PathBuf::from(s))
            } else {
                Err(format!("{} is not a directory!", s))
            }
        })
        .collect()
}

fn replace_spaces(path: &Path) -> Result<PathBuf, String> {
    let name = path
        .file_name()
        .ok_or_else(|| format!("{} has no file name!", path.display()))?
        .to_str()
        .ok_or_else(|| format!("{} is not valid filename!", path.display()))?;
    Ok(path.with_file_name(name.replace(' ', "_")))
}

fn process_file(file: io::Result<DirEntry>) -> Result<(), String> {
    let old_path = file
        .map_err(|e| format!("Failed to read entry: {}", e))?
        .path();
    let new_path = replace_spaces(&old_path)?;

    if old_path == new_path {
        return Ok(());
    }

    rename(&old_path, &new_path)
        .map_err(|e| format!("Failed to rename {}: {}", old_path.display(), e))?;

    println!("Renamed {} to {}", old_path.display(), new_path.display());
    Ok(())
}

fn kill_spaces(dir: &Path) {
    let files = match read_dir(dir) {
        Ok(f) => f,
        Err(e) => return eprintln!("Failed to read directory {}: {}", dir.display(), e),
    };

    for file in files {
        if let Err(e) = process_file(file) {
            eprintln!("{}", e);
        }
    }
}

pub fn run_kill_spaces(dirs: &[String]) {
    let dirs = to_dirs(dirs).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });

    for dir in dirs {
        kill_spaces(&dir);
    }
}
