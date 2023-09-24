<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import Tasks from "./Tasks.svelte";

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

<div>
  <Tasks {tasks} />
</div>
