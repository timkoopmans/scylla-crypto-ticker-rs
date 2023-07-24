use scylla::Session;
use scylla::prepared_statement::PreparedStatement;
use scylla::transport::errors::QueryError;
use chrono::Utc;

pub async fn write_prices(session: &Session, side: &str) -> anyhow::Result<PreparedStatement, QueryError> {
    session
        .prepare(format!(
            "INSERT INTO orders.{} \
        (timestamp, exchange, base, quote, price) \
        VALUES (?, ?, ?, ?, ?)",
            side
        ))
        .await
}

pub async fn read_prices(
    session: &Session,
    side: &str,
) -> anyhow::Result<Vec<f64>, Box<dyn std::error::Error>> {
    let now = Utc::now();
    let seconds_ago = now - chrono::Duration::seconds(300);

    let rows = session
        .query(
            format!(
                "SELECT price FROM orders.{} WHERE timestamp >= ? ALLOW FILTERING",
                side
            ),
            (seconds_ago,),
        )
        .await?
        .rows
        .unwrap_or_default();

    if rows.is_empty() {
        println!("No data in last 300 seconds");
        return Ok(Vec::new());
    }

    Ok(rows
        .iter()
        .filter_map(|row| {
            let price: f64 = row.columns[0].as_ref()?.as_double()?;
            Some(price)
        })
        .collect())
}
