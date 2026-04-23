# Variables
TARGET = riscv64gc-unknown-linux-gnu
DEPLOY_DIR = rektstarsnet-release
REMOTE_HOST = orangepi@192.168.1.10
REMOTE_PATH = ~

# Compilation Flags
export RUSTFLAGS = -C target-feature=+zba,+zbb,+zbs

.PHONY: all build package deploy clean

# Default target
all: build package deploy

# 1. Build the Rust project for RISC-V
build:
	cargo build --release --target $(TARGET)

# 2. Prepare the deployment folder
package: build
	@echo "Packaging files into $(DEPLOY_DIR)..."
	mkdir -p $(DEPLOY_DIR)
	cp -r templates/ static/ ./data.csv target/riscv64gc-unknown-linux-gnu/release/rektstarsnet $(DEPLOY_DIR)/
	@echo "Packaging complete."

# 3. Transfer to Orange Pi
# Note: rsync is preferred for directories to preserve structure efficiently
deploy: package
	@echo "Deploying to $(REMOTE_HOST)..."
	ssh $(REMOTE_HOST) "echo orangepi | sudo -S systemctl stop rektstarsnet"
	scp -r $(DEPLOY_DIR) $(REMOTE_HOST):$(REMOTE_PATH)
	ssh $(REMOTE_HOST) "echo orangepi | sudo -S systemctl start rektstarsnet"

# Clean up local deployment artifacts
clean:
	rm -rf $(DEPLOY_DIR)
	cargo clean
