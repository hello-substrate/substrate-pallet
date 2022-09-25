
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

reset:
	cp node/src/service_bak.rs node/src/service.rs && \
	cp runtime/src/lib_bak.rs runtime/src/lib.rs
1: reset
	cp pallets/example/src/libs/1创建ocw.rs pallets/example/src/lib.rs && cargo run --release -- --dev
2: reset
	cp pallets/example/src/libs/2执行时机.rs pallets/example/src/lib.rs && cargo run --release -- --dev
3: reset
	cp pallets/example/src/libs/3跨块执行.rs pallets/example/src/lib.rs && cargo run --release -- --dev
4: reset
	cp pallets/example/src/libs/4链下存储.rs pallets/example/src/lib.rs && cargo run --release -- --dev
5: reset
	cp pallets/example/src/libs/5原子更改.rs pallets/example/src/lib.rs && cargo run --release -- --dev
6: reset
	cp pallets/example/src/libs/6http请求.rs pallets/example/src/lib.rs && cargo run --release -- --dev
7: reset
	cp pallets/example/src/libs/7签名交易.rs pallets/example/src/lib.rs && \
	cp node/src/7签名交易.rs node/src/service.rs && \
	cp runtime/src/7签名交易.rs runtime/src/lib.rs && \
 	cargo run --release -- --dev
8: reset
	cp pallets/example/src/libs/8未签名交易.rs pallets/example/src/lib.rs && \
	cp runtime/src/8未签名交易.rs runtime/src/lib.rs && \
 	cargo run --release -- --dev

