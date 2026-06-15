use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationChunk {
    pub id: String,
    pub content: String,
    pub source: String,
    pub role: String,
    pub timestamp: String,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub content: String,
    pub source: String,
    pub score: f32,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmartSuggestion {
    pub id: String,
    pub content: String,
    pub source: String,
    pub score: f32,
    pub relevance_reason: String,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    pub total_chunks: usize,
    pub sources: Vec<String>,
    pub embedding_model: String,
}

struct StoredChunk {
    chunk: ConversationChunk,
    embedding: Vec<f32>,
}

pub struct AppState {
    chunks: Mutex<Vec<StoredChunk>>,
    initialized: Mutex<bool>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            chunks: Mutex::new(Vec::new()),
            initialized: Mutex::new(false),
        }
    }
}

const EMBED_DIM: usize = 128;

fn compute_embedding(text: &str) -> Vec<f32> {
    let lower = text.to_lowercase();
    let mut emb = vec![0.0f32; EMBED_DIM];
    for (i, b) in lower.bytes().enumerate() {
        emb[i % EMBED_DIM] += b as f32;
        emb[(i * 7 + 13) % EMBED_DIM] += (b as f32) * 0.5;
    }
    let norm: f32 = emb.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 0.0 {
        for x in &mut emb {
            *x /= norm;
        }
    }
    emb
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let na: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let nb: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if na == 0.0 || nb == 0.0 {
        0.0
    } else {
        dot / (na * nb)
    }
}

fn f32_cmp(a: &f32, b: &f32) -> Ordering {
    b.partial_cmp(a).unwrap_or(Ordering::Equal)
}

#[tauri::command]
pub async fn initialize_memory(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let mut init = state.initialized.lock().await;
    *init = true;
    Ok("Memory initialized".to_string())
}

#[tauri::command]
pub async fn index_conversation(
    state: tauri::State<'_, AppState>,
    id: String,
    content: String,
    source: String,
    role: Option<String>,
    timestamp: Option<String>,
    metadata: Option<serde_json::Value>,
) -> Result<String, String> {
    let embedding = compute_embedding(&content);
    let chunk = ConversationChunk {
        id: id.clone(),
        content,
        source,
        role: role.unwrap_or_else(|| "user".to_string()),
        timestamp: timestamp.unwrap_or_default(),
        metadata: metadata.unwrap_or(serde_json::Value::Null),
    };
    state
        .chunks
        .lock()
        .await
        .push(StoredChunk { chunk, embedding });
    Ok(format!("Indexed {}", id))
}

#[tauri::command]
pub async fn search_memory(
    state: tauri::State<'_, AppState>,
    query: String,
    limit: Option<usize>,
) -> Result<Vec<SearchResult>, String> {
    let limit = limit.unwrap_or(5);
    let q_emb = compute_embedding(&query);
    let chunks = state.chunks.lock().await;
    let mut scored: Vec<(f32, usize)> = chunks
        .iter()
        .enumerate()
        .map(|(i, sc)| (cosine_similarity(&q_emb, &sc.embedding), i))
        .collect();
    scored.sort_by(|a, b| f32_cmp(&a.0, &b.0));
    Ok(scored
        .into_iter()
        .take(limit)
        .map(|(score, i)| {
            let sc = &chunks[i];
            SearchResult {
                id: sc.chunk.id.clone(),
                content: sc.chunk.content.clone(),
                source: sc.chunk.source.clone(),
                score,
                metadata: sc.chunk.metadata.clone(),
            }
        })
        .collect())
}

#[tauri::command]
pub async fn get_memory_stats(state: tauri::State<'_, AppState>) -> Result<MemoryStats, String> {
    let chunks = state.chunks.lock().await;
    let mut sources: Vec<String> = chunks.iter().map(|sc| sc.chunk.source.clone()).collect();
    sources.sort();
    sources.dedup();
    Ok(MemoryStats {
        total_chunks: chunks.len(),
        sources,
        embedding_model: "built-in (hash-128d)".to_string(),
    })
}

#[tauri::command]
pub async fn get_smart_suggestions(
    state: tauri::State<'_, AppState>,
    current_text: String,
) -> Result<Vec<SmartSuggestion>, String> {
    if current_text.trim().is_empty() {
        return Ok(vec![]);
    }
    let q_emb = compute_embedding(&current_text);
    let chunks = state.chunks.lock().await;
    let mut scored: Vec<(f32, usize)> = chunks
        .iter()
        .enumerate()
        .map(|(i, sc)| (cosine_similarity(&q_emb, &sc.embedding), i))
        .collect();
    scored.sort_by(|a, b| f32_cmp(&a.0, &b.0));
    Ok(scored
        .into_iter()
        .take(3)
        .enumerate()
        .map(|(j, (score, i))| {
            let sc = &chunks[i];
            SmartSuggestion {
                id: format!("smart-{}", j + 1),
                content: sc.chunk.content.clone(),
                source: sc.chunk.source.clone(),
                score,
                relevance_reason: format!("Related to your input from {}", sc.chunk.source),
                metadata: sc.chunk.metadata.clone(),
            }
        })
        .collect())
}
