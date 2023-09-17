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
    case "GitHub PR Review":
      return <IconMessage />;
    case "GitHub PR":
      return <IconGitPullRequest />;
    default:
      return <IconFileUnknown />;
  }
}
