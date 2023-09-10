// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::github::AuthenticatedGithubClient;
use tauri::async_runtime::block_on;

mod github;
mod task;

#[tauri::command]
async fn get_tasks(client: tauri::State<'_, AuthenticatedGithubClient>) -> Result<String, String> {
    let tasks = client.get_tasks().await;
    Ok(serde_json::to_string(&tasks).unwrap())
}

async fn init() -> AuthenticatedGithubClient {
    github::init().await
}

fn main() {
    let client = block_on(init());
    tauri::Builder::default()
        .manage(client)
        .invoke_handler(tauri::generate_handler![get_tasks])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
