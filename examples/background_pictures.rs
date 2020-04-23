use momentum::{Client, Fallible};

#[tokio::main]
async fn main() -> Fallible<()> {
    let connection = Client::new(include_str!("../client.id"));
    let feed = connection.get_feed().await?;
    for background in feed.backgrounds {
        println!(
            "background {}, with following url: {}",
            background.attribution, background.source_url
        )
    }
    Ok(())
}
