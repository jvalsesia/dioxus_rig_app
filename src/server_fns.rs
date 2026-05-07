use dioxus::prelude::*;
use crate::Agent;
use dioxus_i18n::t;

#[server]
pub async fn chat_with_agent(prompt: String, preamble: String) -> Result<String, ServerFnError> {
    use rig::{completion::Prompt, providers::openai};
    use rig::client::{ProviderClient, CompletionClient};

    let openai_client: openai::Client = match openai::Client::from_env() {
        Ok(client) => client,
        Err(_) => return Err(ServerFnError::new("OPENAI_API_KEY environment variable not set. Please set it to use the AI chat.")),
    };

    // Build the agent with a custom AI personality
    let agent = openai_client
        .agent("gpt-4o-mini") // Using a fast, modern model
        .preamble(&preamble)
        .build();

    // Prompt the agent
    agent.prompt(&prompt).await.map_err(|e| ServerFnError::new(format!("LLM Error: {}", e)))
}

#[cfg(feature = "server")]
async fn get_or_create_table(lang: &str) -> Result<lancedb::Table, ServerFnError> {
    use lancedb::connect;
    use arrow_schema::{Schema, Field, DataType};
    use arrow_array::{StringArray, RecordBatch, RecordBatchIterator};
    use std::sync::Arc;
    use uuid::Uuid;
    
    // Ensure the data directory exists
    let db = connect("data/lancedb").execute().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    
    let table_names = db.table_names().execute().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    if table_names.contains(&"agents_v2".to_string()) {
        return db.open_table("agents_v2").execute().await.map_err(|e| ServerFnError::new(e.to_string()));
    }

    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Utf8, false),
        Field::new("name", DataType::Utf8, false),
        Field::new("specialty", DataType::Utf8, false),
        Field::new("n8n_webhook_send", DataType::Utf8, true),
        Field::new("n8n_webhook_receive", DataType::Utf8, true),
    ]));

    let name = if lang == "pt-BR" { "Assistente Geral" } else { "General Assistant" };
    let spec = if lang == "pt-BR" { "Você é um assistente geral de IA prestativo, amigável e altamente capaz. Você fornece respostas concisas e precisas." } else { "You are a helpful, friendly, and highly capable general AI assistant. You provide concise and accurate answers." };

    let id = Uuid::new_v4().to_string();
    let id_array = Arc::new(StringArray::from(vec![id]));
    let name_array = Arc::new(StringArray::from(vec![name.to_string()]));
    let spec_array = Arc::new(StringArray::from(vec![spec.to_string()]));
    let send_array = Arc::new(StringArray::from(vec![Option::<String>::None]));
    let recv_array = Arc::new(StringArray::from(vec![Option::<String>::None]));
    
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![id_array as _, name_array as _, spec_array as _, send_array as _, recv_array as _]
    ).map_err(|e| ServerFnError::new(e.to_string()))?;

    let reader = Box::new(RecordBatchIterator::new(vec![Ok(batch)], schema.clone())) as Box<dyn arrow_array::RecordBatchReader + Send>;
    let table = db.create_table("agents_v2", reader).execute().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    
    Ok(table)
}

#[server]
pub async fn get_agents(lang: String) -> Result<Vec<Agent>, ServerFnError> {
    use lancedb::query::ExecutableQuery;
    let table = get_or_create_table(&lang).await?;
    
    let mut stream = table.query().execute().await.map_err(|e: lancedb::Error| ServerFnError::new(e.to_string()))?;
    
    let mut agents = Vec::new();
    
    use futures::StreamExt;
    while let Some(batch_result) = stream.next().await {
        let batch = batch_result.map_err(|e| ServerFnError::new(e.to_string()))?;
        
        use arrow_array::{StringArray, Array};
        let ids = batch.column(0).as_any().downcast_ref::<StringArray>().unwrap();
        let names = batch.column(1).as_any().downcast_ref::<StringArray>().unwrap();
        let specialties = batch.column(2).as_any().downcast_ref::<StringArray>().unwrap();
        let sends = batch.column(3).as_any().downcast_ref::<StringArray>().unwrap();
        let recvs = batch.column(4).as_any().downcast_ref::<StringArray>().unwrap();
        
        for i in 0..batch.num_rows() {
            let n8n_webhook_send = if sends.is_null(i) { None } else { Some(sends.value(i).to_string()) };
            let n8n_webhook_receive = if recvs.is_null(i) { None } else { Some(recvs.value(i).to_string()) };

            agents.push(Agent {
                id: ids.value(i).to_string(),
                name: names.value(i).to_string(),
                specialty: specialties.value(i).to_string(),
                n8n_webhook_send,
                n8n_webhook_receive,
            });
        }
    }
    
    Ok(agents)
}

