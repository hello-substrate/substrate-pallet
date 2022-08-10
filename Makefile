
.PHONY: b br

b:
	cd substrate-node-template && rm ~/.cargo/.package-cache && cargo build

br:
	cd substrate-node-template && rm ~/.cargo/.package-cache && cargo build


