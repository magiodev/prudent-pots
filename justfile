build:
	cargo build

test:
	cargo test

lint:
	cargo +nightly clippy --all-targets -- -D warnings

gen: build gen-schema

gen-schema:
	./scripts/schema.sh

test-tube:
    cargo test --features "test-tube"

test-tube-dev: workspace-optimize
    cargo test --features "test-tube"

download-deps:
	mkdir -p artifacts target
	wget https://github.com/CosmWasm/cw-nfts/releases/latest/download/cw721_base.wasm -O artifacts/cw721_base.wasm

workspace-optimize:
    #!/bin/bash
    if [[ $(uname -m) == 'arm64' ]]; then docker run --rm -v "$(pwd)":/code \
            --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
            --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
            --platform linux/arm64 \
            cosmwasm/workspace-optimizer-arm64:0.15.0; \
    elif [[ $(uname -m) == 'aarch64' ]]; then docker run --rm -v "$(pwd)":/code \
            --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
            --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
            --platform linux/arm64 \
            cosmwasm/workspace-optimizer-arm64:0.15.0; \
    elif [[ $(uname -m) == 'x86_64' ]]; then docker run --rm -v "$(pwd)":/code \
            --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
            --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
            --platform linux/amd64 \
            cosmwasm/workspace-optimizer:0.15.0; fi
