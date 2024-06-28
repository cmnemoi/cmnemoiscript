lint:
	cargo fmt --all
	cargo clippy --all-targets --all-features --allow-dirty --fix -- -D warnings