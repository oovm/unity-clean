use std::{
    fs::{read_dir, DirEntry},
    path::{Path, PathBuf},
};

use gen_iter::GenIter;

// use walkdir::{DirEntry, WalkDir};

pub struct UnityProjectFinder {}

impl UnityProjectFinder {
    pub fn find<P: AsRef<Path>>(root: P) -> impl Iterator<Item = PathBuf> {
        let mut check_list = vec![root.as_ref().to_path_buf()];
        GenIter(move || {
            'outer: while let Some(dir) = check_list.pop() {
                let dir = match read_dir(dir) {
                    Ok(o) => o,
                    Err(e) => {
                        tracing::error!("{e}");
                        continue 'outer;
                    }
                };
                'inner: for path in dir {
                    let (path, name) = match path_info(path) {
                        Ok(o) => o,
                        Err(_) => {
                            tracing::error!("{e}");
                            continue 'outer;
                        }
                    };
                    if fast_skip(&path).unwrap_or(false) {
                        continue 'inner;
                    }
                    if is_unity_path(&path) {
                        println!("Found: {:?}", path.display());
                        yield path;
                        continue 'inner;
                    }
                    if path.is_dir() {
                        check_list.push(path)
                    }
                }
            }
        })
    }
}

pub fn fast_skip(dir: &Path) -> Option<bool> {
    if dir.file_name()?.to_str()?.starts_with('.') {
        return Some(true);
    }
    Some(false)
}

pub fn is_unity_path(dir: &Path) -> bool {
    let assets = dir.join("Assets");
    if !assets.exists() || !assets.is_dir() {
        return false;
    }
    let packages = dir.join("Packages");
    if !packages.exists() || !packages.is_dir() {
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
                trash::delete_all(&path)?;
                continue 'outer;
            }
        }
        if name.ends_with(".csproj") {
            println!("Delete: {:?}", path.display());
            trash::delete(&path)?;
            continue;
        }
        if name.ends_with(".sln") {
            println!("Delete: {:?}", path.display());
            trash::delete(&path)?;
            continue;
        }
    }
    Ok(())
}

fn path_info(entry: std::io::Result<DirEntry>) -> anyhow::Result<(PathBuf, String)> {
    let entry = entry?;
    let name = match entry.file_name().to_str() {
        Some(s) => s.to_string(),
        None => {
            return Err(anyhow::anyhow!("file name is not utf-8"));
        }
    };
    Ok((entry.path().canonicalize()?, name))
}
