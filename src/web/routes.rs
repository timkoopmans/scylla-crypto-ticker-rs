use crate::db::models::{Candle, Metric, Trade};
use crate::util;
use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{get, State};
use scylla::query::Query;
use scylla::{IntoTypedRows, Metrics, Session};
use std::path::Path;
use std::sync::Arc;
use tracing::{error, info};

#[get("/")]
pub async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("public/index.html")).await.ok()
}

#[get("/data/<symbol>", rank = 1)]
pub async fn data(
    symbol: String,
    session: &State<Arc<Session>>,
) -> Result<Json<Vec<Candle>>, status::Custom<String>> {
    let cql_query = Query::new(format!("SELECT * FROM orders.candles WHERE exchange = 'binance_futures_usd' AND base = '{}' AND quote = 'USDT' LIMIT 300;", symbol));

    let rows = session
        .query(cql_query, ())
        .await
        .map_err(|err| status::Custom(Status::InternalServerError, err.to_string()))?
        .rows
        .unwrap_or_default();

    let result: Vec<Candle> = rows.into_typed().filter_map(Result::ok).collect();

    Ok(Json(result))
}

#[get("/data/<symbol>/<duration>", rank = 1)]
pub async fn data_duration(
    symbol: String,
    duration: String,
    session: &State<Arc<Session>>,
) -> Result<Json<Vec<Candle>>, status::Custom<String>> {
    let time_bucket_from = util::parser::time_bucket_from(duration).unwrap();
    info!("time_bucket_from: {}", time_bucket_from);
    let cql_query = Query::new(format!("SELECT * FROM orders.candles WHERE exchange = 'binance_futures_usd' AND base = '{}' AND quote = 'USDT' AND time_bucket >= {};", symbol, time_bucket_from));

    let rows = session
        .query(cql_query, ())
        .await
        .map_err(|err| status::Custom(Status::InternalServerError, err.to_string()))?
        .rows
        .unwrap_or_default();

    let result: Vec<Candle> = rows.into_typed().filter_map(Result::ok).collect();

    Ok(Json(result))
}

#[get("/trades/<symbol>", rank = 1)]
pub async fn trades(
    symbol: String,
    session: &State<Arc<Session>>,
) -> Result<Json<Vec<Trade>>, status::Custom<String>> {
    let cql_query = Query::new(format!("SELECT * FROM orders.trades WHERE exchange = 'binance_futures_usd' AND base = '{}' AND quote = 'USDT';", symbol));

    let rows = session
        .query(cql_query, ())
        .await
        .map_err(|err| status::Custom(Status::InternalServerError, err.to_string()))?
        .rows
        .unwrap_or_default();

    let result: Vec<Trade> = rows.into_typed().filter_map(Result::ok).collect();

    Ok(Json(result))
}

#[get("/metrics", rank = 1)]
pub async fn metrics(
    session: &State<Arc<Session>>,
) -> Result<Json<Metric>, status::Custom<String>> {
    let metrics: Arc<Metrics> = session.get_metrics();

    let mean_latency = match metrics.get_latency_avg_ms() {
        Ok(latency) => latency,
        Err(e) => {
            error!("Failed to get mean latency: {:?}", e);
            return Err(status::Custom(
                Status::InternalServerError,
                format!("Failed to get mean latency: {:?}", e),
            ));
        }
    };

    let p99_latency = match metrics.get_latency_percentile_ms(99.9) {
        Ok(latency) => latency,
        Err(e) => {
            error!("Failed to get p99 latency: {:?}", e);
            return Err(status::Custom(
                Status::InternalServerError,
                format!("Failed to get p99 latency: {:?}", e),
            ));
        }
    };

    let metric = Metric {
        queries_requested: metrics.get_queries_num(),
        iter_queries_requested: metrics.get_queries_iter_num(),
        errors_occurred: metrics.get_errors_num(),
        iter_errors_occurred: metrics.get_errors_iter_num(),
        mean_latency,
        p99_latency,
    };

    Ok(Json(metric))
}
