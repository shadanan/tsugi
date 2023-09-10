import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

import { IconGitPullRequest, IconFileUnknown } from "@tabler/icons-react";
import { listen } from "@tauri-apps/api/event";

interface Task {
  id: string;
  kind: string;
  url: string;
  title: string;
  description: string;
  state: string;
  created_at: string;
  updated_at: string;
  closed_at: string;
  requestor: string;
}

function App() {
  const [tasks, setTasks] = useState<Task[]>([]);

  useEffect(() => {
    (async () => {
      const tasks: Task[] = JSON.parse(await invoke("get_tasks"));
      setTasks(tasks);
    })();
  }, []);

  useEffect(() => {
    (async () => {
      const unlisten = await listen("tasks", (payload: any) => {
        console.log(payload);
      });
    })();
  }, []);

  return (
    <table>
      <thead>
        <tr>
          <th>Title</th>
          <th>Kind</th>
          <th>Date</th>
          <th>Requestor</th>
        </tr>
      </thead>
      <tbody>
        {
          // iterate over tasks and render them
          tasks.map((task: any) => (
            <tr key={task.id}>
              <td>
                <a href={task.url}>{task.title}</a>
              </td>
              <td>
                {task.kind === "github-pull-request-review" ? (
                  <IconGitPullRequest />
                ) : (
                  <IconFileUnknown />
                )}
              </td>
              <td>{task.created_at}</td>
              <td>{task.requestor}</td>
            </tr>
          ))
        }
      </tbody>
    </table>
  );
}

export default App;
