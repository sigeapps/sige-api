use application::cli::MigrateCommand;
use migration::Migrator;
use tracing::{info, Level};

use crate::settings::Settings;

use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub fn start(settings: &Settings, log_level: Level) -> anyhow::Result<()> {
    let filter = EnvFilter::builder()
        .with_default_directive(log_level.into())
        .from_env_lossy();

    tracing_subscriber::registry()
        .with(filter)
        .with(
            fmt::layer()
                .with_target(false)
                .with_thread_ids(false)
                .with_thread_names(false)
                .with_file(false)
                .with_line_number(false)
                .compact(),
        )
        .init();

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
