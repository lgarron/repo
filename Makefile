.PHONY: setup
setup:
	bun install --frozen-lockfile

.PHONY: test
test: cargo-test-help lint

.PHONY: cargo-test-help
cargo-test-help:
	cargo run -- --help > /dev/null

.PHONY: publish
publish: clean
	cargo run -- publish # Dogfood our own `publish` command

.PHONY: readme-cli-update
readme-cli-update:
	bun x readme-cli-help "cargo run --quiet -- --help"
	bun x readme-cli-help --fence cli-help-version "cargo run --quiet -- version --help"
	bun x readme-cli-help --fence cli-help-publish "cargo run --quiet -- publish --help"
	bun x readme-cli-help --fence cli-help-boilerplate "cargo run --quiet -- boilerplate --help"
	bun x readme-cli-help --fence cli-help-setup "cargo run --quiet -- setup --help"
	bun x readme-cli-help --fence cli-help-vcs "cargo run --quiet -- vcs --help"
	bun x readme-cli-help --fence cli-help-workspace "cargo run --quiet -- workspace --help"

.PHONY: readme-cli-check
readme-cli-check: \
	readme-cli-check-main \
	readme-cli-check-version \
	readme-cli-check-publish \
	readme-cli-check-boilerplate \
	readme-cli-check-setup \
	readme-cli-check-vcs

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
readme-cli-check-vcs:
	bun x readme-cli-help --fence cli-help-vcs --check-only "cargo run --quiet -- vcs --help"
readme-cli-check-workspace:
	bun x readme-cli-help --fence cli-help-workspace --check-only "cargo run --quiet -- workspace --help"

.PHONY: lint
lint: readme-cli-check
	bun x @biomejs/biome check
	cargo clippy

.PHONY: format
format: readme-cli-update
	bun x @biomejs/biome check --write

.PHONY: install
install:
	cargo install --path .

.PHONY: uninstall
uninstall:
	cargo uninstall repo

.PHONY: clean
clean:
	rm -rf ./node_modules/

.PHONY: reset
reset: clean
	rm -rf ./target/

