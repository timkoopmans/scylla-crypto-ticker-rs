use crate::db::models::{Candle, Trade};
use crate::db::queries;
use crate::Opt;

use barter_data::exchange::binance::futures::BinanceFuturesUsd;
use barter_data::streams::Streams;
use barter_data::subscription::trade::PublicTrades;
use barter_integration::model::instrument::kind::InstrumentKind;
use scylla::Session;
use std::collections::HashMap;
use std::sync::Arc;
use tokio_stream::StreamExt;

pub async fn subscribe(opt: Opt, session: &Arc<Session>) {
    let insert_trade = queries::write_trades(session)
        .await
        .expect("Failed to prepare query");
    let insert_candle = queries::write_candles(session)
        .await
        .expect("Failed to prepare query");
    let mut current_candles: HashMap<(String, String, String), Candle> = HashMap::new();

    let builder = Streams::<PublicTrades>::builder()
        .subscribe([(
            BinanceFuturesUsd::default(),
            "btc",
            "usdt",
            InstrumentKind::Perpetual,
            PublicTrades,
        )])
        // Separate WebSocket connection for ETH_USDT stream since it's very high volume
        .subscribe([(
            BinanceFuturesUsd::default(),
            "eth",
            "usdt",
            InstrumentKind::Perpetual,
            PublicTrades,
        )])
        // Lower volume Instruments can share a WebSocket connection
        .subscribe([
            (
                BinanceFuturesUsd::default(),
                "xrp",
                "usdt",
                InstrumentKind::Perpetual,
                PublicTrades,
            ),
            (
                BinanceFuturesUsd::default(),
                "bnb",
                "usdt",
                InstrumentKind::Perpetual,
                PublicTrades,
            ),
            (
                BinanceFuturesUsd::default(),
                "doge",
                "usdt",
                InstrumentKind::Perpetual,
                PublicTrades,
            ),
            (
                BinanceFuturesUsd::default(),
                "ada",
                "usdt",
                InstrumentKind::Perpetual,
                PublicTrades,
            ),
            (
                BinanceFuturesUsd::default(),
                "sol",
                "usdt",
                InstrumentKind::Perpetual,
                PublicTrades,
            ),
            (
                BinanceFuturesUsd::default(),
                "trx",
                "usdt",
                InstrumentKind::Perpetual,
                PublicTrades,
            ),
            (
                BinanceFuturesUsd::default(),
                "dot",
                "usdt",
                InstrumentKind::Perpetual,
                PublicTrades,
            ),
            (
                BinanceFuturesUsd::default(),
                "matic",
                "usdt",
                InstrumentKind::Perpetual,
                PublicTrades,
            ),
            (
                BinanceFuturesUsd::default(),
                "ltc",
                "usdt",
                InstrumentKind::Perpetual,
                PublicTrades,
            ),
            (
                BinanceFuturesUsd::default(),
                "shib",
                "usdt",
                InstrumentKind::Perpetual,
                PublicTrades,
            ),
            (
                BinanceFuturesUsd::default(),
                "uni",
                "usdt",
                InstrumentKind::Perpetual,
                PublicTrades,
            ),
            (
                BinanceFuturesUsd::default(),
                "avax",
                "usdt",
                InstrumentKind::Perpetual,
                PublicTrades,
            ),
            (
                BinanceFuturesUsd::default(),
                "link",
                "usdt",
                InstrumentKind::Perpetual,
                PublicTrades,
            ),
            (
                BinanceFuturesUsd::default(),
                "xmr",
                "usdt",
                InstrumentKind::Perpetual,
                PublicTrades,
            ),
        ]);
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

        let key = (
            trade.exchange.clone(),
            trade.base.clone(),
            trade.quote.clone(),
        );

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
            volume: trade.qty,
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
            session
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
                .await
                .expect("Failed to write candle to database");

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

        session
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
            .await
            .expect("Failed to write trade to database");
    }
}
