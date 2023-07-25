use crate::web::routes::*;
use rocket::fs::FileServer;
use rocket::{___internal_relative as relative, routes, Build, Rocket};
use scylla::Session;
use std::sync::Arc;

pub async fn init(session: Arc<Session>) -> Rocket<Build> {
    // Spawn Rocket server as a background task
    rocket::build()
        .mount("/", routes![index, data, data_duration, trades, metrics])
        .mount("/", FileServer::from(relative!("public/")))
        .manage(session)
}
