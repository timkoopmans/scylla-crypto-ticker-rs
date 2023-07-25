use rocket::{get, State};
use scylla::{IntoTypedRows, Session};
use rocket::serde::json::Json;
use rocket::response::status;
use scylla::query::Query;
use rocket::http::Status;
use tracing::info;
use rocket::fs::NamedFile;
use std::path::Path;
use crate::db::models::{Candle, Trade};
use crate::util;

#[get("/")]
pub async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("public/index.html")).await.ok()
}

#[get("/data/<symbol>", rank = 1)]
pub async fn data(symbol: String, session: &State<Session>) -> Result<Json<Vec<Candle>>, status::Custom<String>> {
    let cql_query = Query::new(format!("SELECT * FROM orders.candles WHERE exchange = 'binance_spot' AND base = '{}' AND quote = 'USDT' LIMIT 300;", symbol));

    let rows = session
        .query(cql_query, ())
        .await
        .map_err(|err| status::Custom(Status::InternalServerError, err.to_string()))?
        .rows
        .unwrap_or_default();

    let result: Vec<Candle> = rows.into_typed()
        .filter_map(Result::ok)
        .collect();

    Ok(Json(result))
}

#[get("/data/<symbol>/<duration>", rank = 1)]
pub async fn data_duration(symbol: String, duration: String, session: &State<Session>) -> Result<Json<Vec<Candle>>, status::Custom<String>> {

    let time_bucket_from = util::parser::time_bucket_from(duration).unwrap();
    info!("time_bucket_from: {}", time_bucket_from);
    let cql_query = Query::new(format!("SELECT * FROM orders.candles WHERE exchange = 'binance_spot' AND base = '{}' AND quote = 'USDT' AND time_bucket >= {};", symbol, time_bucket_from));

    let rows = session
        .query(cql_query, ())
        .await
        .map_err(|err| status::Custom(Status::InternalServerError, err.to_string()))?
        .rows
        .unwrap_or_default();

    let result: Vec<Candle> = rows.into_typed()
        .filter_map(Result::ok)
        .collect();

    Ok(Json(result))
}

#[get("/trades/<symbol>", rank = 1)]
pub async fn trades(symbol: String, session: &State<Session>) -> Result<Json<Vec<Trade>>, status::Custom<String>> {
    let cql_query = Query::new(format!("SELECT * FROM orders.trades WHERE exchange = 'binance_spot' AND base = '{}' AND quote = 'USDT';", symbol));

    let rows = session
        .query(cql_query, ())
        .await
        .map_err(|err| status::Custom(Status::InternalServerError, err.to_string()))?
        .rows
        .unwrap_or_default();

    let result: Vec<Trade> = rows.into_typed()
        .filter_map(Result::ok)
        .collect();

    Ok(Json(result))
}
