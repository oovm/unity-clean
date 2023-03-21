use crate::UnityProject;
use fast_walker::{WalkItem, WalkPlan};
use gen_iter::GenIter;
use std::path::Path;

pub fn find_unity_projects<P: AsRef<Path>>(root: P) -> impl Iterator<Item = UnityProject> {
    let plan = WalkPlan::new(root).reject_if(|path, _| path.starts_with(".")).into_iter();
    GenIter(move || {
        for item in plan {
            match item {
                WalkItem::File { .. } => {}
                WalkItem::Directory { path } => {
                    if let Ok(project) = UnityProject::new(path) {
                        yield project;
                    }
                }
                WalkItem::Error { .. } => {}
            }
        }
    })
}
