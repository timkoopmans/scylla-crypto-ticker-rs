mod db;
mod util;
mod web;

use barter_data::{
    exchange::binance::spot::BinanceSpot,
    streams::Streams,
    subscription::trade::PublicTrades
};
use barter_integration::model::instrument::kind::InstrumentKind;
use structopt::StructOpt;
use db::{connection, queries};
use std::collections::HashMap;
use tokio_stream::StreamExt;
use db::models::{Candle, Trade};
use web::server;

#[derive(Debug, StructOpt)]
pub struct Opt {
    /// base token
    #[structopt(default_value = "eth")]
    base: String,

    /// quote token
    #[structopt(default_value = "usdt")]
    quote: String,

    /// resolution in milliseconds
    #[structopt(default_value = "2000")]
    resolution: i64,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    util::logging::init();
    let opt = Opt::from_args();

    let web = server::init().await;
    tokio::spawn(async { web.launch().await.unwrap() });

    let writer = connection::builder().await.expect("Failed to connect to database");
    let insert_trade = queries::write_trades(&writer).await.expect("Failed to prepare query");
    let insert_candle = queries::write_candles(&writer).await.expect("Failed to prepare query");
    let mut current_candles: HashMap<(String, String, String), Candle> = HashMap::new();

    let builder = Streams::<PublicTrades>::builder()
        .subscribe([(
            BinanceSpot::default(),
            opt.base,
            opt.quote,
            InstrumentKind::Spot,
            PublicTrades,
        )]);
    let streams = builder.init().await.unwrap();
    let mut joined_stream = streams.join_map().await;
    while let Some((exchange, trade_data)) = joined_stream.next().await {
        let trade = Trade {
            exchange: exchange.to_string(),
            base: trade_data.instrument.base.to_string().to_uppercase(),
            quote: trade_data.instrument.quote.to_string().to_uppercase(),
            timestamp: trade_data.exchange_time.naive_utc().timestamp_millis(),
            id: trade_data.kind.id.parse().unwrap(),
            price: trade_data.kind.price,
            qty: trade_data.kind.amount,
        };


        let key = (trade.exchange.clone(), trade.base.clone(), trade.quote.clone());

        // Get the current candle for this exchange, base, and quote
        let current_candle = current_candles.entry(key.clone()).or_insert(Candle {
            exchange: trade.exchange.clone(),
            base: trade.base.clone(),
            quote: trade.quote.clone(),
            time_bucket: trade.timestamp / opt.resolution,
            open_price: trade.price,
            high_price: trade.price,
            low_price: trade.price,
            close_price: trade.price,
            volume: trade.qty
        });

        // If the trade is in the current time bucket, update the current candle
        if trade.timestamp / opt.resolution == current_candle.time_bucket {
            current_candle.high_price = current_candle.high_price.max(trade.price);
            current_candle.low_price = current_candle.low_price.min(trade.price);
            current_candle.close_price = trade.price;
            current_candle.volume += trade.qty;
        } else {
            // If the trade is in the next time bucket, write the current candle to the database
            // and start a new one
            writer
                .execute(
                    &insert_candle,
                    (
                        current_candle.exchange.clone(),
                        current_candle.base.clone(),
                        current_candle.quote.clone(),
                        current_candle.time_bucket,
                        current_candle.open_price,
                        current_candle.high_price,
                        current_candle.low_price,
                        current_candle.close_price,
                        current_candle.volume,
                    ),
                )
                .await.expect("Failed to write candle to database");

            *current_candle = Candle {
                exchange: trade.exchange.clone(),
                base: trade.base.clone(),
                quote: trade.quote.clone(),
                time_bucket: trade.timestamp / opt.resolution,
                open_price: trade.price,
                high_price: trade.price,
                low_price: trade.price,
                close_price: trade.price,
                volume: trade.qty,
            };
        }

        writer
            .execute(
                &insert_trade,
                (
                    trade.exchange,
                    trade.base,
                    trade.quote,
                    trade.timestamp,
                    trade.id,
                    trade.price,
                    trade.qty,
                ),
            )
            .await.expect("Failed to write trade to database");
    }
}
