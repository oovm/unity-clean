use clap::Parser;
use unity_tools::UnityTools;

pub fn main() {
    let app = UnityTools::parse();
    app.run();
}
