#![feature(generators)]

mod projects;

mod errors;

pub mod utils;

pub use crate::{
    errors::{UnityError, UnityResult},
    projects::{meta_file::UnityMetaFile, UnityProject},
};
