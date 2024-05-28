build:
	cargo build

lint:
	cargo fmt -- --check && cargo clippy --all-targets --all-features -- -D warnings

test:
	test.coverage.lcov

test.coverage.lcov:
	cargo tarpaulin --out lcov --output-dir coverage

test.doc:
	cargo test --doc

test.unit:
	cargo test --bins --lib
