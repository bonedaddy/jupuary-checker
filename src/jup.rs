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
    #[serde(rename = "user_public_key")]
    pub user_public_key: String,
    #[serde(rename = "swap_score")]
    pub swap_score: f64,
    #[serde(rename = "swap_tier")]
    pub swap_tier: i64,
    #[serde(rename = "swap_allocation_base")]
    pub swap_allocation_base: i64,
    #[serde(rename = "swap_consistency_bonus")]
    pub swap_consistency_bonus: i64,
    #[serde(rename = "expert_score")]
    pub expert_score: i64,
    #[serde(rename = "expert_tier")]
    pub expert_tier: i64,
    #[serde(rename = "expert_allocation")]
    pub expert_allocation: i64,
    #[serde(rename = "stakers_score")]
    pub stakers_score: f64,
    #[serde(rename = "stakers_allocation_base")]
    pub stakers_allocation_base: i64,
    #[serde(rename = "stakers_super_voter_bonus")]
    pub stakers_super_voter_bonus: i64,
    #[serde(rename = "stakers_super_staker_bonus")]
    pub stakers_super_staker_bonus: i64,
    #[serde(rename = "flagged_as_ATA")]
    pub flagged_as_ata: bool,
    #[serde(rename = "flagged_by_chainanalysis")]
    pub flagged_by_chainanalysis: bool,
    #[serde(rename = "included_in_airdrop")]
    pub included_in_airdrop: bool,
    #[serde(rename = "think_got_hacked")]
    pub think_got_hacked: bool,
    #[serde(rename = "swapped_under_3_weeks")]
    pub swapped_under_3_weeks: bool,
    #[serde(rename = "likely_sybil_farmer")]
    pub likely_sybil_farmer: bool,
    #[serde(rename = "swap_score_too_low")]
    pub swap_score_too_low: bool,
    #[serde(rename = "not_qualified_for_swap")]
    pub not_qualified_for_swap: bool,
    #[serde(rename = "too_many_sus_txs")]
    pub too_many_sus_txs: bool,
    #[serde(rename = "failure_rate_too_high")]
    pub failure_rate_too_high: bool,
    #[serde(rename = "used_other_jup_products")]
    pub used_other_jup_products: bool,
    #[serde(rename = "no_swaps_during_airdrop_period")]
    pub no_swaps_during_airdrop_period: bool,
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
    res.json().await.with_context(|| "failed to deserialize response")
}