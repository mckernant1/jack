use std::{collections::BTreeMap, io::stdout};

use crate::{
    args::raw::{Format, Raw},
    commands::{cwl_client, wait_for_query},
};
use color_eyre::eyre::eyre;
use color_eyre::Result;
use csv::WriterBuilder;
use log::debug;

pub async fn search_raw(raw: Raw) -> Result<()> {
    debug!("Starting raw command with args, {:?}", raw);
    let result = cwl_client()
        .await
        .start_query()
        .log_group_name(raw.log_group)
        .start_time(raw.start_time.timestamp())
        .end_time(raw.end_time.timestamp())
        .query_string(raw.query_string)
        .send()
        .await?;

    let query_id = result.query_id().ok_or(eyre!(""))?;
    let results = wait_for_query(query_id).await?;

    match raw.format {
        Format::Json => display_json_output(results),
        Format::Csv => display_csv_output(results),
    }
}

fn display_json_output(results: Vec<BTreeMap<String, String>>) -> Result<()> {
    for res in results {
        println!("{}", serde_json::to_string(&res)?);
    }
    Ok(())
}

fn display_csv_output(results: Vec<BTreeMap<String, String>>) -> Result<()> {
    let mut wtr = WriterBuilder::new().from_writer(stdout());

    if let Some(map) = results.first() {
        wtr.write_record(map.keys())?;
        wtr.flush()?;
    }

    for res in results {
        wtr.write_record(res.values())?;
        wtr.flush()?;
    }

    Ok(())
}
