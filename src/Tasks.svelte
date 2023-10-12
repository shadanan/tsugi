<script lang="ts">
  export let tasks: Task[] = [];
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
      <tr class={task.state}>
        <td class={task.state}>
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

  .tasks td.closed {
    text-decoration: line-through;
  }

  .tasks tr.closed td {
    filter: brightness(0.3);
  }
</style>
