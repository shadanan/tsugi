use crate::{error::TsugiError, task::Task};
use rusqlite::Connection;
use tauri::AppHandle;

pub struct Sqlite {
    conn: Connection,
}

impl Sqlite {
    pub fn new(app: &AppHandle) -> Result<Self, TsugiError> {
        let sqlite = Self {
            conn: Connection::open(
                app.path_resolver()
                    .app_local_data_dir()
                    .unwrap()
                    .join("tsugi.db"),
            )?,
        };
        sqlite.conn.execute(
            "
            CREATE TABLE IF NOT EXISTS tasks (
                id TEXT NOT NULL,
                kind TEXT NOT NULL,
                url TEXT NOT NULL,
                title TEXT NOT NULL,
                description TEXT NOT NULL,
                state TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                closed_at TEXT,
                requestor TEXT,
                PRIMARY KEY (id, kind)
            )",
            [],
        )?;
        Ok(sqlite)
    }

    pub fn update(&self, kind: &str, tasks: Vec<Task>) -> Result<(), TsugiError> {
        let query = "
            INSERT INTO tasks (
                id, 
                kind, 
                url, 
                title, 
                description, 
                state, 
                created_at, 
                updated_at, 
                closed_at, 
                requestor
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
        for task in tasks {
            let mut stmt = self.conn.prepare(query)?;
            stmt.execute((
                task.id.as_str(),
                kind,
                task.url.as_str(),
                task.title.as_str(),
                task.description.as_str(),
                task.state.as_str(),
                task.created_at.as_str(),
                task.updated_at.as_str(),
                task.closed_at.as_str(),
                task.requestor.as_str(),
            ))?;
        }
        Ok(())
    }
}
