use super::*;
use std::fs::{read_dir, DirEntry};

const DELETE_DIR: &[&str] = &["Library", "Logs", "obj", "Temp"];
const DELETE_EXT: &[&str] = &[".sln", ".csproj"];

impl UnityProject {
    pub fn delete_useless(&self) -> UnityResult<()> {
        'outer: for file in read_dir(&self.root)? {
            let (path, name) = path_info(file)?;
            for dir in DELETE_DIR {
                if name.eq_ignore_ascii_case(dir) {
                    println!("Delete: {:?}", path.display());
                    trash::delete(&path)?;
                    continue 'outer;
                }
            }
            for file in DELETE_EXT {
                if name.ends_with(file) {
                    println!("Delete: {:?}", path.display());
                    trash::delete(&path)?;
                    continue 'outer;
                }
            }
        }
        Ok(())
    }
}

fn path_info(entry: std::io::Result<DirEntry>) -> UnityResult<(PathBuf, String)> {
    let entry = entry?;
    let name = match entry.file_name().to_str() {
        Some(s) => s.to_string(),
        None => Err(UnityError::custom_error("File name is not utf-8"))?,
    };
    Ok((entry.path().canonicalize()?, name))
}
