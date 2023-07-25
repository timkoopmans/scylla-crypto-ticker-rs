use rocket::{___internal_relative as relative, Build, Rocket, routes};
use rocket::fs::FileServer;
use crate::db::connection;
use crate::web::routes::*;

pub async fn init() -> Rocket<Build> {
    let session = connection::builder().await.expect("Failed to connect to database");

    // Spawn Rocket server as a background task
    let rocket = rocket::build().mount("/", routes![index, data, data_duration, trades])
        .mount("/", FileServer::from(relative!("public/")))
        .manage(session);
    rocket
}
