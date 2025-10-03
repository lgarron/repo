.PHONY: setup
setup: setup-js

.PHONY: setup-js
setup-js:
	bun install --frozen-lockfile

.PHONY: test
test: cargo-test-help lint

.PHONY: cargo-test-help
cargo-test-help:
	cargo run -- --help > /dev/null

.PHONY: publish
publish: clean publish-rust publish-js

.PHONY: publish-rust
publish-rust:
	# TODO: Remove the `--no-verify` fallback: https://github.com/rust-lang/cargo/issues/15951
	cargo run -- publish || cargo publish --no-verify # Dogfood our own `publish` command

.PHONY: publish-js
publish-js:
	bun run -- 'script/release-npm.ts'

.PHONY: readme-cli-update
readme-cli-update: ./target/debug/repo
	bun x readme-cli-help "cargo run --quiet -- --help"
	bun x readme-cli-help --fence cli-help-version "./target/debug/repo version --help"
	bun x readme-cli-help --fence cli-help-publish "./target/debug/repo publish --help"
	bun x readme-cli-help --fence cli-help-boilerplate "./target/debug/repo boilerplate --help"
	bun x readme-cli-help --fence cli-help-setup "./target/debug/repo setup --help"
	bun x readme-cli-help --fence cli-help-vcs "./target/debug/repo vcs --help"
	bun x readme-cli-help --fence cli-help-workspace "./target/debug/repo workspace --help"
	bun x readme-cli-help --fence cli-help-dependencies "./target/debug/repo dependencies --help"

.PHONY: readme-cli-check
readme-cli-check: \
	readme-cli-check-main \
	readme-cli-check-version \
	readme-cli-check-publish \
	readme-cli-check-boilerplate \
	readme-cli-check-setup \
	readme-cli-check-vcs \
	readme-cli-check-workspace \
	readme-cli-check-dependencies

readme-cli-check-main: setup-js ./target/debug/repo
	bun x readme-cli-help --check-only "./target/debug/repo --help"
readme-cli-check-version: setup-js ./target/debug/repo
	bun x readme-cli-help --fence cli-help-version --check-only "./target/debug/repo version --help"
readme-cli-check-publish: setup-js ./target/debug/repo
	bun x readme-cli-help --fence cli-help-publish --check-only "./target/debug/repo publish --help"
readme-cli-check-boilerplate: setup-js ./target/debug/repo
	bun x readme-cli-help --fence cli-help-boilerplate --check-only "./target/debug/repo boilerplate --help"
readme-cli-check-setup: setup-js ./target/debug/repo
	bun x readme-cli-help --fence cli-help-setup --check-only "./target/debug/repo setup --help"
readme-cli-check-vcs: setup-js ./target/debug/repo
	bun x readme-cli-help --fence cli-help-vcs --check-only "./target/debug/repo vcs --help"
readme-cli-check-workspace: setup-js ./target/debug/repo
	bun x readme-cli-help --fence cli-help-workspace --check-only "./target/debug/repo workspace --help"
readme-cli-check-dependencies: setup-js ./target/debug/repo
	bun x readme-cli-help --fence cli-help-dependencies --check-only "./target/debug/repo dependencies --help"

.PHONY: build-debug
build-debug: ./target/debug/repo

.PHONY: ./target/debug/repo
./target/debug/repo:
	cargo build

.PHONY: lint
lint: lint-js readme-cli-check
	cargo clippy

.PHONY: lint-js
lint-js: setup-js
	bun x @biomejs/biome check

.PHONY: format
format: setup-js readme-cli-update
	bun x @biomejs/biome check --write

.PHONY: install
install:
	cargo install --path .

.PHONY: uninstall
uninstall:
	cargo uninstall repo

.PHONY: clean
clean:
	rm -rf ./.temp/
	rm -rf \
		./src/js/@lgarron-repo/repo/package.json \
		./src/js/@lgarron-repo/repo-*

.PHONY: reset
reset: clean
	rm -rf ./node_modules/
	rm -rf ./target/

.PHONY: build-release
build-release:
	cargo build --release
