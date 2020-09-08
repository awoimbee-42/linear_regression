.PHONY: setup build npm-refresh serve doc lint help

easy-compile: compile-docker symlink ## For correctors ;) (requires docker)

compile-local: ## Compile the code using a local install of Rust
	cargo build --release

compile-docker: ## Compile the code using docker
	docker run --rm --user "$$(id -u)":"$$(id -g)" -v "$$PWD":/usr/src/myapp -w /usr/src/myapp rust:latest cargo build --release

symlink: ## Create symlinks to the executables in the root directory
	ln -s target/release/accuracy accuracy
	ln -s target/release/estimate estimate
	ln -s target/release/train train

strip: ## Run the strip command on release executables
	strip target/release/accuracy
	strip target/release/estimate
	strip target/release/train

clean: ## Remove unwanted files (light clean)
	rm -f accuracy estimate train thetas.csv training_results.svg

help: ## Show this help.
	@perl -e '$(HELP_FUN)' $(MAKEFILE_LIST)


.DEFAULT_GOAL = help
SHELL := /bin/bash

GREEN := $(shell command -v tput >/dev/null 2>&1 && tput -Txterm setaf 2 || echo "")
YELLOW := $(shell command -v tput >/dev/null 2>&1 && tput -Txterm setaf 3 || echo "")
RED := $(shell command -v tput >/dev/null 2>&1 && tput -Txterm setaf 1 || echo "")
RESET := $(shell command -v tput >/dev/null 2>&1 && tput -Txterm sgr0 || echo "")

HELP_FUN = %help; \
	while(<>) { push @{$$help{$$2 // "Other"}}, [$$1, $$3] if /^([a-zA-Z\-._]+)\s*:.*\#\#(?:@([a-zA-Z\-_]+))?\s(.*)$$/ }; \
	print "$(RESET)project: $(PURPLE)$(NAME)$(RESET)\n"; \
	print "usage: make [target]\n\n"; \
	for (sort keys %help) { \
	print "$$_:\n"; \
	for (@{$$help{$$_}}) { \
	$$sep = " " x (25 - length $$_->[0]); \
	print " ${YELLOW}$$_->[0]${RESET}$$sep${GREEN}$$_->[1]${RESET}\n"; \
	}; \
	print "\n"; }
