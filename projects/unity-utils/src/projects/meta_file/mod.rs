use std::fmt::{Debug, Formatter};
use super::*;


pub struct UnityMetaFile {
    pub meta: PathBuf,
    pub file: PathBuf,
}


impl Debug for UnityMetaFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnityMetaFile")
            .field("meta", &to_unix_path(&self.meta))
            .field("file", &to_unix_path(&self.file))
            .finish()
    }
}

impl UnityProject {
    pub fn find_meta(&self) -> impl Iterator<Item=UnityMetaFile> {
        let plan = WalkPlan::new(self.root.join("Assets"))
            .reject_if(|path, _| path.starts_with("."))
            .ignore_if(|path| !path.to_string_lossy().ends_with(".meta"));
        GenIter(move || {
            for item in plan.into_iter() {
                match item {
                    WalkItem::File { path } => {
                        let meta = path.clone();
                        let file = path.with_extension("");
                        if meta.exists() && file.exists() {
                            yield UnityMetaFile { meta, file };
                        }
                    }
                    WalkItem::Directory { .. } => {}
                    WalkItem::Error { .. } => {}
                }
            }
        })
    }
}