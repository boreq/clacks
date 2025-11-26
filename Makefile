USER_ID := $(shell id -u)
GROUP_ID := $(shell id -g)

CONTAINER_ENGINE?=podman

ifeq ($(CONTAINER_ENGINE),podman)
    CONTAINER_FLAGS = --userns=keep-id
else ifeq ($(CONTAINER_ENGINE),docker)
    CONTAINER_FLAGS = -u $(USER_ID):$(GROUP_ID)
else
    $(error Unknown CONTAINER_ENGINE "$(CONTAINER_ENGINE)")
endif

FRONTEND_BUILDER_CONTAINER_ENGINE=$(CONTAINER_ENGINE)
FRONTEND_DIRECTORY=./clacks-frontend
DOCKER_FRONTEND_BUILDER_TAG=clacks/frontend-builder

BACKEND_DIRECTORY=./clacks-backend
MAIN_BINARY_NAME=clacks
CROSS_CONTAINER_ENGINE=$(CONTAINER_ENGINE)
CROSS_RPI_RUSTFLAGS=-C target-cpu=arm1176jz-s
CROSS_RPI_TARGET=arm-unknown-linux-gnueabihf
CROSS_RPI_FEATURES=raspberry_pi serve_frontend
CROSS_RPI_COMMAND_BUILD=RUSTFLAGS="$(CROSS_RPI_RUSTFLAGS)" CROSS_CONTAINER_ENGINE=$(CROSS_CONTAINER_ENGINE) cross build --target="$(CROSS_RPI_TARGET)" --features="$(CROSS_RPI_FEATURES)" --release

.PHONY: build
build: build-frontend build-backend

.PHONY: build-frontend
build-frontend:
	$(FRONTEND_BUILDER_CONTAINER_ENGINE) build $(FRONTEND_DIRECTORY) -t $(DOCKER_FRONTEND_BUILDER_TAG)
	$(FRONTEND_BUILDER_CONTAINER_ENGINE) run --rm -v $(FRONTEND_DIRECTORY):/clacks-frontend $(CONTAINER_FLAGS) $(DOCKER_FRONTEND_BUILDER_TAG)

.PHONY: build-backend
build-backend:
	cargo install cross --git https://github.com/cross-rs/cross
	cd $(BACKEND_DIRECTORY) && $(CROSS_RPI_COMMAND_BUILD) --bin=$(MAIN_BINARY_NAME)
	@echo 'Binary location:'
	@echo '$(BACKEND_DIRECTORY)/target/$(CROSS_RPI_TARGET)/release/clacks'
