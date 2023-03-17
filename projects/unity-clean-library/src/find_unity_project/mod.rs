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
            if path.join("Assets").exists() {
                if is_unity_path(&path) {
                    projects.push(path);
                    continue;
                }
            }
            if path_buf.is_dir() {
                check_list.push(path_buf)
            }
        }
    }
    Ok(projects)
}

pub fn is_unity_path(path: &Path) -> Option<PathBuf> {
    Ok(path.join("Assets").exists())
}


#[test]
fn test() {
    find_unity_project(Path::new("F:\\MiniGames")).unwrap();
}