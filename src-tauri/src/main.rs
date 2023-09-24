// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod error;
mod github;
mod plugin;
mod task;

use crate::plugin::Plugin;
use crate::task::Task;
use error::TsugiError;
use std::{collections::HashSet, sync::Mutex};
use task::{GetTasksResponse, PluginStatus};
use tauri::api::notification::Notification;
use tauri::async_runtime::block_on;

async fn collect_results(plugins: &Vec<Box<dyn Plugin>>) -> GetTasksResponse {
    let mut tasks: Vec<Task> = Vec::new();
    let mut plugin_statuses: Vec<PluginStatus> = Vec::new();
    for plugin in plugins {
        let result = plugin.tasks().await;
        match result {
            Ok(plugin_tasks) => {
                tasks.extend(plugin_tasks);
                plugin_statuses.push(PluginStatus {
                    name: plugin.name(),
                    status: "ok".to_string(),
                    message: "".to_string(),
                });
            }
            Err(e) => {
                plugin_statuses.push(PluginStatus {
                    name: plugin.name(),
                    status: "error".to_string(),
                    message: format!("{:?}", e),
                });
            }
        }
    }
    GetTasksResponse {
        plugins: plugin_statuses,
        tasks,
    }
}

#[tauri::command]
async fn get_tasks(
    app: tauri::AppHandle,
    plugins: tauri::State<'_, Vec<Box<dyn Plugin>>>,
    previous_task_ids: tauri::State<'_, Mutex<HashSet<String>>>,
) -> Result<GetTasksResponse, TsugiError> {
    let results = collect_results(&plugins).await;
    let current_tasks = &results.tasks;
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

    Ok(results)
}

fn main() {
    let plugins = vec![block_on(github::init())];

    let previous_task_ids: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
    tauri::Builder::default()
        .manage(plugins)
        .manage(previous_task_ids)
        .invoke_handler(tauri::generate_handler![get_tasks])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
