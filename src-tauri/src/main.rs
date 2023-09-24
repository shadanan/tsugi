// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod error;
mod github;
mod plugin;
mod task;

use crate::error::TsugiError;
use crate::plugin::Plugin;
use crate::task::Task;
use std::{collections::HashSet, sync::Mutex};
use tauri::api::notification::Notification;
use tauri::async_runtime::block_on;

#[tauri::command]
async fn get_tasks(
    app: tauri::AppHandle,
    client: tauri::State<'_, Box<dyn Plugin>>,
    previous_task_ids: tauri::State<'_, Mutex<HashSet<String>>>,
) -> Result<Vec<Task>, TsugiError> {
    let current_tasks = client.get_tasks().await?;
    let new_task_ids = current_tasks
        .iter()
        .map(|t| t.id())
        .collect::<HashSet<_>>()
        .difference(&previous_task_ids.lock().unwrap())
        .cloned()
        .collect::<HashSet<_>>();
    let identifier = &app.config().tauri.bundle.identifier;
    current_tasks.iter().for_each(|t| {
        if new_task_ids.contains(&t.id()) {
            let result = Notification::new(identifier)
                .title(format!("New {}", t.kind))
                .body(&t.title)
                .show();
            if let Err(e) = result {
                println!("Error showing notification: {:?}", e);
            }
        }
    });
    previous_task_ids.lock().unwrap().extend(new_task_ids);

    Ok(current_tasks)
}

fn main() {
    let client = block_on(github::init());

    let previous_task_ids: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
    tauri::Builder::default()
        .manage(client)
        .manage(previous_task_ids)
        .invoke_handler(tauri::generate_handler![get_tasks])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
