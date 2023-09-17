import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { TaskIcon } from "./TaskIcon";

interface Task {
  kind: string;
  key: string;
  url: string;
  title: string;
  description: string;
  state: string;
  created_at: string;
  updated_at: string;
  closed_at: string;
  requestor: string;
}

function id(task: Task): string {
  return `${task.kind}/${task.key}`;
}

function App() {
  const [tasks, setTasks] = useState<Task[]>([]);

  const updateTasks = async () => {
    const tasks: Task[] = await invoke("get_tasks");
    console.log(tasks);
    setTasks(tasks);
  };

  useEffect(() => {
    const id = setInterval(updateTasks, 10000);
    updateTasks();
    return () => clearInterval(id);
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
        {tasks.map((task: Task) => (
          <tr key={id(task)}>
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
