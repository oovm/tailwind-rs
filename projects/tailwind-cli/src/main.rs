use std::path::PathBuf;

use clap::Parser;

pub use self::{commands::TailwindCommands, run::Mode};

mod commands;
mod run;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct TailwindApp {
    /// Sets a custom config file
    #[clap(parse(from_os_str), value_name = "DIR")]
    workspace: Option<PathBuf>,
    /// Sets a custom config file
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    config: Option<PathBuf>,
    #[clap(short, long, value_name = "GLOB")]
    pattern: Option<String>,
    #[clap(short, long)]
    minify: Option<bool>,
    #[clap(long)]
    obfuscate: Option<bool>,
    #[clap(long)]
    dry_run: bool,
    #[clap(short, parse(from_occurrences))]
    details: usize,
    #[clap(long, arg_enum)]
    mode: Option<Mode>,
    #[clap(subcommand)]
    command: Option<TailwindCommands>,
}

fn main() {
    let cli = TailwindApp::parse();
    let (cfg, mut builder) = cli.build_config();
    cli.run(&cfg, &mut builder).ok();
}
