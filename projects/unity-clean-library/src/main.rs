use std::path::Path;

use sub_projects::{delete_useless, find_unity_project};


fn main() {
    let out = find_unity_project(Path::new("F:\\MiniGames")).unwrap();
    for path in out {
        delete_useless(&path).unwrap();
    }
}