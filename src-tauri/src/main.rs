// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod error;
mod github;
mod plugin;
mod storage;
mod task;

use crate::plugin::Plugin;
use error::TsugiError;
use github::{AuthenticatedGithubClient, GitHubPrAuthorPlugin, GitHubPrReviewPlugin};
use std::fs;
use storage::Storage;
use task::{GetTasksResponse, PluginStatus};
use tauri::async_runtime::block_on;
use tauri::Manager;

async fn update_tasks(storage: &Storage, plugin: &Box<dyn Plugin>) -> Result<(), TsugiError> {
    storage.update(&plugin.name(), plugin.tasks().await?)
}

async fn collect_results(storage: &Storage, plugins: &Vec<Box<dyn Plugin>>) -> GetTasksResponse {
    let mut statuses: Vec<PluginStatus> = Vec::new();
    // TODO: Run all plugins in parallel
    for plugin in plugins {
        match update_tasks(&storage, &plugin).await {
            Ok(()) => {
                statuses.push(PluginStatus {
                    name: plugin.name(),
                    status: "ok".to_string(),
                    message: "".to_string(),
                });
            }
            Err(e) => {
                statuses.push(PluginStatus {
                    name: plugin.name(),
                    status: "error".to_string(),
                    message: format!("{:?}", e),
                });
            }
        }
    }

    let tasks = match storage.all_tasks() {
        Ok(tasks) => tasks,
        Err(e) => {
            statuses.push(PluginStatus {
                name: "storage".to_string(),
                status: "error".to_string(),
                message: format!("{:?}", e),
            });
            Vec::new()
        }
    };

    GetTasksResponse { statuses, tasks }
}

#[tauri::command]
async fn get_tasks(
    plugins: tauri::State<'_, Vec<Box<dyn Plugin>>>,
    storage: tauri::State<'_, Storage>,
) -> Result<GetTasksResponse, TsugiError> {
    let results = collect_results(&storage, &plugins).await;
    Ok(results)
}

fn main() {
    let client = block_on(AuthenticatedGithubClient::new());
    let plugins = vec![
        GitHubPrReviewPlugin::new(&client),
        GitHubPrAuthorPlugin::new(&client),
    ];

    tauri::Builder::default()
        .setup(|app| {
            let data_dir = app.path_resolver().app_local_data_dir().unwrap();
            fs::create_dir_all(&data_dir)?;
            let identifier = &app.config().tauri.bundle.identifier;
            let storage = Storage::new(&data_dir, identifier)?;
            app.manage(storage);
            Ok(())
        })
        .manage(plugins)
        .invoke_handler(tauri::generate_handler![get_tasks])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
