mod db;
mod util;
mod web;

use db::connection;
use std::sync::Arc;
use structopt::StructOpt;
use util::market_data;
use web::server;

#[derive(Debug, StructOpt)]
pub struct Opt {
    /// resolution in milliseconds
    #[structopt(default_value = "2000")]
    resolution: i64,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    util::logging::init();
    let opt = Opt::from_args();

    let session = Arc::new(
        connection::builder()
            .await
            .expect("Failed to connect to database"),
    );

    let web = server::init(session.clone()).await;
    tokio::spawn(async { web.launch().await.unwrap() });

    market_data::subscribe(opt, &session).await;
}
