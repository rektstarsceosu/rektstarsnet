# variables
TARGET = riscv64gc-unknown-linux-gnu
DEPLOY_DIR = rektstarsnet-release
REMOTE_HOST ?= orenji@192.168.1.5
REMOTE_PATH = ~

export RUSTFLAGS = -C target-feature=+zba,+zbb,+zbs

.PHONY: all build package deploy clean

all: build package deploy

build:
	cargo build --release --target $(TARGET)

package: build
	@echo "Packaging files into $(DEPLOY_DIR)..."
	mkdir -p $(DEPLOY_DIR)
	cp -r templates/ static/ ./data.csv target/riscv64gc-unknown-linux-gnu/release/rektstarsnet $(DEPLOY_DIR)/
	echo "" > $(DEPLOY_DIR)/agents.log
	@echo "Packaging complete."

deploy: package
	@echo "Deploying to $(REMOTE_HOST)..."
	ssh $(REMOTE_HOST) "sudo systemctl stop rektstarsnet"
	scp -r $(DEPLOY_DIR) $(REMOTE_HOST):$(REMOTE_PATH)
	ssh $(REMOTE_HOST) "sudo systemctl start rektstarsnet"


clean:
	rm -rf $(DEPLOY_DIR)
	cargo clean
