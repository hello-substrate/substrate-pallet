# Substrate Node Template

https://docs.substrate.io/reference/how-to-guides/weights/add-benchmarks/

```
./target/release/node-template benchmark pallet \
--chain dev \
--pallet pallet-example \
--extrinsic '*' \
--steps 20 \
--repeat 10 \
--output pallets/example/src/weights.rs
```