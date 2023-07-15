use std::collections::BTreeMap;
use std::time::Duration;

use aws_sdk_cloudwatchlogs::types::QueryStatus;
use aws_sdk_cloudwatchlogs::Client;
use color_eyre::{eyre::eyre, Result};
use log::info;
use tokio::sync::OnceCell;

pub mod raw;
pub mod search;

const WAIT_SECS: u64 = 15;

static CWL: OnceCell<Client> = OnceCell::const_new();

pub async fn cwl_client() -> &'static Client {
    CWL.get_or_init(|| async {
        let config = aws_config::load_from_env().await;
        Client::new(&config)
    })
    .await
}

pub async fn wait_for_query(query_id: &str) -> Result<Vec<BTreeMap<String, String>>> {
    loop {
        let result = cwl_client()
            .await
            .get_query_results()
            .query_id(query_id)
            .send()
            .await?;

        let status = result.status().expect("Got none from query results");
        info!("QueryId: {}, Status: {}", query_id, status.as_str());
        match status {
            QueryStatus::Complete => {
                break Ok(result
                    .results()
                    .expect("Got None for results")
                    .iter()
                    .map(|it| {
                        it.iter()
                            .filter(|it| it.field().is_some() && it.value().is_some())
                            .map(|it| {
                                (
                                    it.field().unwrap().to_string(),
                                    it.value().unwrap().to_string(),
                                )
                            })
                            .collect()
                    })
                    .collect())
            }
            QueryStatus::Running | QueryStatus::Scheduled => {
                tokio::time::sleep(Duration::from_secs(WAIT_SECS)).await;
            }
            QueryStatus::Failed
            | QueryStatus::Cancelled
            | QueryStatus::Timeout
            | QueryStatus::Unknown(_)
            | QueryStatus::UnknownValue
            | _ => {
                break Err(eyre!(
                    "Hit Error Status waiting for Cloudwatch Log Insights to complete: {}",
                    status.as_str()
                ))
            }
        }
    }
}
