use crate::{Client, Fallible};

static CLIENT_ID: &str = include_str!("../client.id");

#[tokio::test]
async fn run_get_feed() -> Fallible<()> {
    let connection = Client::new(CLIENT_ID);
    connection.get_feed().await?;
    Ok(())
}
