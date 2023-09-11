import {
  IconMessage,
  IconGitPullRequest,
  IconFileUnknown,
} from "@tabler/icons-react";

interface TaskIconProps {
  kind: string;
}

export function TaskIcon({ kind }: TaskIconProps) {
  switch (kind) {
    case "github-pull-request-reviewer":
      return <IconMessage />;
    case "github-pull-request-author":
      return <IconGitPullRequest />;
    default:
      return <IconFileUnknown />;
  }
}
