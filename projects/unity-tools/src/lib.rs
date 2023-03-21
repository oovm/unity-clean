pub use crate::commands::UnityCommands;
use clap::Parser;

mod commands;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = include_str!("../readme.md"))]
pub struct UnityTools {
    #[clap(subcommand)]
    pub command: UnityCommands,
}
