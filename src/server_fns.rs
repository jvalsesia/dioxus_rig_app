use dioxus::prelude::*;
use crate::{Agent, Skill};

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

#[server]
pub async fn test_evolution_connection(url: String, api_key: String) -> Result<String, ServerFnError> {
    let client = reqwest::Client::new();
    let endpoint = format!("{}/instance/fetchInstances", url.trim_end_matches('/'));
    let res = client.get(&endpoint)
        .header("apikey", &api_key)
        .send()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    if res.status().is_success() {
        Ok("✓ Connection successful!".to_string())
    } else {
        Err(ServerFnError::new(format!("Status {}", res.status())))
    }
}

#[server]
pub async fn verify_evolution_instance(url: String, api_key: String, instance: String) -> Result<String, ServerFnError> {
    let client = reqwest::Client::new();
    let endpoint = format!("{}/instance/connectionState/{}", url.trim_end_matches('/'), instance);
    let res = client.get(&endpoint)
        .header("apikey", &api_key)
        .send()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    if res.status().is_success() {
        use std::time::{SystemTime, UNIX_EPOCH};
        let secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        Ok(format!("{:02}:{:02}:{:02}", (secs % 86400) / 3600, (secs % 3600) / 60, secs % 60))
    } else {
        Err(ServerFnError::new(format!("Status {}", res.status())))
    }
}

#[cfg(feature = "server")]
fn default_skill_seeds(lang: &str) -> Vec<(String, String, String, String)> {
    use uuid::Uuid;
    let pt = lang == "pt-BR";
    let s = |name_en: &str, name_pt: &str, desc_en: &str, desc_pt: &str, cat: &str| {
        (
            Uuid::new_v4().to_string(),
            if pt { name_pt.to_string() } else { name_en.to_string() },
            if pt { desc_pt.to_string() } else { desc_en.to_string() },
            cat.to_string(),
        )
    };
    vec![
        s("Search information", "Buscar informações", "Searches and returns information from documents, knowledge bases or the web.", "Pesquisa e retorna informações de documentos, bases de conhecimento ou da web.", "research"),
        s("Perform calculations", "Realizar cálculos", "Performs mathematical and financial calculations accurately.", "Executa cálculos matemáticos e financeiros com precisão.", "utilities"),
        s("Check schedule", "Consultar agenda", "Checks appointments, events and time availability.", "Consulta compromissos, eventos e disponibilidade de horários.", "productivity"),
        s("Send email", "Enviar e-mail", "Composes and sends emails through configured accounts.", "Redige e envia e-mails através de contas configuradas.", "communication"),
        s("Query data (API)", "Consultar dados (API)", "Queries data in external systems through integrated APIs.", "Consulta dados em sistemas externos via APIs integradas.", "integrations"),
        s("Summarize documents", "Resumir documentos", "Generates concise summaries from long texts and documents.", "Gera resumos concisos a partir de textos e documentos longos.", "research"),
        s("Translate texts", "Traduzir textos", "Translates content between multiple languages with contextual accuracy.", "Traduz conteúdos entre múltiplos idiomas com precisão contextual.", "communication"),
        s("Generate reports", "Gerar relatórios", "Compiles data and generates structured reports on demand.", "Compila dados e gera relatórios estruturados sob demanda.", "productivity"),
        s("Sentiment analysis", "Análise de sentimento", "Identifies the emotional tone of messages and reviews.", "Identifica o tom emocional de mensagens e avaliações.", "research"),
        s("Schedule meetings", "Agendar reuniões", "Creates and manages calendar events automatically.", "Cria e gerencia eventos no calendário automaticamente.", "productivity"),
        s("Send SMS", "Enviar SMS", "Sends SMS messages through configured providers.", "Envia mensagens SMS através de provedores configurados.", "communication"),
        s("Custom webhooks", "Webhooks personalizados", "Triggers webhooks to external systems based on events.", "Dispara webhooks para sistemas externos baseado em eventos.", "integrations"),
    ]
}

