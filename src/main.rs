use clap::Parser;
use sige_api::args::{Args, Commands};
use sige_api::commands::migrate;
use sige_api::commands::start;
use sige_api::settings::Settings;
use tracing::Level;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let settings = Settings::new()?;

    match &args.command {
        Some(Commands::Start) => Ok(start::start(&settings, Level::INFO)?),
        Some(Commands::Dev) => Ok(start::start(&settings, Level::DEBUG)?),
        Some(Commands::Migrate) => Ok(migrate::migrate(&settings)?),
        None => Ok(()),
    }
}
