use application::cli::MigrateCommand;
use migration::Migrator;
use tracing::{info, Level};

use crate::settings::Settings;

pub fn start(settings: &Settings, log_level: Level) -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_max_level(log_level).init();

    info!(
        "Starting SIGE api on http://{}:{}!",
        settings.address.host, settings.address.port
    );

    web::start(
        &settings.address.host,
        settings.address.port,
        &settings.database.url,
    )?;

    Ok(())
}

pub fn migrate(db_url: &str, command: MigrateCommand) -> anyhow::Result<()> {
    Migrator::from_cli_command(db_url, command)?;

    Ok(())
}
