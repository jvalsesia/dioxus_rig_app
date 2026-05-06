use dioxus::prelude::*;
use crate::Agent;

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
async fn get_or_create_table() -> Result<lancedb::Table, ServerFnError> {
    use lancedb::connect;
    use arrow_schema::{Schema, Field, DataType};
    use arrow_array::{StringArray, RecordBatch, RecordBatchIterator};
    use std::sync::Arc;
    use uuid::Uuid;
    
    // Ensure the data directory exists
    let db = connect("data/lancedb").execute().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    
    let table_names = db.table_names().execute().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    if table_names.contains(&"agents".to_string()) {
        return db.open_table("agents").execute().await.map_err(|e| ServerFnError::new(e.to_string()));
    }

    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Utf8, false),
        Field::new("name", DataType::Utf8, false),
        Field::new("specialty", DataType::Utf8, false),
    ]));

    // Create initial agent
    let id = Uuid::new_v4().to_string();
    let id_array = Arc::new(StringArray::from(vec![id]));
    let name_array = Arc::new(StringArray::from(vec!["General Assistant".to_string()]));
    let spec_array = Arc::new(StringArray::from(vec!["You are a helpful, friendly, and highly capable general AI assistant. You provide concise and accurate answers.".to_string()]));
    
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![id_array as _, name_array as _, spec_array as _]
    ).map_err(|e| ServerFnError::new(e.to_string()))?;

    let reader = Box::new(RecordBatchIterator::new(vec![Ok(batch)], schema.clone())) as Box<dyn arrow_array::RecordBatchReader + Send>;
    let table = db.create_table("agents", reader).execute().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    
    Ok(table)
}

#[server]
pub async fn get_agents() -> Result<Vec<Agent>, ServerFnError> {
    use lancedb::query::ExecutableQuery;
    let table = get_or_create_table().await?;
    
    let mut stream = table.query().execute().await.map_err(|e: lancedb::Error| ServerFnError::new(e.to_string()))?;
    
    let mut agents = Vec::new();
    
    use futures::StreamExt;
    while let Some(batch_result) = stream.next().await {
        let batch = batch_result.map_err(|e| ServerFnError::new(e.to_string()))?;
        
        use arrow_array::StringArray;
        let ids = batch.column(0).as_any().downcast_ref::<StringArray>().unwrap();
        let names = batch.column(1).as_any().downcast_ref::<StringArray>().unwrap();
        let specialties = batch.column(2).as_any().downcast_ref::<StringArray>().unwrap();
        
        for i in 0..batch.num_rows() {
            agents.push(Agent {
                id: ids.value(i).to_string(),
                name: names.value(i).to_string(),
                specialty: specialties.value(i).to_string(),
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

    let table = get_or_create_table().await?;
    let schema = table.schema().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    
    let id = Uuid::new_v4().to_string();
    let id_array = Arc::new(StringArray::from(vec![id.clone()]));
    let name_array = Arc::new(StringArray::from(vec![name.clone()]));
    let spec_array = Arc::new(StringArray::from(vec![specialty.clone()]));
    
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![id_array as _, name_array as _, spec_array as _]
    ).map_err(|e| ServerFnError::new(e.to_string()))?;

    let reader = Box::new(RecordBatchIterator::new(vec![Ok(batch)], schema.clone())) as Box<dyn arrow_array::RecordBatchReader + Send>;
    table.add(reader).execute().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    
    Ok(Agent { id, name, specialty })
}

#[server]
pub async fn delete_agent(id: String) -> Result<(), ServerFnError> {
    let table = get_or_create_table().await?;
    table.delete(&format!("id = '{}'", id)).await.map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(())
}

#[server]
pub async fn update_agent(id: String, name: String, specialty: String) -> Result<Agent, ServerFnError> {
    use arrow_array::{StringArray, RecordBatch, RecordBatchIterator};
    use std::sync::Arc;

    let table = get_or_create_table().await?;
    let schema = table.schema().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    
    table.delete(&format!("id = '{}'", id)).await.map_err(|e| ServerFnError::new(e.to_string()))?;

    let id_array = Arc::new(StringArray::from(vec![id.clone()]));
    let name_array = Arc::new(StringArray::from(vec![name.clone()]));
    let spec_array = Arc::new(StringArray::from(vec![specialty.clone()]));
    
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![id_array as _, name_array as _, spec_array as _]
    ).map_err(|e| ServerFnError::new(e.to_string()))?;

    let reader = Box::new(RecordBatchIterator::new(vec![Ok(batch)], schema.clone())) as Box<dyn arrow_array::RecordBatchReader + Send>;
    table.add(reader).execute().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    
    Ok(Agent { id, name, specialty })
}
