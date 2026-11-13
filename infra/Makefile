ROOT    := $(abspath ..)
COMPOSE := docker compose -p agent-team -f compose.yaml
TIMEOUT ?= 300
ACTIONLINT := $(if $(shell command -v actionlint),actionlint,docker run --rm -v $(ROOT):/repo -w /repo rhysd/actionlint:1.7.7)

.PHONY: lint test build plan up down deploy check

lint:
	$(COMPOSE) config -q
	cd $(ROOT) && $(ACTIONLINT) -color=false .github/workflows/*.yml

test:
	true

build:
	@if [ ! -d ../backend ] || [ ! -d ../frontend ]; then echo "infra build: skipped, ../backend or ../frontend not on this branch yet"; exit 0; fi; \
	$(COMPOSE) build

plan:
	@echo "project: agent-team"
	@$(COMPOSE) config --services | sed 's/^/service: /'
	@$(COMPOSE) config --images | sed 's/^/image:   /'

up:
	$(COMPOSE) up -d --build
	@end=$$(( $$(date +%s) + $(TIMEOUT) )); \
	until curl -fsS http://localhost:8080/health && curl -fsS http://localhost:3000/health; do \
		if [ $$(date +%s) -ge $$end ]; then echo "infra up: /health not ready after $(TIMEOUT)s"; $(COMPOSE) logs; exit 1; fi; \
		sleep 2; \
	done; echo; echo "infra up: backend and frontend healthy"

down:
	$(COMPOSE) down --remove-orphans

deploy:
	@echo "infra deploy (stub): would deploy images:"; $(COMPOSE) config --images | sed 's/^/  /'

check: lint test
