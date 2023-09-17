<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

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

  let tasks: Task[] = [];

  async function updateTasks() {
    tasks = await invoke("get_tasks");
    console.log(tasks);
  }

  onMount(() => {
    const updateTasksHandle = setInterval(updateTasks, 60000);
    updateTasks();
    return () => clearInterval(updateTasksHandle);
  });
</script>

<table class="tasks">
  <thead>
    <tr>
      <th>Title</th>
      <th>Kind</th>
      <th>Date</th>
      <th>Requestor</th>
    </tr>
  </thead>
  <tbody>
    {#each tasks as task}
      <tr>
        <td>
          <a href={task.url} target="_blank">
            {task.title}
          </a>
        </td>
        <td>
          {task.kind}
        </td>
        <td>{task.created_at}</td>
        <td>{task.requestor}</td>
      </tr>
    {/each}
  </tbody>
</table>

<style>
  .tasks {
    width: 100%;
    text-align: left;
    border-collapse: collapse;
  }

  .tasks th {
    background-color: #021920;
  }

  .tasks td {
    background-color: #06384d;
  }

  .tasks th,
  .tasks td {
    padding: 0.5rem;
    border: 1px solid #666;
  }
</style>
