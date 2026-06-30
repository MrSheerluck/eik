use eik::clients::{Client, Message};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let api_key =
        std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set in .env or environment");

    let client = Client::new("https://api.openai.com/v1", api_key, "gpt-4o-mini");

    let response = client
        .complete("Tell me a short fun fact about maths")
        .temperature(1.0)
        .max_tokens(100)
        .await?;

    println!("Complete: {}", response.text);

    let messages = vec![Message::user("What is the capital of France?")];

    let response = client.chat(&messages).await?;

    println!("Chat: {}", response.text);

    Ok(())
}
