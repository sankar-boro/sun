mod try_join;

use hyper::{Client, client::HttpConnector, Uri};
use hyper::body::HttpBody as _;
use tokio::io::{stdout, AsyncWriteExt as _};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Still inside `async fn main`...
    let client = Client::new();

    // Parse an `http::Uri`...
    let uri_1: Uri = "http://localhost:3000".parse()?;
    let uri_2: Uri = "http://localhost:3000/about".parse()?;

    let _ = try_join::try_join(fetch_thing(client.clone(), uri_1), fetch_thing(client.clone(), uri_2)).await?;
    Ok(())
}

async fn fetch_thing(client: Client<HttpConnector>, uri: Uri) -> std::io::Result<()> {
    let mut resp = client.get(uri).await.unwrap();
    // println!("Response: {}", resp.status());
    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk.unwrap()).await?;
    }
    Ok(())
}