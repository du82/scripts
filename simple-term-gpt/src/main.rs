use chatgpt::prelude::*;
use chatgpt::types::CompletionResponse;

#[tokio::main]
async fn main() -> Result<()> {
    let key = "ENTER YOUR CHATGPT KEY HERE";
    let client = ChatGPT::new(key)?;
    let prompt = "Write a poem about how unintuitive the OpenAI API is";

    println!("\nPrompt: {}", prompt);
    let response: CompletionResponse = client
        .send_message(prompt)
        .await?;
    println!("\nResponse: {}", response.message().content);

    Ok(())
}
