mod db;
mod util;

use anyhow::Result;
use barter_data::{
    exchange::{binance::spot::BinanceSpot, ExchangeId},
    streams::Streams,
    subscription::book::OrderBooksL2,
};
use barter_integration::model::instrument::kind::InstrumentKind;
use structopt::StructOpt;
use db::{connection, queries};
use util::{charts, parsers};

#[derive(Debug, StructOpt)]
pub struct Opt {
    /// base token
    #[structopt(default_value = "eth")]
    base: String,

    /// quote token
    #[structopt(default_value = "usdt")]
    quote: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    util::logging::init();
    let opt = Opt::from_args();

    let writer = connection::builder().await?;
    let _insert_bids = queries::write_prices(&writer, "bids").await?;
    let insert_asks = queries::write_prices(&writer, "asks").await?;

    let mut streams = Streams::<OrderBooksL2>::builder()
        .subscribe([(
            BinanceSpot::default(),
            opt.base,
            opt.quote,
            InstrumentKind::Spot,
            OrderBooksL2,
        )])
        .init()
        .await?;

    let mut binance_stream = streams
        .select(ExchangeId::BinanceSpot)
        .ok_or(anyhow::anyhow!("Failed to select BinanceSpot"))?;

    let print_chart_task = tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(15));
        loop {
            interval.tick().await;

            let reader = connection::builder()
                .await
                .expect("Failed to connect to db");
            if let Err(e) = charts::print_depth_chart(&reader, "asks").await {
                eprintln!("Failed to print chart: {}", e);
            }
        }
    });

    while let Some(order_book) = binance_stream.recv().await {
        let asks = format!("{:?}", order_book.kind.asks);
        let levels_start = asks.find("levels: [").unwrap() + "levels: [".len();
        let levels_end = asks.rfind(']').unwrap();
        let levels_str = &asks[levels_start..levels_end];
        let ask_prices = parsers::extract_prices(levels_str);

        for price in ask_prices {
            writer
                .execute(
                    &insert_asks,
                    (
                        order_book.exchange_time,
                        order_book.exchange.to_string(),
                        order_book.instrument.base.to_string(),
                        order_book.instrument.quote.to_string(),
                        price,
                    ),
                )
                .await?;
        }
    }

    print_chart_task.await.unwrap();
    Ok(())
}
