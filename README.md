# 添加可信节点
[docs](https://docs.substrate.io/tutorials/get-started/trusted-network/)
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
```
./target/release/node-template build-spec --chain=customSpec.json --raw --disable-default-bootnode > customSpecRaw.json
```

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
3. 查看
生成类似
```
ls /tmp/node01/chains/local_testnet/keystore
617572611441ddcb22724420b87ee295c6d47c5adff0ce598c87d3c749b776ba9a647f04
6772616e1441ddcb22724420b87ee295c6d47c5adff0ce598c87d3c749b776ba9a647f04
```
4. 启动
```
./target/release/node-template \
  --base-path /tmp/node01 \
  --chain ./customSpecRaw.json \
  --name MyNode01 \
  --validator \
  --port 30333 \
  --ws-port 9945 \
  --rpc-port 9933 \
  --rpc-methods Unsafe \
  --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
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
  --name MyNode02 \
  --validator \
  --port 30334 \
  --ws-port 9946 \
  --rpc-port 9934 \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWGrTU61wQQkpjCNKd1rjdLbJWy8CCUZYP9qztJcTrSE4s \
  --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
  --rpc-methods Unsafe \
  --password-interactive
```

# 授权特定节点加入网络
[docs](https://docs.substrate.io/tutorials/get-started/permissioned-network/)

## 添加节点授权托盘
1. 添加依赖
`runtime/Cargo.toml`
```
[dependencies]
pallet-node-authorization = { default-features = false, version = "4.0.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v0.9.27" }

[features]
default = ['std']
std = [
 ...
 "pallet-node-authorization/std",    # add this line
 ...
]
```
2. 检查
`cargo check -p node-template-runtime`

## 添加管理规则
1. 将托盘配置为使用`EnsureRoot`可以使用 Sudo 托盘调用`pallet-node-authorization`的功能
   1. 在`runtime/src/lib.rs`中
   2. 添加 `use frame_system::EnsureRoot;`
2. 实现 Config 特征
- 在`runtime/src/lib.rs`中
```
parameter_types! {
 pub const MaxWellKnownNodes: u32 = 8;
 pub const MaxPeerIdLength: u32 = 128;
}

impl pallet_node_authorization::Config for Runtime {
 type Event = Event;
 type MaxWellKnownNodes = MaxWellKnownNodes;
 type MaxPeerIdLength = MaxPeerIdLength;
 type AddOrigin = EnsureRoot<AccountId>;
 type RemoveOrigin = EnsureRoot<AccountId>;
 type SwapOrigin = EnsureRoot<AccountId>;
 type ResetOrigin = EnsureRoot<AccountId>;
 type WeightInfo = ();
}

construct_runtime! 宏中
NodeAuthorization: pallet_node_authorization::{Pallet, Call, Storage, Event<T>, Config<T>},
```
3. 检查
`cargo check -p node-template-runtime`

## 为授权节点添加创世存储
额外的配置来处理对等标识符和帐户标识符。例如，PeerId以 [bs58](https://docs.rs/bs58/latest/bs58/) 格式编码
1. 在`node/Cargo.toml`
```
[dependencies]
bs58 = "0.4.0"
```
2. 在`node/src/chain_spec.rs`
```
use sp_core::OpaquePeerId; // OpaquePeerId 结构包装了Vec<u8>，表示为我们的“PeerId”。
use node_template_runtime::NodeAuthorizationConfig; // NodeAuthorization的创世配置
```
3. 找到testnet_genesis为 FRAME 模块配置初始存储状态的函数
   1. 第一个元素是PeerId,bs58::decode 将人类可读的（PeerId）转换为字节
   2. 第二个元素是AccountId表示该节点所有者的。
先获取节点密钥和 peerID `subkey generate-node-key`
[Decoded PeerID in hex](https://whisperd.tech/bs58-codec/)
```
node-key: dbe80b6e9cfea7ae517a2006eca1be6083ede57b39e432769f4e2fd418b4ccf8
PeerID:   12D3KooWNkXzBATF2q6a8uDvEfGZGH9bKjHSBWcrThH1AJh2mCWy
PeerID hex: 002408011220c02cb53330c3d1f38bbb7041d927833977bcc8421ddbeadca4c99231f454ad06

node-key: 34a9979011eaab45b50d7cdc11a5cf0ce330e61c4b258b9012d5a0df72867459
PeerID:   12D3KooW9ycrz3ApbHVdfmP8LSHnF8sLzmzrix9Z9pXTHU5wYDLT
PeerID hex: 002408011220025ed7733b11c607bdd9119a2dbb014397786790f9bc9be3fb1334b807923790
```

```
 node_authorization: NodeAuthorizationConfig {
   nodes: vec![
     (
       OpaquePeerId(bs58::decode("12D3KooWNkXzBATF2q6a8uDvEfGZGH9bKjHSBWcrThH1AJh2mCWy").into_vec().unwrap()),
       endowed_accounts[0].clone()
     ),
     (
       OpaquePeerId(bs58::decode("12D3KooW9ycrz3ApbHVdfmP8LSHnF8sLzmzrix9Z9pXTHU5wYDLT").into_vec().unwrap()),
       endowed_accounts[1].clone()
     ),
   ],
 },
```

## 启动
- 启动第一个节点
```
./target/release/node-template \
  --base-path /tmp/node01 \
  --chain ./customSpecRaw.json \
  --port 30333 \
  --ws-port 9945 \
  --rpc-port 9933 \
  --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
  --node-key=dbe80b6e9cfea7ae517a2006eca1be6083ede57b39e432769f4e2fd418b4ccf8 \
  --validator \
  --rpc-methods Unsafe \
  --name MyNode01 \
  --password-interactive
```
- 第二个节点
```
./target/release/node-template \
  --base-path /tmp/node02 \
  --chain ./customSpecRaw.json \
  --name MyNode02 \
  --validator \
  --port 30334 \
  --ws-port 9946 \
  --rpc-port 9934 \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWLmrYDLoNTyTYtRdDyZLWDe1paxzxTw5RgjmHLfzW96SX \
  --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
  --rpc-methods Unsafe \
  --password-interactive
```
./target/release/node-template \
  --base-path /tmp/node02 \
  --chain ./customSpecRaw.json \
  --name MyNode02 \
  --validator \
  --port 30334 \
  --ws-port 9946 \
  --rpc-port 9934 \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWLmrYDLoNTyTYtRdDyZLWDe1paxzxTw5RgjmHLfzW96SX \
  --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
  --rpc-methods Unsafe \
  --password-interactive



