use crate::{Client, Error};
use fehler::throws;

static CLIENT_ID: &str = include_str!("../client.id");

#[throws]
#[tokio::test]
async fn run_get_feed() {
    let connection = Client::new(CLIENT_ID);
    connection.get_feed().await?;
}
