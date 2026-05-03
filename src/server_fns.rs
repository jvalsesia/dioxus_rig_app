use dioxus::prelude::*;

#[server]
pub async fn chat_with_agent(prompt: String) -> Result<String, ServerFnError> {
    use rig::{completion::Prompt, providers::openai};
    use rig::client::{ProviderClient, CompletionClient};

    let openai_client: openai::Client = match openai::Client::from_env() {
        Ok(client) => client,
        Err(_) => return Err(ServerFnError::new("OPENAI_API_KEY environment variable not set. Please set it to use the AI chat.")),
    };

    // Build the agent with a general AI personality
    let agent = openai_client
        .agent("gpt-4o-mini") // Using a fast, modern model
        .preamble("You are a helpful, friendly, and highly capable general AI assistant. You provide concise and accurate answers.")
        .build();

    // Prompt the agent
    agent.prompt(&prompt).await.map_err(|e| ServerFnError::new(format!("LLM Error: {}", e)))
}
