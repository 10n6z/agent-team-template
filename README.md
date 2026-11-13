# agent-team-template

One pinned GitHub issue as a shared board for the team's coding agents.
Every agent posts there when it finishes something or needs something, and
tags the teammate it needs by GitHub username. Each agent waits for its own
tag, so work can hand off between people's agents without anyone sending a
text. Works with any agent that can run a shell command — the board script
only needs `bash` and the GitHub CLI (`gh`).

Based on the setup described at
[The Hackathon Agent Board](https://marvelous-relative-5de.notion.site/hack).

## Setup (about 5 minutes)

1. Open a new issue from the **Message board** issue template
   (`.github/ISSUE_TEMPLATE/message-board.md`) and pin it.
2. If it isn't issue #1, every teammate runs `export BOARD_ISSUE=<number>`
   (put it in your shell profile so it sticks).
3. Fill in your team's GitHub logins and the owner table in `AGENTS.md`.
4. Every teammate runs `gh auth login` once, then `tools/board whoami` to
   check it's wired up.

Claude Code picks the board up on its own through the hooks in
`.claude/settings.json` — it checks for unread posts when a session starts
and again on every prompt. Other agents should read `AGENTS.md` and run
`tools/board unread` before each task (see AGENTS.md for the recommended
loop).

## Commands

```
tools/board post "[Sam's Claude] login page is done, @login2 the API route is ready"
tools/board unread
tools/board wait [--interval SECONDS] [--timeout SECONDS]
tools/board read [-n N]
tools/board whoami
```

## Requirements

- `bash`, `git`
- GitHub CLI (`gh`), logged in (`gh auth login`)
- Write access to this repo

## FAQ

**Will this hit GitHub's rate limit?** No. `tools/board wait` checks once
every 30 seconds by default, and an idle check is one API call — about 120
calls/hour per agent, against GitHub's 5,000/hour per logged-in account. If
several agents share one GitHub account, slow it down with
`tools/board wait --interval 60`.

**Can the repo be private?** Yes. Every teammate just needs repo access and
their own `gh auth login`.

**What if two agents need the same file?** The agent that needs it asks on
the board and waits for the owner, or the team lead, to hand it off. Only
the team lead moves work between people (see `AGENTS.md`).

**Does it only work with Claude Code?** No. The script only needs `bash` and
`gh`, so any agent that can run a shell command can use it. Claude Code gets
the automatic hooks on top of that.
