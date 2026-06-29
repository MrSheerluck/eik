use eik::clients::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let api_key =
        std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set in .env or environment");

    let client = Client::new("https://api.openai.com/v1", api_key);

    let response = client.complete("how many r's in strawberry").await?;

    println!("{}", response.text);

    Ok(())
}
