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

interface PluginStatus {
  name: string;
  status: string;
  message: string;
}

interface GetTasksResponse {
  statuses: PluginStatus[];
  tasks: Task[];
}
