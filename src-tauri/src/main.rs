// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashSet;

use crate::github::AuthenticatedGithubClient;
use tauri::async_runtime::block_on;

mod github;
mod task;

#[tauri::command]
async fn get_tasks(
    client: tauri::State<'_, AuthenticatedGithubClient>,
    previous_task_ids: tauri::State<'_, HashSet<String>>,
) -> Result<Vec<task::Task>, String> {
    let new_tasks = client.get_tasks().await;
    let new_task_ids = new_tasks
        .iter()
        .map(|t| &t.id)
        .cloned()
        .collect::<HashSet<_>>()
        .difference(&previous_task_ids)
        .cloned()
        .collect::<HashSet<_>>();
    // Log the new task IDs
    for id in new_task_ids.iter() {
        println!("New task: {id}");
    }

    Ok(new_tasks)
}

async fn init() -> AuthenticatedGithubClient {
    github::init().await
}

fn main() {
    let client = block_on(init());
    let previous_task_ids: HashSet<String> = HashSet::new();
    tauri::Builder::default()
        .manage(client)
        .manage(previous_task_ids)
        .invoke_handler(tauri::generate_handler![get_tasks])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
