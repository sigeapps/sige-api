use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    #[command(about = "Start the api on production mode")]
    Start,
    #[command(about = "Start the api on development mode")]
    Dev,
    Migrate(MigrateArgs),
}

#[derive(Parser, Debug, Clone)]
#[command(about = "Manage database migrations", long_about = None)]
pub struct MigrateArgs {
    #[command(subcommand)]
    pub command: MigrateCommand,
}

#[derive(Subcommand, Debug, Clone)]
pub enum MigrateCommand {
    Up,
    Refresh,
    Down,
}
