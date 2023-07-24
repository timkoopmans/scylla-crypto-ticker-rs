use crate::db::queries;
use lowcharts::plot;
use scylla::Session;

pub async fn print_depth_chart(
    session: &Session,
    side: &str,
) -> anyhow::Result<(), Box<dyn std::error::Error>> {
    let prices = queries::read_prices(session, side).await?;
    if prices.is_empty() {
        return Ok(());
    }

    let options = plot::HistogramOptions {
        intervals: 16,
        ..Default::default()
    };
    let histogram = plot::Histogram::new(&prices, options);
    print!("{}", histogram);

    Ok(())
}
