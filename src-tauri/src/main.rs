// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod error;
mod github;
mod plugin;
mod storage;
mod task;

use error::TsugiError;
use github::{AuthenticatedGithubClient, GitHubPrAuthorPlugin, GitHubPrReviewPlugin};
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use storage::Storage;
use task::GetTasksResponse;
use tauri::api::notification::Notification;
use tauri::async_runtime::block_on;
use tauri::{App, Manager};

struct AppLocalDataDir(PathBuf);

#[tauri::command]
async fn get_tasks(
    data_dir: tauri::State<'_, AppLocalDataDir>,
) -> Result<GetTasksResponse, TsugiError> {
    let storage = Storage::new(&data_dir.0)?;
    match storage.all_tasks() {
        Ok(tasks) => Ok(GetTasksResponse { tasks }),
        Err(e) => Err(e),
    }
}

fn poll(data_dir: &PathBuf, identifier: &str) -> Result<Sender<&'static str>, TsugiError> {
    let (tx, rx) = std::sync::mpsc::channel::<&str>();

    let client = block_on(AuthenticatedGithubClient::new());
    let plugins = vec![
        GitHubPrReviewPlugin::new(&client),
        GitHubPrAuthorPlugin::new(&client),
    ];

    let storage = Storage::new(&data_dir)?.ensure_schema()?;
    let identifier = identifier.to_string();

    thread::spawn(move || {
        while let Ok(_) = rx.recv() {
            println!("⏰ Updating tasks...");
            for plugin in &plugins {
                let name = plugin.name();

                let tasks = match block_on(plugin.tasks()) {
                    Ok(tasks) => tasks,
                    Err(error) => {
                        println!("🛑 Error getting tasks for plugin {name}: {error}");
                        continue;
                    }
                };

                match storage.update(&name, tasks) {
                    Ok(tasks) => {
                        for task in tasks {
                            if let Err(e) = Notification::new(&identifier)
                                .title(format!("New {}", name))
                                .body(&task.title)
                                .show()
                            {
                                println!("Failed to show notification: {:?}", e);
                            }
                        }
                        println!("✅ Updated tasks for plugin {name}");
                        // TODO: trigger a refresh of the UI
                    }
                    Err(error) => {
                        println!("🛑 Error updating tasks for plugin {name}: {error}");
                    }
                }
            }
        }
    });

    {
        let tx = tx.clone();
        thread::spawn(move || loop {
            if let Err(e) = tx.send("poll") {
                println!("Error sending message: {:?}", e);
            }
            thread::sleep(Duration::from_secs(60));
        });
    }

    Ok(tx)
}

fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = app.path_resolver().app_local_data_dir().unwrap();
    fs::create_dir_all(&data_dir)?;
    let identifier = &app.config().tauri.bundle.identifier;
    app.manage(AppLocalDataDir(data_dir.clone()));
    poll(&data_dir, &identifier)?;

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(setup)
        .invoke_handler(tauri::generate_handler![get_tasks])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
