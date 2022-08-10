
.PHONY: b br

b:
	rm ~/.cargo/.package-cache && cargo build

br:
	rm ~/.cargo/.package-cache && cargo build --release

rr:
	rm ~/.cargo/.package-cache && cargo run --release -- --dev


