#![feature(generators)]

mod deduplication;
mod project_clean;
mod errors;

use fast_walker::utils::to_unix_path;
pub use project_clean::*;

mod utils;

pub use crate::errors::{UnityError, UnityResult};
pub use crate::deduplication::{UnityProject, meta_file::UnityMetaFile};

