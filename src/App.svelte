<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import Tasks from "./Tasks.svelte";

  let getTasksResponse: GetTasksResponse = { statuses: [], tasks: [] };

  async function updateTasks() {
    getTasksResponse = await invoke("get_tasks");
    console.log(getTasksResponse);
  }

  onMount(() => {
    const updateTasksHandle = setInterval(updateTasks, 60000);
    updateTasks();
    return () => clearInterval(updateTasksHandle);
  });
</script>

<div>
  <Tasks tasks={getTasksResponse.tasks} />
</div>
