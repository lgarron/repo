.PHONY: setup
setup:
	bun install --no-save

.PHONY: test
test: cargo-test-help lint

.PHONY: cargo-test-help
cargo-test-help:
	cargo run -- --help > /dev/null

.PHONY: publish
publish:
	cargo run -- publish # Dogfood our own `publish` command

.PHONY: readme-cli-update
readme-cli-update:
	bun x readme-cli-help "cargo run --quiet -- --help"
	bun x readme-cli-help --fence cli-help-version "cargo run --quiet -- version --help"
	bun x readme-cli-help --fence cli-help-publish "cargo run --quiet -- publish --help"
	bun x readme-cli-help --fence cli-help-boilerplate "cargo run --quiet -- boilerplate --help"
	bun x readme-cli-help --fence cli-help-setup "cargo run --quiet -- setup --help"

.PHONY: readme-cli-check
readme-cli-check: \
	readme-cli-check-main \
	readme-cli-check-version \
	readme-cli-check-publish \
	readme-cli-check-boilerplate \
	readme-cli-check-setup

readme-cli-check-main:
	bun x readme-cli-help --check-only "cargo run --quiet -- --help"
readme-cli-check-version:
	bun x readme-cli-help --fence cli-help-version --check-only "cargo run --quiet -- version --help"
readme-cli-check-publish:
	bun x readme-cli-help --fence cli-help-publish --check-only "cargo run --quiet -- publish --help"
readme-cli-check-boilerplate:
	bun x readme-cli-help --fence cli-help-boilerplate --check-only "cargo run --quiet -- boilerplate --help"
readme-cli-check-setup:
	bun x readme-cli-help --fence cli-help-setup --check-only "cargo run --quiet -- setup --help"

.PHONY: lint
lint: readme-cli-check
	bun x biome check
	cargo clippy

.PHONY: format
format: readme-cli-update
	bun x biome check --write

.PHONY: install
install:
	cargo install --path .

.PHONY: uninstall
uninstall:
	cargo uninstall repo
