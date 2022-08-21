# Substrate Node Template
- 使用一组有权力(创建与完成块)的的私人验证器启动一个小型的独立区块链网络。
> 基板节点模板使用权威证明共识模型，也称为权威回合或Aura共识。Aura共识协议将块生产限制在授权帐户的轮换列表。授权帐户（授权）以循环方式创建块，通常被认为是网络中值得信赖的参与者。

## 生成随机短语于密钥
1. `subkey generate --scheme Sr25519 --password-interactive`
  1. 密码`1`
    ```
    Secret phrase:     mimic wrong assist hybrid water gorilla dwarf flight polar round sure deal
    Network ID:        substrate
    Secret seed:       0x41990fbfa6e9bcf6d58c557f8740e1e3718ad5bacd74383caae4b71b6689ffb1
    Public key (hex):  0xd8bef00f3b253b107558e223a0649c9d10572a5b68aad818882f9fae3ff71b23
    Account ID:        0xd8bef00f3b253b107558e223a0649c9d10572a5b68aad818882f9fae3ff71b23
    Public key (SS58): 5GxtvMBrZ9qRGqj9rm4HQtZ8ZTJNAUZf78hyoiAzGEEFawEQ
    SS58 Address:      5GxtvMBrZ9qRGqj9rm4HQtZ8ZTJNAUZf78hyoiAzGEEFawEQ
    ```
  2. 使用aura生成块 公钥: `5GxtvMBrZ9qRGqj9rm4HQtZ8ZTJNAUZf78hyoiAzGEEFawEQ`
  3. 使用Ed25519签名方案导出密钥
      ```
      subkey inspect --password-interactive --scheme Ed25519 "mimic wrong assist hybrid water gorilla dwarf flight polar round sure deal"

      Secret phrase:     mimic wrong assist hybrid water gorilla dwarf flight polar round sure deal
      Network ID:        substrate
      Secret seed:       0x41990fbfa6e9bcf6d58c557f8740e1e3718ad5bacd74383caae4b71b6689ffb1
      Public key (hex):  0x61460dcbb811886073e95bf626fe96b4aa2f0e3cfbdbd94f16d648a09237fb6d
      Account ID:        0x61460dcbb811886073e95bf626fe96b4aa2f0e3cfbdbd94f16d648a09237fb6d
      Public key (SS58): 5EGFK4vkkFKPzhSpzArGSnT18184kgx3yHtKQ7HJoKvvxbeE
      SS58 Address:      5EGFK4vkkFKPzhSpzArGSnT18184kgx3yHtKQ7HJoKvvxbeE
      ```
  4. 使用grandpa完成一个节点的块,公钥: `5EGFK4vkkFKPzhSpzArGSnT18184kgx3yHtKQ7HJoKvvxbeE`
2. 第二组
    ```
    Secret phrase:     loop extend alien air tube target bachelor range test winter filter glimpse
    Network ID:        substrate
    Secret seed:       0x03549cd7636a352132bcd6c37edce4571eebce02df197f365ec8b31b66a6e8a3
    Public key (hex):  0x68f4120af77504e112d9a32fd6110c782f92a6edeb993d8bfa4840fd4423572c
    Account ID:        0x68f4120af77504e112d9a32fd6110c782f92a6edeb993d8bfa4840fd4423572c
    Public key (SS58): 5ESKLkCyVU9kr652zFR5duDQkDkwLe9RGCa61sHAd3nR7ZMK
    SS58 Address:      5ESKLkCyVU9kr652zFR5duDQkDkwLe9RGCa61sHAd3nR7ZMK


    subkey inspect --password-interactive --scheme Ed25519 "loop extend alien air tube target bachelor range test winter filter glimpse"

    Secret phrase:     loop extend alien air tube target bachelor range test winter filter glimpse
    Network ID:        substrate
    Secret seed:       0x851268391285b0c3607896510dcb0163f603e54b468bfa52b9ffd7b2f9154f6e
    Public key (hex):  0x4b8c7f8acc9b731b60d842a2854eca60fd61fdcd8c2b2e879f5eaeefa32ea655
    Account ID:        0x4b8c7f8acc9b731b60d842a2854eca60fd61fdcd8c2b2e879f5eaeefa32ea655
    Public key (SS58): 5DmmBhGPbxU8gnsGYbkVNkR15xFhTfjfdoB2t7zpGWxZco2c
    SS58 Address:      5DmmBhGPbxU8gnsGYbkVNkR15xFhTfjfdoB2t7zpGWxZco2c
    ```

## 创建链规范
```
./target/release/node-template build-spec --disable-default-bootnode --chain local > customSpec.json
```
修改 `name` 字段
1. 修改aura字段指定有权创建块的节点。添加Sr25519 SS58地址密钥
```

```
2. 修改grandpa字段以指定有权完成块的节点。添加Ed25519 SS58地址密钥
第一个值是地址。第二个值用于支持加权投票。在本例中，每个验证器的权重为1票
```
```
3. 请务必为每个验证器使用唯一的密钥。如果两个验证器具有相同的密钥，它们会产生冲突的块。

## 转换原始格式
`./target/release/node-template build-spec --chain=customSpec.json --raw --disable-default-bootnode > customSpecRaw.json`

## 启动
1. aura密钥插入 keystore
```
./target/release/node-template key insert --base-path /tmp/node01 \
  --chain customSpecRaw.json \
  --scheme Sr25519 \
  --suri "mimic wrong assist hybrid water gorilla dwarf flight polar round sure deal" \
  --password-interactive \
  --key-type aura
```
2. grandpa密钥插入 keystore
```
./target/release/node-template key insert \
  --base-path /tmp/node01 \
  --chain customSpecRaw.json \
  --scheme Ed25519 \
  --suri "mimic wrong assist hybrid water gorilla dwarf flight polar round sure deal" \
  --password-interactive \
  --key-type gran
```
3. 查看 `ls /tmp/node01/chains/local_testnet/keystore`
生成类似
```
617572611441ddcb22724420b87ee295c6d47c5adff0ce598c87d3c749b776ba9a647f04
6772616e1441ddcb22724420b87ee295c6d47c5adff0ce598c87d3c749b776ba9a647f04
```
4. 启动
```
./target/release/node-template \
  --base-path /tmp/node01 \
  --chain ./customSpecRaw.json \
  --port 30333 \
  --ws-port 9945 \
  --rpc-port 9933 \
  --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
  --validator \
  --rpc-methods Unsafe \
  --name MyNode01 \
  --password-interactive
```

## 其他节点加入
1. aura密钥插入 keystore(同上)
2. grandpa密钥插入 keystore(同上)
3. 启动
```
./target/release/node-template \
  --base-path /tmp/node02 \
  --chain ./customSpecRaw.json \
  --port 30334 \
  --ws-port 9946 \
  --rpc-port 9934 \
  --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
  --validator \
  --rpc-methods Unsafe \
  --name MyNode02 \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWLmrYDLoNTyTYtRdDyZLWDe1paxzxTw5RgjmHLfzW96SX \
  --password-interactive
```

