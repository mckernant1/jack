pub mod raw;
pub mod search;

use chrono::{DateTime, Local};
use chrono_english::{parse_date_string, DateError, Dialect};
use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

use self::{raw::Raw, search::Search};

/// For fetching and parsing cloudwatch logs
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Logging level
    #[command(flatten)]
    pub verbose: Verbosity,
    /// how we want to search the logs
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    /// Search through the logs with a string or regex
    Search(Search),
    /// Put in a raw query https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CWL_QuerySyntax.html
    Raw(Raw),
}

pub fn parse_date_time(arg: &str) -> Result<DateTime<Local>, DateError> {
    parse_date_string(arg, Local::now(), Dialect::Us)
}
