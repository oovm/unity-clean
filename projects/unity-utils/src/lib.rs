#![feature(generators)]

mod projects;
mod project_clean;
mod errors;


pub use project_clean::*;

mod utils;

pub use crate::errors::{UnityError, UnityResult};
pub use crate::projects::{UnityProject, meta_file::UnityMetaFile};

