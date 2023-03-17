use unity_clean::{delete_useless, UnityProjectFinder};

fn main() {
    for path in UnityProjectFinder::find("F:\\") {
        match delete_useless(&path) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error: {:?}, {}", e, path.display());
            }
        }
    }
}
