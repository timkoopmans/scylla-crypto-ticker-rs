extern crate parse_duration;

use anyhow::Result;
use chrono::Utc;
use parse_duration::parse;

pub fn time_bucket_from(duration: String) -> Result<i64> {
    let input_duration = parse(&duration).map_err(|_| anyhow::anyhow!("Invalid resolution format."))?;

    let duration_secs = input_duration.as_secs();
    let time_bucket_from = (Utc::now().timestamp() - duration_secs as i64) / 2;

    Ok(time_bucket_from)
}