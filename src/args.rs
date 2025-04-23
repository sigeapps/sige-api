use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]

pub enum Commands {
    #[command(about = "Start the api on production mode")]
    Start,
    #[command(about = "Start the api on development mode")]
    Dev,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}
