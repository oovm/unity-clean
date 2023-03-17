use std::fs::DirEntry;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

// use walkdir::{DirEntry, WalkDir};

pub struct UnityCleaner {}


pub fn find_unity_project(root: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut projects = vec![];
    let mut check_list = vec![root.to_path_buf()];
    while let Some(dir) = check_list.pop() {
        for path in read_dir(dir)? {
            let path = path?.path();
            if fast_skip(&path).unwrap_or(false) {
                continue;
            }
            if is_unity_path(&path) {
                println!("Found: {:?}", path.display());
                projects.push(path);
                continue;
            }
            if path.is_dir() {
                check_list.push(path)
            }
        }
    }
    Ok(projects)
}

pub fn fast_skip(dir: &Path) -> Option<bool> {
    if dir.file_name()?.to_str()?.starts_with('.') {
        return Some(true);
    }
    Some(false)
}

pub fn is_unity_path(dir: &Path) -> bool {
    let assets = dir.join("Assets");
    if assets.exists() && assets.is_dir() {
        return false;
    }
    let packages = dir.join("Packages");
    if packages.exists() && packages.is_dir() {
        return false;
    }
    let project_settings = dir.join("ProjectSettings");
    if !project_settings.exists() || !project_settings.is_dir() {
        return false;
    }
    return true;
}

pub fn delete_useless(path: &Path) -> anyhow::Result<()> {
    'outer: for file in read_dir(path)? {
        let (path, name) = path_info(file)?;
        let delete_dir = &["Library", "Logs", "obj", "Temp"];
        for dir in delete_dir {
            if name.eq_ignore_ascii_case(dir) {
                println!("Delete: {:?}", path.display());
                std::fs::remove_dir_all(&path)?;
                continue 'outer;
            }
        }
        if path.ends_with(".csproj") {
            println!("Delete: {:?}", path.display());
            std::fs::remove_file(&path)?;
            continue;
        }
        if path.ends_with(".sln") {
            println!("Delete: {:?}", path.display());
            std::fs::remove_file(&path)?;
            continue;
        }
    }
    Ok(())
}

fn path_info(entry: std::io::Result<DirEntry>) -> anyhow::Result<(PathBuf, String)> {
    let entry = entry?;
    let name = match entry.file_name().to_str() {
        Some(s) => { s.to_string() }
        None => {
            return Err(anyhow::anyhow!("file name is not utf-8"));
        }
    };
    Ok((entry.path().canonicalize()?, name))
}
