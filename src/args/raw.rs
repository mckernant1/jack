use super::parse_date_time;
use chrono::{DateTime, Local};
use clap::{Args, ValueEnum};

#[derive(Args, Debug, Clone)]
pub struct Raw {
    /// Log group
    #[arg(short = 'L', long)]
    pub log_group: String,

    /// The CWL Query String https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CWL_QuerySyntax.html
    #[arg(short = 'Q', long)]
    pub query_string: String,

    /// Start time for query
    #[arg(short, long, value_parser = parse_date_time)]
    pub start_time: DateTime<Local>,

    /// End time for query
    #[arg(short, long, value_parser = parse_date_time)]
    pub end_time: DateTime<Local>,

    #[arg(short, long, value_enum, default_value_t = Format::Csv)]
    pub format: Format,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Format {
    Json,
    Csv,
}
