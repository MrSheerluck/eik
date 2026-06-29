use eik::clients::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key =
        std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable not set");

    let client = Client::new("https://openrouter.ai/api/v1", api_key);

    let response = client.complete("Who are you?").await?;

    println!("{}", response.text);

    Ok(())
}
