use super::{cwl_client, wait_for_query};
use crate::args::search::Search;

use color_eyre::{eyre::eyre, Result};
use log::debug;

pub async fn search(search: Search) -> Result<()> {
    debug!("Starting search command with args, {:?}", search);
    let result = cwl_client()
        .await
        .start_query()
        .log_group_name(search.log_group)
        .start_time(search.start_time.timestamp())
        .end_time(search.end_time.timestamp())
        .query_string(format!(
            r#"
            fields @message
            | filter @message like /{}/
        "#,
            search.key
        ))
        .limit(search.limit)
        .send()
        .await?;
    let query_id = result
        .query_id()
        .ok_or(eyre!("Didn't get a query id back from cloudwatch"))?;

    let results = wait_for_query(query_id).await?;

    for log in results.iter().filter_map(|it| it.get("@message")) {
        println!("{}", log);
    }

    Ok(())
}
