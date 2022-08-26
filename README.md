```
polkadot \
--alice \
--validator \
--base-path /tmp/relay/alice \
--chain ./relay/rococo-custom-2-raw.json \
--port 30333 \
--ws-port 9944
```


```
polkadot \
--bob \
--validator \
--base-path /tmp/relay-bob \
--chain ./relay/rococo-custom-2-raw.json \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWN41bTz4FadGwJ1vCMukNebKnMePv5yevKTwQZzhfFFYn \
--port 30334 \
--ws-port 9945
```

```
git clone --depth 1 --branch polkadot-v0.9.27 https://github.com/substrate-developer-hub/substrate-parachain-template

# Switch into the parachain template directory
cd substrate-parachain-template

# Build the parachain template collator
cargo b -r
```