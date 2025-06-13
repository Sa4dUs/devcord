FRONTEND_SERVICE := frontend

ifeq ($(OS),Windows_NT)
  # Windows CMD / PowerShell
  SET_ENV_CMD = set NODE_ENV=$(NODE_ENV)&&
else
  # Linux / macOS
  SET_ENV_CMD = NODE_ENV=$(NODE_ENV)
endif

NODE_ENV ?= production

dev:
	$(eval NODE_ENV := development)
	$(SET_ENV_CMD) docker-compose up $(FRONTEND_SERVICE)

start:
	$(eval NODE_ENV := production)
	$(SET_ENV_CMD) docker-compose up $(FRONTEND_SERVICE)

stop:
	docker-compose down

shell:
	docker-compose run --rm $(FRONTEND_SERVICE) sh

build:
	docker-compose run --rm $(FRONTEND_SERVICE) npm run build

test:
	npm run test

test-e2e:
	npm run test:e2e
