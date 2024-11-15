.PHONY: setup
setup:
	bun install --no-save

.PHONY: publish
publish:
	cargo publish

.PHONY: readme-cli-update
readme-cli-update:
	bun x readme-cli-help "cargo run --quiet -- --help"
	bun x readme-cli-help --fence cli-help-version "cargo run --quiet -- version --help"
	bun x readme-cli-help --fence cli-help-ci "cargo run --quiet -- ci --help"

.PHONY: readme-cli-check
readme-cli-check:
	bun x readme-cli-help --check-only "cargo run --quiet -- --help"
	bun x readme-cli-help --fence cli-help-version --check-only "cargo run --quiet -- version --help"
	bun x readme-cli-help --fence cli-help-ci --check-only "cargo run --quiet -- ci --help"

.PHONY: lint
lint: readme-cli-check
