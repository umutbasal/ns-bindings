fmt:
	cargo clippy --fix --allow-dirty --allow-staged && cargo fmt --all -- --check
