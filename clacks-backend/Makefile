.PHONY: ci
ci: test-verbose fix clippy fmt check-repository-unchanged

.PHONY: cleanup
cleanup: fix clippy fmt

.PHONY: test
test:
	cargo test

.PHONY: test-verbose
test-verbose:
	RUST_BACKTRACE=1 cargo test -- --nocapture

.PHONY: clippy
clippy:
	cargo clippy -- -D warnings
	cargo clippy --tests -- -D warnings

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: fix
fix:
	cargo fix --allow-staged
	cargo clippy --fix --allow-staged
	cargo fix --tests --allow-staged
	cargo clippy --tests --fix --allow-staged
	cargo fmt

.PHONY: check-repository-unchanged
check-repository-unchanged:
	../check_repository_unchanged.sh