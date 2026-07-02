use eik::agent::Agent;
use eik::clients::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let api_key =
        std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set in .env or environment");

    let client = Client::new("https://api.openai.com/v1", api_key, "gpt-4o-mini");

    let agent = Agent::new(&client).system("You respond only in haiku form.");

    let response = agent
        .prompt("What is the capital of India?")
        .temperature(0.5)
        .max_tokens(50)
        .await?;

    println!("Agent: {}", response.text);

    let response = client
        .complete("Is Earth flat or not?")
        .temperature(1.0)
        .max_tokens(100)
        .await?;

    println!("Client: {}", response.text);

    Ok(())
}
