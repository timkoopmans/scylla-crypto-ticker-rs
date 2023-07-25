use rocket::serde::Serialize;
use scylla::FromRow;

#[derive(Clone, Debug, Serialize, FromRow)]
pub struct Trade {
    pub exchange: String,
    pub base: String,
    pub quote: String,
    pub timestamp: i64,
    pub id: i64,
    pub price: f64,
    pub qty: f64
}

#[derive(Clone, Debug, Serialize, FromRow)]
pub struct Candle {
    pub exchange: String,
    pub base: String,
    pub quote: String,
    pub time_bucket: i64,
    pub open_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub close_price: f64,
    pub volume: f64,
}
