# Roles/dirs come from .claude/owners (single source of truth).
ROLES := $(shell awk '$$1 ~ /^agent\// {print $$2}' .claude/owners)
DIRS  := $(shell awk '$$1 ~ /^agent\// {sub("/$$","",$$3); print $$3}' .claude/owners)
REPO  := $(notdir $(CURDIR))

.PHONY: check worktrees

check: ## every agent's checks that exist on this branch
	@for d in $(DIRS); do [ ! -f $$d/Makefile ] || $(MAKE) -C $$d check || exit 1; done

worktrees: ## one worktree per agent branch, next to this repo: ../$(REPO)-<role>
	@for r in $(ROLES); do [ -d ../$(REPO)-$$r ] || git worktree add ../$(REPO)-$$r agent/$$r; done
