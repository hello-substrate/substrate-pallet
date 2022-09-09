
.PHONY: b

b:
	cargo build --release

br:
	cargo build --release && ./target/release/node-template --dev

clear:
	rm ~/.cargo/.package-cache

rr:
	cargo run --release -- --dev

check:
	cargo check -p node-template-runtime
