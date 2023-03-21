use crate::UnityTools;
use clap::Parser;
use unity_utils::helper::find_unity_projects;

#[derive(Parser, Debug)]
pub enum UnityCommands {
    Clean(Box<UnityClean>),
}

#[derive(Parser, Debug)]
pub struct UnityClean {
    /// Clean all files
    #[clap(short, long)]
    pub all: bool,
}

impl UnityTools {
    pub fn run(&self) {
        match &self.command {
            UnityCommands::Clean(cmd) => cmd.run(),
        }
    }
}

impl UnityClean {
    pub fn run(&self) {
        match self.all {
            true => self.clean_all(),
            false => self.clean_this(),
        }
    }
    fn clean_all(&self) {
        for i in find_unity_projects(".") {
            match i.delete_useless() {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error: {:?}, {}", e, i.root_path().display());
                }
            }
        }
    }
    fn clean_this(&self) {}
}
