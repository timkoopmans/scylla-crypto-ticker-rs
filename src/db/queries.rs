use scylla::Session;
use scylla::prepared_statement::PreparedStatement;
use scylla::transport::errors::QueryError;

pub async fn write_trades(session: &Session) -> anyhow::Result<PreparedStatement, QueryError> {
    session
        .prepare("INSERT INTO orders.trades \
            (exchange, base, quote, timestamp, id, price, qty) \
            VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .await
}

pub async fn write_candles(session: &Session) -> anyhow::Result<PreparedStatement, QueryError> {
    session
        .prepare("INSERT INTO orders.candles \
            (exchange, base, quote, time_bucket, open_price, high_price, low_price, close_price, volume) \
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .await
}
