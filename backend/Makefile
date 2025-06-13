BACKEND_FOLDER := backend

ifndef SERVICE
  $(error Required SERVICE=<service-name>)
endif

BUILD_MODE ?= debug

ifeq ($(OS),Windows_NT)
  SET_ENV_CMD = set RUST_BACKTRACE=1&&
else
  SET_ENV_CMD = RUST_BACKTRACE=1
endif

DOCKER_BUILD_MODE = $(if $(filter $(BUILD_MODE),release),--release,)

DOCKER_COMPOSE := docker-compose

run:
	$(SET_ENV_CMD) $(DOCKER_COMPOSE) up $(SERVICE)

run-release:
	$(MAKE) run SERVICE=$(SERVICE) BUILD_MODE=release

run-debug:
	$(MAKE) run SERVICE=$(SERVICE) BUILD_MODE=debug

stop:
	$(DOCKER_COMPOSE) down

shell:
	$(DOCKER_COMPOSE) run --rm $(SERVICE) sh

test:
	cargo test

clean:
	cargo clean
