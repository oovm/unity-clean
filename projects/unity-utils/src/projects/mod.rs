use std::path::{Path, PathBuf};
use fast_walker::{WalkItem, WalkPlan};
use gen_iter::GenIter;
use crate::{UnityError, UnityResult};
use fast_walker::utils::to_unix_path;
pub mod meta_file;
use std::fmt::{Debug, Formatter};

pub struct UnityProject {
    root: PathBuf,
}

mod clean;

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
    pub fn assets_path(&self) -> PathBuf {
        self.root.join("Assets")
    }
}

