use eik::clients::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let api_key =
        std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set in .env or environment");

    let client = Client::new("https://api.openai.com/v1", api_key, "gpt-3.5-turbo");

    let response = client
        .complete("tell me a short fun fact about maths")
        .temperature(1.0)
        .max_tokens(100)
        .await?;

    println!("{}", response.text);

    Ok(())
}
