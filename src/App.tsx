import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { TaskIcon } from "./TaskIcon";

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
      const tasks: Task[] = await invoke("get_tasks");
      setTasks(tasks);
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
        {tasks.map((task: any) => (
          <tr key={task.id}>
            <td>
              <a href={task.url} target="_blank">
                {task.title}
              </a>
            </td>
            <td>
              <TaskIcon kind={task.kind} />
            </td>
            <td>{task.created_at}</td>
            <td>{task.requestor}</td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}

export default App;
