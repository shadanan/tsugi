use crate::{error::TsugiError, plugin, task};
use rusqlite::Connection;
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

const SCHEMA_SQL: &str = "
CREATE TABLE IF NOT EXISTS tasks (
  kind TEXT NOT NULL,
  id TEXT NOT NULL,
  url TEXT NOT NULL,
  title TEXT NOT NULL,
  description TEXT NOT NULL,
  state TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  closed_at TEXT,
  requestor TEXT,
  PRIMARY KEY (kind, id)
)";

const LIST_SQL: &str = "
SELECT * 
FROM tasks 
ORDER BY 
  state DESC,
  created_at DESC";

const OPEN_SQL: &str = "
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

const UPDATE_SQL: &str = "
UPDATE tasks 
  SET url = ?, 
      title = ?, 
      description = ?, 
      state = ?, 
      updated_at = ?, 
      closed_at = ?, 
      requestor = ? 
WHERE kind = ? 
  AND id = ?";

const CLOSE_SQL: &str = "
UPDATE tasks 
  SET state = 'closed' 
WHERE kind = ? 
  AND id = ?";

pub struct Storage {
    conn: Connection,
}

impl Storage {
    pub fn new(data_dir: &PathBuf) -> Result<Self, TsugiError> {
        let conn = Connection::open(data_dir.join("tsugi.sqlite"))?;

        Ok(Self { conn })
    }

    pub fn ensure_schema(self) -> Result<Self, TsugiError> {
        self.conn.execute(SCHEMA_SQL, [])?;
        Ok(self)
    }

    pub fn all_tasks(&self) -> Result<Vec<task::Task>, TsugiError> {
        let mut stmt = self.conn.prepare(LIST_SQL)?;
        let rows = stmt.query_map([], |row| {
            Ok(task::Task {
                kind: row.get("kind")?,
                id: row.get("id")?,
                url: row.get("url")?,
                title: row.get("title")?,
                description: row.get("description")?,
                state: row.get("state")?,
                created_at: row.get("created_at")?,
                updated_at: row.get("updated_at")?,
                closed_at: row.get("closed_at")?,
                requestor: row.get("requestor")?,
            })
        })?;
        let mut tasks = Vec::new();
        for row in rows {
            tasks.push(row?);
        }
        Ok(tasks)
    }

    pub fn tasks(&self, kind: &str) -> Result<Vec<task::Task>, TsugiError> {
        let mut stmt = self.conn.prepare("SELECT * FROM tasks WHERE kind = ?")?;
        let rows = stmt.query_map([kind], |row| {
            Ok(task::Task {
                kind: row.get("kind")?,
                id: row.get("id")?,
                url: row.get("url")?,
                title: row.get("title")?,
                description: row.get("description")?,
                state: row.get("state")?,
                created_at: row.get("created_at")?,
                updated_at: row.get("updated_at")?,
                closed_at: row.get("closed_at")?,
                requestor: row.get("requestor")?,
            })
        })?;
        let mut tasks = Vec::new();
        for row in rows {
            tasks.push(row?);
        }
        Ok(tasks)
    }

    pub fn update(
        &self,
        kind: &str,
        tasks: Vec<plugin::Task>,
    ) -> Result<HashSet<task::Task>, TsugiError> {
        let previous_tasks = self
            .tasks(kind)?
            .into_iter()
            .map(|t| (t.id.clone(), t))
            .collect::<HashMap<_, _>>();

        let current_tasks = tasks
            .into_iter()
            .map(|t| (t.id.clone(), t.to(kind)))
            .collect::<HashMap<_, _>>();

        let all_task_ids = previous_tasks
            .keys()
            .chain(current_tasks.keys())
            .collect::<HashSet<_>>();

        let mut open_stmt = self.conn.prepare(OPEN_SQL)?;
        let mut update_stmt = self.conn.prepare(UPDATE_SQL)?;
        let mut close_stmt = self.conn.prepare(CLOSE_SQL)?;

        let mut new_tasks: HashSet<task::Task> = HashSet::new();

        for task_id in all_task_ids {
            let previous_task = previous_tasks.get(task_id);
            let current_task = current_tasks.get(task_id);
            match (previous_task, current_task) {
                (Some(previous_task), Some(current_task)) => {
                    if previous_task != current_task {
                        update_stmt.execute((
                            &current_task.url,
                            &current_task.title,
                            &current_task.description,
                            &current_task.state,
                            &current_task.updated_at,
                            &current_task.closed_at,
                            &current_task.requestor,
                            kind,
                            task_id,
                        ))?;
                    }
                }
                (Some(previous_task), None) => {
                    if previous_task.state != "closed" {
                        close_stmt.execute([kind, task_id])?;
                    }
                }
                (None, Some(current_task)) => {
                    open_stmt.execute((
                        &current_task.id,
                        &current_task.kind,
                        &current_task.url,
                        &current_task.title,
                        &current_task.description,
                        &current_task.state,
                        &current_task.created_at,
                        &current_task.updated_at,
                        &current_task.closed_at,
                        &current_task.requestor,
                    ))?;

                    new_tasks.insert(current_task.clone());
                }
                (None, None) => {}
            }
        }

        Ok(new_tasks)
    }
}
