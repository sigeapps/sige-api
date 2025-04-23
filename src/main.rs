use clap::Parser;
use sige_api::args::{Args, Commands};
use sige_api::commands::start;
use sige_api::settings::Settings;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let settings = Settings::new()?;

    match &args.command {
        Some(Commands::Start) => Ok(start::start(&settings)),
        Some(Commands::Dev) => Ok(start::start(&settings)),
        None => Ok(()),
    }
}
