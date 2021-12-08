mod commands;
use clap::Parser;

pub use self::commands::TailwindCommands;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct TailwindApp {
    #[clap(subcommand)]
    command: TailwindCommands,
}

fn main() {
    let cli = TailwindApp::parse();
    cli.command.run(&cli)
}
