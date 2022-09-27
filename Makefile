
.PHONY: b

b:
	cargo build --release

br:
	cargo build --release && ./target/release/node-template --dev

clear:
	rm ~/.cargo/.package-cache

r:
	cargo run --release -- --dev

c:
	cargo check -p node-template-runtime
