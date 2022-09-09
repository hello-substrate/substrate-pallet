
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

node1:
	./target/release/node-template --base-path /tmp/node01 \
        --chain custom \
        --port 30333 \
        --ws-port 9944 \
        --rpc-port 9933 \
        --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
        --node-key=2771a6fb1ee93a39773f4f26715966cad41db0d843c8e60f48b9e2cadf6b5906 \
        --rpc-methods Unsafe \
        --name MyNode01 --validator \
        --password-interactive

node2:
	./target/release/node-template --base-path /tmp/node02 \
        --chain custom \
        --port 30334 \
        --ws-port 9945 \
        --rpc-port 9934 \
        --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
        --node-key=84decb33517c08018d8c7a18b597a5d8a2ce4cfe57d2ce1e97774da1368bb6a4 \
        --rpc-methods Unsafe \
        --name MyNode02 --validator \
        --password-interactive

node3:
	./target/release/node-template --base-path /tmp/node03 \
        --chain custom \
        --name MyNode03 --validator \
        --node-key=e4e6a2fe748c955b7306edfbe00c9868e7eab2b919427188dfe4bedb2de1d57c \
        --port 30335 \
        --ws-port 9946 \
        --offchain-worker always

node4:
	./target/release/node-template --base-path /tmp/node04 \
        --chain custom \
        --name MyNode04 --validator \
        --node-key=05e36c497df61895ca7a3a548dcaf475d22edc070416b910dba13b6b15d07888 \
        --port 30336 \
        --ws-port 9947 \
        --offchain-worker always