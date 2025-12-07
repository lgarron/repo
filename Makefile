.PHONY: setup
setup: setup-js

.PHONY: setup-js
setup-js:
	bun install --frozen-lockfile

.PHONY: test
test: cargo-test cargo-test-help lint

.PHONY: cargo-test
cargo-test:
	cargo test

.PHONY: cargo-test-help
cargo-test-help:
	cargo run -- --help > /dev/null

.PHONY: publish
publish: test clean publish-rust publish-js

.PHONY: publish-rust
publish-rust:
	# TODO: Remove `--no-verify`: https://github.com/rust-lang/cargo/issues/15951
	cargo publish --no-verify # Dogfood our own `publish` command

.PHONY: publish-js
publish-js:
	bun run -- 'script/release-npm.ts'

.PHONY: readme-cli-update
readme-cli-update: ./target/debug/repo
	bun x readme-cli-help update

.PHONY: readme-cli-check
readme-cli-check: setup-js ./target/debug/repo
	bun x readme-cli-help check

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
	bun x tsc --noEmit --project .

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
		./src/js/@lgarron-bin/repo/package.json \
		./src/js/@lgarron-bin/repo-*

.PHONY: reset
reset: clean
	rm -rf ./node_modules/
	rm -rf ./target/

.PHONY: build-release
build-release:
	cargo build --release

.PHONY: test-published-package
test-published-package:
	# `bun` is not available on all platforms, so we use `npx`.
	# Further, GitHub Actions has a shim for `npm` that requires additional workarounds inside a script.
	# So we just perform simple checks here.
	npm install --save @lgarron-bin/repo
	npx @lgarron-bin/repo --help
	npx @lgarron-bin/repo workspace root
	ls -al ./node_modules/@lgarron-bin/
