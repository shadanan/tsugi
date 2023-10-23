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

interface GetTasksResponse {
  tasks: Task[];
}
