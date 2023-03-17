use sub_projects::{delete_useless, UnityProjectFinder};

fn main() {
    for path in UnityProjectFinder::find("F:\\MiniGames") {
        delete_useless(&path).unwrap();
    }
}
