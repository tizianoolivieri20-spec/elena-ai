// Tauri main.rs for Elena AI
#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;
use uuid::Uuid;
use chrono::Local;

mod ollama;
mod database;
mod elena;

use ollama::OllamaClient;
use elena::ElenaPersonality;

// Message structure for chat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    id: String,
    role: String, // "user" or "assistant"
    content: String,
    timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationState {
    messages: Vec<ChatMessage>,
    user_mood: String,
    conversation_context: String,
}

impl Default for ConversationState {
    fn default() -> Self {
        Self {
            messages: vec![],
            user_mood: "neutral".to_string(),
            conversation_context: String::new(),
        }
    }
}

// App state
pub struct AppState {
    conversation: Mutex<ConversationState>,
    ollama_client: OllamaClient,
    elena: ElenaPersonality,
}

// Commands
#[tauri::command]
async fn send_message(
    message: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let mut conv = state.conversation.lock().unwrap();

    // Add user message
    let user_msg = ChatMessage {
        id: Uuid::new_v4().to_string(),
        role: "user".to_string(),
        content: message.clone(),
        timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };
    conv.messages.push(user_msg);

    // Detect user mood
    conv.user_mood = state.elena.detect_mood(&message);

    // Build prompt with Elena's personality
    let system_prompt = state.elena.get_system_prompt(&conv.user_mood);
    let context = state.elena.build_context(&conv.messages);

    // Get response from Ollama
    let response = state
        .ollama_client
        .generate(&system_prompt, &context)
        .await
        .map_err(|e| e.to_string())?;

    // Add Elena's response
    let elena_msg = ChatMessage {
        id: Uuid::new_v4().to_string(),
        role: "assistant".to_string(),
        content: response.clone(),
        timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };
    conv.messages.push(elena_msg);

    Ok(response)
}

#[tauri::command]
fn get_conversation(state: State<'_, AppState>) -> Vec<ChatMessage> {
    state.conversation.lock().unwrap().messages.clone()
}

#[tauri::command]
fn clear_conversation(state: State<'_, AppState>) {
    let mut conv = state.conversation.lock().unwrap();
    conv.messages.clear();
    conv.user_mood = "neutral".to_string();
}

#[tauri::command]
fn get_user_mood(state: State<'_, AppState>) -> String {
    state.conversation.lock().unwrap().user_mood.clone()
}

#[tauri::command]
fn set_user_mood(mood: String, state: State<'_, AppState>) {
    state.conversation.lock().unwrap().user_mood = mood;
}

#[tokio::main]
async fn main() {
    let ollama_client = OllamaClient::new("http://localhost:11434".to_string());
    let elena = ElenaPersonality::new("Tiziano O.");

    let app_state = AppState {
        conversation: Mutex::new(ConversationState::default()),
        ollama_client,
        elena,
    };

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            send_message,
            get_conversation,
            clear_conversation,
            get_user_mood,
            set_user_mood,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