#[server]
pub async fn add_agent(name: String, specialty: String) -> Result<Agent, ServerFnError> {
    use arrow_array::{StringArray, RecordBatch, RecordBatchIterator};
    use std::sync::Arc;
    use uuid::Uuid;

    let table = get_or_create_table("en-US").await?;
    let schema = table.schema().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    
    let id = Uuid::new_v4().to_string();
    let id_array = Arc::new(StringArray::from(vec![id.clone()]));
    let name_array = Arc::new(StringArray::from(vec![name.clone()]));
    let spec_array = Arc::new(StringArray::from(vec![specialty.clone()]));
    let send_array = Arc::new(StringArray::from(vec![Option::<String>::None]));
    let recv_array = Arc::new(StringArray::from(vec![Option::<String>::None]));
    
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![id_array as _, name_array as _, spec_array as _, send_array as _, recv_array as _]
    ).map_err(|e| ServerFnError::new(e.to_string()))?;

    let reader = Box::new(RecordBatchIterator::new(vec![Ok(batch)], schema.clone())) as Box<dyn arrow_array::RecordBatchReader + Send>;
    table.add(reader).execute().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    
    Ok(Agent { id, name, specialty, n8n_webhook_send: None, n8n_webhook_receive: None })
}

#[server]
pub async fn delete_agent(id: String) -> Result<(), ServerFnError> {
    let table = get_or_create_table("en-US").await?;
    table.delete(&format!("id = '{}'", id)).await.map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(())
}

#[server]
pub async fn update_agent(id: String, name: String, specialty: String, n8n_webhook_send: Option<String>, n8n_webhook_receive: Option<String>) -> Result<Agent, ServerFnError> {
    use arrow_array::{StringArray, RecordBatch, RecordBatchIterator};
    use std::sync::Arc;

    let table = get_or_create_table("en-US").await?;
    let schema = table.schema().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    
    table.delete(&format!("id = '{}'", id)).await.map_err(|e| ServerFnError::new(e.to_string()))?;

    let id_array = Arc::new(StringArray::from(vec![id.clone()]));
    let name_array = Arc::new(StringArray::from(vec![name.clone()]));
    let spec_array = Arc::new(StringArray::from(vec![specialty.clone()]));
    let send_array = Arc::new(StringArray::from(vec![n8n_webhook_send.clone()]));
    let recv_array = Arc::new(StringArray::from(vec![n8n_webhook_receive.clone()]));
    
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![id_array as _, name_array as _, spec_array as _, send_array as _, recv_array as _]
    ).map_err(|e| ServerFnError::new(e.to_string()))?;

    let reader = Box::new(RecordBatchIterator::new(vec![Ok(batch)], schema.clone())) as Box<dyn arrow_array::RecordBatchReader + Send>;
    table.add(reader).execute().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    
    Ok(Agent { id, name, specialty, n8n_webhook_send, n8n_webhook_receive })
}

#[server]
pub async fn test_n8n_webhook(url: String) -> Result<String, ServerFnError> {
    let client = reqwest::Client::new();
    let res = client.post(&url)
        .json(&serde_json::json!({"test": true}))
        .send()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    
    if res.status().is_success() {
        Ok("Connection successful!".to_string())
    } else {
        Err(ServerFnError::new(format!("Received status code: {}", res.status())))
    }
}
