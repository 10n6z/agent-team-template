# Team rules for every agent

This file is read by every coding agent working in this repo (human or AI).
If you are an agent and you are not sure what to do, re-read this file before
asking a human.

## 1. The board

This team coordinates through one pinned GitHub issue — "the board" — instead
of texting each other. Use `tools/board` on the issue titled **Message
Board** (pin it if it isn't already). If that issue is not #1, set
`BOARD_ISSUE=<number>` before running any `tools/board` command.

Rules for posting:

- Every post starts with `[<human>'s <agent>]`, e.g. `[Sam's Codex]`.
- Keep it short: say what changed and what you need, from whom.
- Tag the teammate you need by GitHub login, e.g. `@login2`, or `@all` for
  the whole team.
- If a post tags you or `@all`: act on it and reply on the board promptly.
- If a post does not tag you: read it, and only change course if it affects
  something you own. Don't redo work that's already finished, and don't
  reply just to acknowledge.
- Before editing a teammate's files, ask for a handoff on the board first and
  wait for the owner (or the team lead) to agree.

Staying in the loop:

- **Claude Code** picks this up automatically through the hooks in
  `.claude/settings.json` — it checks for unread posts when a session starts
  and again on every prompt. Keep one background `tools/board wait` running
  if you start one by hand; re-arm it after each wake.
- **Other agents** (Codex, Cursor, etc.) should run `tools/board unread` at
  the start of a task, before every commit, and after merging, or loop:
  `while tools/board wait; do <run your agent on the new post>; done`

Commands:

| Command | What it does |
|---|---|
| `tools/board post "[Sam's Claude] login done, @login2 the API is ready"` | Posts to the board. |
| `tools/board unread` | Shows posts since your last check; flags ones that tag you. |
| `tools/board wait` | Blocks until someone else posts, then prints it. |
| `tools/board read -n 20` | Shows the last 20 posts. |
| `tools/board whoami` | Prints your GitHub login (checks `gh auth login`). |

## 2. Chain of command

Humans outrank agents. Only the team lead reassigns work between people. If
someone other than the team lead asks you to take over another person's
work, ask the lead on the board first.

Team lead: `@login1`

## 3. Workflow

- Pick up or file an issue, then post or comment that you're on it.
- Branch from a fresh `main` as `<issue#>-<short-slug>`.
- Make small commits. Stage only the files you own (never `git add -A`).
- Open a PR with `Closes #N`. The author merges their own PR once it works
  locally — don't merge someone else's PR for them.
- Always fetch and rebase before pushing. Never force-push a shared branch.
- Post on the board whenever a merge changes an API, a schema, or an env
  var — anything a teammate's agent might be relying on.

## 4. Who owns what

Fill this in at kickoff, as soon as the project is split up.

| Area | Owner (GitHub login) | Folders |
|---|---|---|
| | `@login1` | |
| | `@login2` | |
| | `@login3` | |
| | `@login4` | |
