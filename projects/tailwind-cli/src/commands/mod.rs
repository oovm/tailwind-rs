use std::path::PathBuf;

use clap::Subcommand;

use tailwind_rs::{CLIConfig, Result};

#[derive(Subcommand)]
pub enum TailwindCommands {
    Init {
        #[clap(value_parser, value_name = "DIR")]
        workspace: Option<PathBuf>,
    },
}

impl TailwindCommands {
    pub fn run(&self, config: &CLIConfig) -> Result<()> {
        let _ = config;
        println!("?");
        match self {
            Self::Init { workspace } => {
                println!("'myapp add' was used, name is: {:?}", workspace)
            },
        }
        Ok(())
    }
}
