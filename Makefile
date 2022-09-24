
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

1:
	cp pallets/example/src/libs/1创建ocw.rs pallets/example/src/lib.rs && cargo run --release -- --dev
2:
	cp pallets/example/src/libs/2执行时机.rs pallets/example/src/lib.rs && cargo run --release -- --dev
3:
	cp pallets/example/src/libs/3跨块执行.rs pallets/example/src/lib.rs && cargo run --release -- --dev
4:
	cp pallets/example/src/libs/4链下存储.rs pallets/example/src/lib.rs && cargo run --release -- --dev
5:
	cp pallets/example/src/libs/5原子更改.rs pallets/example/src/lib.rs && cargo run --release -- --dev

