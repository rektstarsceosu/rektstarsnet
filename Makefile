# variables
TARGET = riscv64gc-unknown-linux-gnu
DEPLOY_DIR = ./*
REMOTE_HOST ?= orenji@192.168.1.5
REMOTE_PATH = /var/www/html/rektstars/
export RUSTFLAGS = -C target-feature=+zba,+zbb,+zbs

.PHONY: all build

all: deploy
deploy:
	@echo "Deploying to $(REMOTE_HOST)..."
	rsync -avr $(DEPLOY_DIR) $(REMOTE_HOST):$(REMOTE_PATH)


