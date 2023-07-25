use crate::db::connection;
use crate::web::routes::*;
use rocket::fs::FileServer;
use rocket::{___internal_relative as relative, routes, Build, Rocket};

pub async fn init() -> Rocket<Build> {
    let session = connection::builder()
        .await
        .expect("Failed to connect to database");

    // Spawn Rocket server as a background task
    let rocket = rocket::build()
        .mount("/", routes![index, data, data_duration, trades, metrics])
        .mount("/", FileServer::from(relative!("public/")))
        .manage(session);
    rocket
}
