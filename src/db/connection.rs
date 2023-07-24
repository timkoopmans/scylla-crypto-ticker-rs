use scylla::transport::errors::NewSessionError;
use scylla::{Session, SessionBuilder};

pub async fn builder() -> anyhow::Result<Session, NewSessionError> {
    SessionBuilder::new()
        .known_node("127.0.0.1:9042")
        .build()
        .await
}
