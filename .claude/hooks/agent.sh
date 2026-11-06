#!/usr/bin/env bash
# Claude Code hooks for the agent team. Role = current branch, looked up in .claude/owners.
#   start → SessionStart: inject role, role CLAUDE.md, current task (orchestrator: team status)
#   guard → PreToolUse(Edit|Write|MultiEdit|NotebookEdit): block edits outside owned paths
#   stop  → Stop: block finishing while `make -C <role dir> check` fails
set -uo pipefail
cd "${CLAUDE_PROJECT_DIR:-$(git rev-parse --show-toplevel)}" || exit 0
root=$PWD
branch=$(git branch --show-current)
line=$(awk -v b="$branch" '$1==b' .claude/owners)
[ -n "$line" ] || exit 0   # branch not in owners: no role, no restrictions
read -r _ role paths <<<"$line"
dir=${paths%% *}; dir=${dir%/}

start() {
  echo "# You are the $role agent (branch $branch)"
  echo "Editable paths (hook-enforced): $paths"
  [ -f "$dir/CLAUDE.md" ] && cat "$dir/CLAUDE.md"
  if [ "$role" = orchestrator ]; then
    echo; echo "## Team status"
    awk '$1 ~ /^agent\// {print $1, $3}' .claude/owners | while read -r b d; do
      ahead=$(git rev-list --count "main..$b" 2>/dev/null || echo "no branch")
      echo "### $b — $ahead commits ahead of main"
      git show "$b:${d%/}/STATUS.md" 2>/dev/null || echo "(no STATUS.md yet)"
    done
  else
    echo; echo "## Current task (read live from main)"
    git show "main:orchestrator/tasks/$role.md" 2>/dev/null || echo "No task assigned yet. Wait for the orchestrator."
  fi
}

guard() {
  file=$(jq -r '.tool_input.file_path // .tool_input.notebook_path // empty')
  [ -n "$file" ] || exit 0
  case "$file" in /*) ;; *) file="$root/$file" ;; esac
  rel=${file#"$root"/}
  case "$rel" in *..*) ;; *) for p in $paths; do case "$rel" in "$p"*) exit 0 ;; esac; done ;; esac
  echo "BLOCKED: $role agent may only edit: $paths. '$rel' is not yours — add a Request to $dir/STATUS.md instead." >&2
  exit 2
}

stop() {
  [ "$(jq -r '.stop_hook_active // false')" = true ] && exit 0   # avoid stop-hook loops
  [ -f "$dir/Makefile" ] || exit 0
  out=$(make -C "$dir" check 2>&1) && exit 0
  printf 'make -C %s check failed. Fix it before finishing:\n%s\n' "$dir" "$(tail -n 40 <<<"$out")" >&2
  exit 2
}

case "${1:-}" in start) start ;; guard) guard ;; stop) stop ;; esac
