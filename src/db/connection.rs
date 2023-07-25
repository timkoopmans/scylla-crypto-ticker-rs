use scylla::{Session, SessionBuilder};
use std::env;
use anyhow::{Error, Result};

pub async fn builder() -> Result<Session> {
    let database_url = env::var("DATABASE_URL")?;

    SessionBuilder::new()
        .known_node(&database_url)
        .build()
        .await
        .map_err(Error::from)
}