#[cfg(feature = "server")]
async fn get_or_create_skills_table(lang: &str) -> Result<lancedb::Table, ServerFnError> {
    use lancedb::connect;
    use arrow_schema::{Schema, Field, DataType};
    use arrow_array::{StringArray, RecordBatch, RecordBatchIterator};
    use std::sync::Arc;

    let db = connect("data/lancedb").execute().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    let table_names = db.table_names().execute().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    if table_names.contains(&"skills_v1".to_string()) {
        return db.open_table("skills_v1").execute().await.map_err(|e| ServerFnError::new(e.to_string()));
    }

    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Utf8, false),
        Field::new("name", DataType::Utf8, false),
        Field::new("description", DataType::Utf8, false),
        Field::new("category", DataType::Utf8, false),
    ]));

    let seeds = default_skill_seeds(lang);
    let ids: Vec<String> = seeds.iter().map(|t| t.0.clone()).collect();
    let names: Vec<String> = seeds.iter().map(|t| t.1.clone()).collect();
    let descs: Vec<String> = seeds.iter().map(|t| t.2.clone()).collect();
    let cats: Vec<String> = seeds.iter().map(|t| t.3.clone()).collect();

    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(StringArray::from(ids)) as _,
            Arc::new(StringArray::from(names)) as _,
            Arc::new(StringArray::from(descs)) as _,
            Arc::new(StringArray::from(cats)) as _,
        ],
    ).map_err(|e| ServerFnError::new(e.to_string()))?;

    let reader = Box::new(RecordBatchIterator::new(vec![Ok(batch)], schema.clone())) as Box<dyn arrow_array::RecordBatchReader + Send>;
    let table = db.create_table("skills_v1", reader).execute().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(table)
}

#[server]
pub async fn get_skills(lang: String) -> Result<Vec<Skill>, ServerFnError> {
    use lancedb::query::ExecutableQuery;
    use futures::StreamExt;
    use arrow_array::{StringArray, Array};

    let table = get_or_create_skills_table(&lang).await?;
    let mut stream = table.query().execute().await.map_err(|e: lancedb::Error| ServerFnError::new(e.to_string()))?;
    let mut skills = Vec::new();

    while let Some(batch_result) = stream.next().await {
        let batch = batch_result.map_err(|e| ServerFnError::new(e.to_string()))?;
        let ids = batch.column(0).as_any().downcast_ref::<StringArray>().unwrap();
        let names = batch.column(1).as_any().downcast_ref::<StringArray>().unwrap();
        let descs = batch.column(2).as_any().downcast_ref::<StringArray>().unwrap();
        let cats = batch.column(3).as_any().downcast_ref::<StringArray>().unwrap();
        for i in 0..batch.num_rows() {
            skills.push(Skill {
                id: ids.value(i).to_string(),
                name: names.value(i).to_string(),
                description: descs.value(i).to_string(),
                category: cats.value(i).to_string(),
            });
        }
    }

    Ok(skills)
}

#[server]
pub async fn add_skill(name: String, description: String, category: String) -> Result<Skill, ServerFnError> {
    use arrow_array::{StringArray, RecordBatch, RecordBatchIterator};
    use std::sync::Arc;
    use uuid::Uuid;

    let table = get_or_create_skills_table("en-US").await?;
    let schema = table.schema().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    let id = Uuid::new_v4().to_string();
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(StringArray::from(vec![id.clone()])) as _,
            Arc::new(StringArray::from(vec![name.clone()])) as _,
            Arc::new(StringArray::from(vec![description.clone()])) as _,
            Arc::new(StringArray::from(vec![category.clone()])) as _,
        ],
    ).map_err(|e| ServerFnError::new(e.to_string()))?;

    let reader = Box::new(RecordBatchIterator::new(vec![Ok(batch)], schema.clone())) as Box<dyn arrow_array::RecordBatchReader + Send>;
    table.add(reader).execute().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(Skill { id, name, description, category })
}

#[server]
pub async fn delete_skill(id: String) -> Result<(), ServerFnError> {
    let table = get_or_create_skills_table("en-US").await?;
    table.delete(&format!("id = '{}'", id)).await.map_err(|e| ServerFnError::new(e.to_string()))?;
    // Cascade: remove any associations referencing this skill
    if let Ok(join) = get_or_create_agent_skills_table().await {
        let _ = join.delete(&format!("skill_id = '{}'", id)).await;
    }
    Ok(())
}

