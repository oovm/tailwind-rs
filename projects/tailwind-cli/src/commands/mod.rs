use std::path::PathBuf;

use clap::Subcommand;

use tailwind_rs::{GlobalConfig, Result};

#[derive(Subcommand)]
pub enum TailwindCommands {
    Init {
        #[clap(parse(from_os_str), value_name = "DIR")]
        workspace: Option<PathBuf>,
    },
}

impl TailwindCommands {
    pub fn run(&self, config: &GlobalConfig) -> Result<()> {
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
