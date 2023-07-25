use scylla::prepared_statement::PreparedStatement;
use scylla::transport::errors::QueryError;
use scylla::Session;
use std::sync::Arc;

pub async fn write_trades(session: &Arc<Session>) -> anyhow::Result<PreparedStatement, QueryError> {
    session
        .prepare(
            "INSERT INTO orders.trades \
            (exchange, base, quote, timestamp, id, price, qty) \
            VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .await
}

pub async fn write_candles(
    session: &Arc<Session>,
) -> anyhow::Result<PreparedStatement, QueryError> {
    session
        .prepare("INSERT INTO orders.candles \
            (exchange, base, quote, time_bucket, open_price, high_price, low_price, close_price, volume) \
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .await
}