#[cfg(feature = "server")]
async fn get_or_create_agent_skills_table() -> Result<lancedb::Table, ServerFnError> {
    use lancedb::connect;
    use arrow_schema::{Schema, Field, DataType};
    use arrow_array::{StringArray, RecordBatch, RecordBatchIterator};
    use std::sync::Arc;

    let db = connect("data/lancedb").execute().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    let table_names = db.table_names().execute().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    if table_names.contains(&"agent_skills".to_string()) {
        return db.open_table("agent_skills").execute().await.map_err(|e| ServerFnError::new(e.to_string()));
    }

    let schema = Arc::new(Schema::new(vec![
        Field::new("agent_id", DataType::Utf8, false),
        Field::new("skill_id", DataType::Utf8, false),
    ]));

    // LanceDB requires at least one row to create a table; seed with a sentinel and immediately delete it.
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(StringArray::from(vec!["__seed__".to_string()])) as _,
            Arc::new(StringArray::from(vec!["__seed__".to_string()])) as _,
        ],
    ).map_err(|e| ServerFnError::new(e.to_string()))?;

    let reader = Box::new(RecordBatchIterator::new(vec![Ok(batch)], schema.clone())) as Box<dyn arrow_array::RecordBatchReader + Send>;
    let table = db.create_table("agent_skills", reader).execute().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    table.delete("agent_id = '__seed__'").await.map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(table)
}

#[server]
pub async fn get_agent_skill_ids(agent_id: String) -> Result<Vec<String>, ServerFnError> {
    use lancedb::query::{ExecutableQuery, QueryBase};
    use futures::StreamExt;
    use arrow_array::{StringArray, Array};

    let table = get_or_create_agent_skills_table().await?;
    let mut stream = table
        .query()
        .only_if(format!("agent_id = '{}'", agent_id))
        .execute()
        .await
        .map_err(|e: lancedb::Error| ServerFnError::new(e.to_string()))?;

    let mut ids = Vec::new();
    while let Some(batch_result) = stream.next().await {
        let batch = batch_result.map_err(|e| ServerFnError::new(e.to_string()))?;
        let skill_ids = batch.column(1).as_any().downcast_ref::<StringArray>().unwrap();
        for i in 0..batch.num_rows() {
            ids.push(skill_ids.value(i).to_string());
        }
    }
    Ok(ids)
}

#[server]
pub async fn set_agent_skills(agent_id: String, skill_ids: Vec<String>) -> Result<(), ServerFnError> {
    use arrow_array::{StringArray, RecordBatch, RecordBatchIterator};
    use std::sync::Arc;

    let table = get_or_create_agent_skills_table().await?;
    table.delete(&format!("agent_id = '{}'", agent_id)).await.map_err(|e| ServerFnError::new(e.to_string()))?;

    if skill_ids.is_empty() {
        return Ok(());
    }

    let schema = table.schema().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    let agent_col: Vec<String> = std::iter::repeat(agent_id).take(skill_ids.len()).collect();

    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(StringArray::from(agent_col)) as _,
            Arc::new(StringArray::from(skill_ids)) as _,
        ],
    ).map_err(|e| ServerFnError::new(e.to_string()))?;

    let reader = Box::new(RecordBatchIterator::new(vec![Ok(batch)], schema.clone())) as Box<dyn arrow_array::RecordBatchReader + Send>;
    table.add(reader).execute().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(())
}

#[server]
pub async fn associate_evolution_webhook(
    url: String,
    api_key: String,
    instance: String,
    webhook_url: String,
    ignore_groups: bool,
) -> Result<String, ServerFnError> {
    let client = reqwest::Client::new();
    let endpoint = format!("{}/webhook/set/{}", url.trim_end_matches('/'), instance);
    let events = if ignore_groups {
        serde_json::json!(["MESSAGES_UPSERT"])
    } else {
        serde_json::json!(["MESSAGES_UPSERT", "GROUPS_UPSERT"])
    };
    let body = serde_json::json!({
        "url": webhook_url,
        "webhook_by_events": false,
        "webhook_base64": false,
        "events": events
    });
    let res = client.post(&endpoint)
        .header("apikey", &api_key)
        .json(&body)
        .send()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    if res.status().is_success() {
        Ok("✓ Webhook associated successfully!".to_string())
    } else {
        Err(ServerFnError::new(format!("Status {}", res.status())))
    }
}
