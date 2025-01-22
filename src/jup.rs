use std::str::FromStr;

use anyhow::Context;
use reqwest::header::HeaderValue;
use serde::{Serialize, Deserialize};
use serde_json::Value;

const URL: &str = "https://jupuary.jup.ag/api/allocation";

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub status: String,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    #[serde(rename = "total_allocated")]
    pub total_allocated: i64,
}


pub fn new_url(wallet: &str) -> String {
    format!("{URL}?wallet={wallet}")
}

pub async fn send_request(
    client: &reqwest::Client,
    wallet: &str
) -> anyhow::Result<Root> {
    let mut req = client.get(new_url(wallet)).build()?;
    req.headers_mut().append(
        "referer",
        HeaderValue::from_str(&format!("https://jupuary.jup.ag/allocation/{wallet}"))?,
    );
    let res = client.execute(req).await?;
    res.json().await.with_context(|| "failed to deserialize")
}