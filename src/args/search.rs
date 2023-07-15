use super::parse_date_time;
use chrono::{DateTime, Local};
use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct Search {
    /// Log group
    #[arg(short = 'L', long)]
    pub log_group: String,

    /// The regex to search for
    #[arg(short, long)]
    pub key: String,

    /// Start time for query
    #[arg(short, long, value_parser = parse_date_time)]
    pub start_time: DateTime<Local>,

    /// End time for query
    #[arg(short, long, value_parser = parse_date_time)]
    pub end_time: DateTime<Local>,

    /// Max number of log events
    #[arg(short, long, default_value_t = 100)]
    pub limit: usize,
}
