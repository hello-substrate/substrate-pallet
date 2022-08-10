
.PHONY: b br

b:
	cargo build

clear:
	rm ~/.cargo/.package-cache

br:
	cargo build --release

rr:
	cargo run --release -- --dev


