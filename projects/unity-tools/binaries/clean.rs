use unity_utils::helper::find_unity_projects;

pub fn main() {
    for i in find_unity_projects(".") {
        match i.delete_useless() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error: {:?}, {}", e, i.root_path().display());
            }
        }
    }
}
