use std::path::Path;

use jwalk::{rayon::iter::ParallelBridge, Parallelism};

fn find_meta<P: AsRef<Path>>(root: P, threads: usize) {
    jwalk::WalkDir::new(root)
        .parallelism(Parallelism::RayonNewPool(threads))
        .into_iter()
        .par_bridge()
        .filter_map(|dir_entry_result| {
            let dir_entry = dir_entry_result.ok()?;
            if dir_entry.file_type().is_file() {
                let path = dir_entry.path();
                let text = std::fs::read_to_string(path).ok()?;
                if text.contains("hello world") {
                    return Some(true);
                }
            }
            None
        })
        .count();
}
