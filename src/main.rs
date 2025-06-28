use application::cli::{CliArgs, Commands};
use clap::Parser;
use dotenv::dotenv;
use sige_api::{commands, settings::Settings};
use tracing::Level;

fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let args = CliArgs::parse();

    let settings = Settings::new()?;

    match &args.command {
        Some(Commands::Start) => Ok(commands::start(&settings, Level::INFO)?),
        Some(Commands::Dev) => Ok(commands::start(&settings, Level::DEBUG)?),
        Some(Commands::Migrate(args)) => Ok(commands::migrate(
            &settings.database.url,
            args.command.clone(),
        )?),
        None => Ok(()),
    }
}
