#![deny(clippy::all)]

use args::Cli;
use clap::Parser;
use color_eyre::Result;
use log::info;

pub mod args;
pub mod commands;

use args::Command::{Raw, Search};
use commands::search::search;

use crate::commands::raw::search_raw;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let cli: Cli = Cli::parse();
    env_logger::builder()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    info!("Hello!");

    match cli.command {
        Search(s) => search(s).await,
        Raw(r) => search_raw(r).await,
    }?;

    Ok(())
}
