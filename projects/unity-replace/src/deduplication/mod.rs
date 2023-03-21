use std::path::{Path, PathBuf};
use fast_walker::{WalkItem, WalkPlan};
use gen_iter::GenIter;
use crate::{UnityError, UnityResult};

pub mod meta_file;

pub struct UnityProject {
    root: PathBuf,
}

impl UnityProject {
    pub fn new<P: AsRef<Path>>(root: P) -> UnityResult<Self> {
        let out = Self {
            root: root.as_ref().to_path_buf(),
        };
        let assets = out.assets_path();
        if !assets.exists() || !assets.is_dir() {
            Err(UnityError::custom_error("Assets folder not found"))?
        }
        Ok(out)
    }
    pub fn assets_path(&self) -> &Path {
        self.root.join("Assets").as_path()
    }

}

