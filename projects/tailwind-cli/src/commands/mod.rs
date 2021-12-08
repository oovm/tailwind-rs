use clap::Subcommand;

use crate::TailwindApp;

#[derive(Subcommand)]
pub enum TailwindCommands {
    /// Adds files to myapp
    Build { name: Option<String> },
}

impl TailwindCommands {
    pub fn run(&self, config: &TailwindApp) {
        let _ = config;
        match self {
            Self::Build { name } => {
                println!("'myapp add' was used, name is: {:?}", name)
            },
        }
    }
}
