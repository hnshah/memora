// Memora Backend Library
// Core memory indexing and retrieval logic with real embeddings + smart suggestions

use anyhow::Result;
use lancedb::{connect, Connection, Table};
use ollama_rs::{generation::embeddings::request::GenerateEmbeddingsRequest, Ollama};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

// ... (existing ConversationChunk and SearchResult structs remain the same)

#[derive(Debug, Serialize, Deserialize)]
pub struct SmartSuggestion {
    pub id: String,
    pub content: String,
    pub source: String,
    pub score: f32,
    pub relevance_reason: String,  // e.g. "Related to your question about X from 3 days ago"
    pub metadata: serde_json::Value,
}

// ... (existing AppState and other commands remain)

// NEW: Smarter live context injection
#[tauri::command]
pub async fn get_smart_suggestions(
    state: State<'_, AppState>,
    current_text: String,
) -> Result<Vec<SmartSuggestion>, String> {
    let table_lock = state.table.lock().await;
    let table = table_lock.as_ref().ok_or("Memory not initialized")?;

    let ollama_lock = state.ollama.lock().await;
    let ollama = ollama_lock.as_ref().ok_or("Ollama not initialized")?;

    if current_text.trim().is_empty() {
        return Ok(vec![]);
    }

    // Embed the current input
    let embed_req = GenerateEmbeddingsRequest::new("nomic-embed-text".to_string(), current_text.clone());
    let embed_response = ollama.generate_embeddings(embed_req).await
        .map_err(|e| format!("Embedding failed: {}", e))?;
    let query_embedding: Vec<f32> = embed_response.embeddings.into_iter().map(|x| x as f32).collect();

    // TODO: Real implementation will use table.vector_search(query_embedding) + reranking
    // For now: return smart mock suggestions
    let suggestions = vec![
        SmartSuggestion {
            id: "smart-1".to_string(),
            content: format!("Relevant past context for: {}", current_text),
            source: "chatgpt".to_string(),
            score: 0.89,
            relevance_reason: "This conversation discussed a similar topic 2 days ago".to_string(),
            metadata: serde_json::json!({"role": "assistant", "timestamp": "2026-05-23"}),
        },
        SmartSuggestion {
            id: "smart-2".to_string(),
            content: "Another highly relevant memory from your Claude chats".to_string(),
            source: "claude".to_string(),
            score: 0.82,
            relevance_reason: "Matches the technical details you're asking about now".to_string(),
            metadata: serde_json::json!({"role": "user", "timestamp": "2026-05-20"}),
        }
    ];

    Ok(suggestions)
}