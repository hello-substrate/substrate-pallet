# 联盟链

## 随机生成账号

生成的账户需要导入浏览器钱包中以供测试使用(如 polkadot 钱包插件)

### 第一个

```
subkey generate --scheme Sr25519 --password-interactive
Secret phrase:       attract broken prize foam expand clog scene net put broccoli whip bar
  Network ID:        substrate
  Secret seed:       0x7f97b4ac88cb629402cca06a69bb0eb67590d023e5fec1b0f219ff1d41dd9589
  Public key (hex):  0x7a37a1689bdf29ee90cffe2f9168993221d650750ba5b61ede9bc5fa7f61144f
  Account ID:        0x7a37a1689bdf29ee90cffe2f9168993221d650750ba5b61ede9bc5fa7f61144f
  Public key (SS58): 5EpxDzVtmm9SsXH9TuksJyKy4TmcPSpw3Ao7nPdVSRUujPyN
  SS58 Address:      5EpxDzVtmm9SsXH9TuksJyKy4TmcPSpw3Ao7nPdVSRUujPyN

subkey inspect --password-interactive --scheme Ed25519 "attract broken prize foam expand clog scene net put broccoli whip bar"
Secret phrase:       attract broken prize foam expand clog scene net put broccoli whip bar
  Network ID:        substrate
  Secret seed:       0x7f97b4ac88cb629402cca06a69bb0eb67590d023e5fec1b0f219ff1d41dd9589
  Public key (hex):  0xbef30ffbbc9e5477085bd1b3ba050d1d8b2a90798f72db3d5e408fa772ddd309
  Account ID:        0xbef30ffbbc9e5477085bd1b3ba050d1d8b2a90798f72db3d5e408fa772ddd309
  Public key (SS58): 5GP5AE63KXmwcXeKcudUpWibXHGxU5gtAfFsaSnFjtX4yB5f
  SS58 Address:      5GP5AE63KXmwcXeKcudUpWibXHGxU5gtAfFsaSnFjtX4yB5f
```

### 其余三个

```
Secret phrase:       garment calm oppose urban genre mango hobby brown scrub admit item globe
  Network ID:        substrate
  Secret seed:       0x55468d5b72d617bd46b2f2a0c962e00d7142bc854e04680f35751b4bd763a3ae
  Public key (hex):  0x3e7d28a354a9a2b720a785788cba777882e0ef20d797e50cc00adb1f4f467f3f
  Account ID:        0x3e7d28a354a9a2b720a785788cba777882e0ef20d797e50cc00adb1f4f467f3f
  Public key (SS58): 5DUe1JBcPM9kLVcfXQhSdJqETKx3USdDH4nrgRCL3NkT33vW
  SS58 Address:      5DUe1JBcPM9kLVcfXQhSdJqETKx3USdDH4nrgRCL3NkT33vW
subkey inspect --password-interactive --scheme Ed25519 "garment calm oppose urban genre mango hobby brown scrub admit item globe"
Secret phrase:       garment calm oppose urban genre mango hobby brown scrub admit item globe
  Network ID:        substrate
  Secret seed:       0x55468d5b72d617bd46b2f2a0c962e00d7142bc854e04680f35751b4bd763a3ae
  Public key (hex):  0xcb76dc10cfc3fae1750c57d26dfa6799041445e943430de9123ce2afda81742f
  Account ID:        0xcb76dc10cfc3fae1750c57d26dfa6799041445e943430de9123ce2afda81742f
  Public key (SS58): 5GfUtNEy5bcqyPj5B1mX5GAPb8CvMkbjzxtWyBMHi38kWPJr
  SS58 Address:      5GfUtNEy5bcqyPj5B1mX5GAPb8CvMkbjzxtWyBMHi38kWPJr

Secret phrase:       thunder black neglect brush van slam question adapt blush debate piece chair
  Network ID:        substrate
  Secret seed:       0x6d3c76ed936d94a6ee005250b128d43a26e7cdac7db1d115c03c8eb5c6b8b0cc
  Public key (hex):  0xbec06d51fe564cd15685c87aeb6fcd498093acdee960f8cdc8a4d3bbcbc40d5b
  Account ID:        0xbec06d51fe564cd15685c87aeb6fcd498093acdee960f8cdc8a4d3bbcbc40d5b
  Public key (SS58): 5GNp7oHLi3V4hdrPEDNmt6WzQuZMve5Rk8yjWvYrLQ9HGSF2
  SS58 Address:      5GNp7oHLi3V4hdrPEDNmt6WzQuZMve5Rk8yjWvYrLQ9HGSF2
subkey inspect --password-interactive --scheme Ed25519 "thunder black neglect brush van slam question adapt blush debate piece chair"
Secret phrase:       thunder black neglect brush van slam question adapt blush debate piece chair
  Network ID:        substrate
  Secret seed:       0x6d3c76ed936d94a6ee005250b128d43a26e7cdac7db1d115c03c8eb5c6b8b0cc
  Public key (hex):  0x2e7d63f0c0617e083e8d81a608613bd68667f10a9f52743d77fec5d9e9a1d92e
  Account ID:        0x2e7d63f0c0617e083e8d81a608613bd68667f10a9f52743d77fec5d9e9a1d92e
  Public key (SS58): 5D7fJrL3n66rMVr2Tf4nKpwSZdmKGiCRUj1tGviPxAFbmiDR
  SS58 Address:      5D7fJrL3n66rMVr2Tf4nKpwSZdmKGiCRUj1tGviPxAFbmiDR

Secret phrase:       omit tenant festival suggest dutch host shallow shoulder sort protect excess end
  Network ID:        substrate
  Secret seed:       0x909cdd4ed7bd324d8e7245e1d9ba990df82deee3e08e641655e092ca9448dd21
  Public key (hex):  0xa6e1e0bfd1026295fb0f6aa87a54778cbbf649c3d465b45db07cdfb896713c74
  Account ID:        0xa6e1e0bfd1026295fb0f6aa87a54778cbbf649c3d465b45db07cdfb896713c74
  Public key (SS58): 5FqWuVJTDyLpzT54qQNGgjrYXnVVicFfYNNsnkFEKrzBM7ec
  SS58 Address:      5FqWuVJTDyLpzT54qQNGgjrYXnVVicFfYNNsnkFEKrzBM7ec
subkey inspect --password-interactive --scheme Ed25519 "omit tenant festival suggest dutch host shallow shoulder sort protect excess end"
Secret phrase:       omit tenant festival suggest dutch host shallow shoulder sort protect excess end
  Network ID:        substrate
  Secret seed:       0x909cdd4ed7bd324d8e7245e1d9ba990df82deee3e08e641655e092ca9448dd21
  Public key (hex):  0x6456965a7c62a7808d092765debb01c75bef1f387a059f784e9bcd912a4df833
  Account ID:        0x6456965a7c62a7808d092765debb01c75bef1f387a059f784e9bcd912a4df833
  Public key (SS58): 5ELGNGK37FeKy5sagJ26KxE2XwDbANk2A7fRXwbJpMg9idiH
  SS58 Address:      5ELGNGK37FeKy5sagJ26KxE2XwDbANk2A7fRXwbJpMg9idiH
```

## 生成 node 节点的 密钥和 peerID

- subkey generate-node-key
- 解码 peerID https://whisperd.tech/bs58-codec/

| 账户 | 节点密钥                                                         | PeerId                                               | Decoded PeerID in hex                                                        |
| ---- | ---------------------------------------------------------------- | ---------------------------------------------------- | ---------------------------------------------------------------------------- |
| 1    | 2771a6fb1ee93a39773f4f26715966cad41db0d843c8e60f48b9e2cadf6b5906 | 12D3KooWS1bPAhtw8Rq7fi86sxAbSn8FygbtrEFqmc2cdwpkSbdW | 002408011220f09b523170f609580ec0a35e19fe8db8c79cc32b8fb67b3b7c51e3212d1f7385 |
| 2    | 84decb33517c08018d8c7a18b597a5d8a2ce4cfe57d2ce1e97774da1368bb6a4 | 12D3KooWSF9KBXnESAVvd8p7pmZpYi4ykmVwHYYqWcp6SR5dckwE | 002408011220f413f9f53b2990661382e262028666f3c26ac19c35c7b1a43cad0bc425577d0d |
| 3    | e4e6a2fe748c955b7306edfbe00c9868e7eab2b919427188dfe4bedb2de1d57c | 12D3KooWRgMZq9SMvrAgJAyHjjRExG14MXpbqWQYXSk7c8vJe293 | 002408011220ebada9e91ff135d71f47fa3c4daf336fc569db64baf04999ed768998bf11b22e |
| 4    | 05e36c497df61895ca7a3a548dcaf475d22edc070416b910dba13b6b15d07888 | 12D3KooWS3kx9oMGpBupCpyXCsJFB1UgPdAdU6Mw7DWnPP3nsa5y | 002408011220f1294d52c768ebe73f54b9adfb3a022c0f4676a98fc2c67f541928ae07e2b524 |

## 添加 pallet-node-authorization (runtime/Cargo.toml)

```
[dependencies]
pallet-node-authorization = { default-features = false, version = "4.0.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v0.9.27" }

[features]
std = [
 "pallet-node-authorization/std",    # add this line
]
```

## 添加管理规则(runtime/src/lib.rs)

> 配置 pallet-node-authorization 使用 EnsureRoot 特权方法, Sudo pallet 可调用的.
> Sudo 托盘默认包含在节点模板中，使您能够通过 root 级管理帐户进行调用。在生产环境中，您将使用更现实的基于治理的检查。

## 为托盘实现 Config 特征(runtime/src/lib.rs)

```
use frame_system::EnsureRoot;
impl pallet_node_authorization::Config for Runtime {
	type Event = Event;
	type MaxWellKnownNodes = ConstU32<8>;
	type MaxPeerIdLength = ConstU32<128>;
	type AddOrigin = EnsureRoot<AccountId>;
	type RemoveOrigin = EnsureRoot<AccountId>;
	type SwapOrigin = EnsureRoot<AccountId>;
	type ResetOrigin = EnsureRoot<AccountId>;
	type WeightInfo = ();
}

construct_runtime!(
  NodeAuthorization: pallet_node_authorization::{Pallet, Call, Storage, Event<T>, Config<T>},
});
```

## 为授权节点添加创世存储(node/Cargo.toml)

> PeerId 以 bs58 格式编码，因此您需要在 node/Cargo.toml 中添加 bs58 库依赖项来解码 PeerId 以获取其字节。

```
[dependencies]
bs58 = { version = "0.4.0" }
```

## 创世存储添加有权加入网络的节点(node/src/chain_spec.rs)

```
use sp_core::OpaquePeerId; // A struct wraps Vec<u8>, represents as our `PeerId`.
use node_template_runtime::NodeAuthorizationConfig;

GenesisConfig 初始化默认已授权的 PeerID

node_authorization: NodeAuthorizationConfig {
  nodes: vec![
    (
      OpaquePeerId(bs58::decode("12D3KooWS1bPAhtw8Rq7fi86sxAbSn8FygbtrEFqmc2cdwpkSbdW").into_vec().unwrap()),
      endowed_accounts[0].clone()
    ),
    (
      OpaquePeerId(bs58::decode("12D3KooWSF9KBXnESAVvd8p7pmZpYi4ykmVwHYYqWcp6SR5dckwE").into_vec().unwrap()),
      endowed_accounts[1].clone()
    ),
  ],
},
```

## check

`cargo check -p node-template-runtime`

## 自定义链规范

### 基于 local 链规范修改

在 `node/src/chain_spec.rs` 中

- 修改 `fn get_from_seed`

```
TPublic::Pair::from_string(&format!("//{}", seed), None)
改为,以可以匹配 Secret phrase(短语) 与 Secret seed(0x开头的 64位 hex 字符串)
let mut seed = seed.to_string(); // seed 末尾 ///xx ///后面表示密码
if seed.len() < 20 {
    seed = format!("//{}", seed); //兼容 local与dev链规范的 Alice等需要`//`开头的账户
}
TPublic::Pair::from_string(&seed, None)
```

- 复制并修改 `fn local_testnet_config -> fn custom_testnet_config`

```
pub fn custom_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;
	let root_seed = "0x7f97b4ac88cb629402cca06a69bb0eb67590d023e5fec1b0f219ff1d41dd9589";
	let sun_seed = "0x55468d5b72d617bd46b2f2a0c962e00d7142bc854e04680f35751b4bd763a3ae";
	let wen_seed = "0x6d3c76ed936d94a6ee005250b128d43a26e7cdac7db1d115c03c8eb5c6b8b0cc";
	let ming_seed = "0x909cdd4ed7bd324d8e7245e1d9ba990df82deee3e08e641655e092ca9448dd21";
	Ok(ChainSpec::from_genesis(
		"Custom Testnet", // Name
		"custom_testnet", // ID
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed(root_seed), authority_keys_from_seed(sun_seed)],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>(root_seed),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>(root_seed),
					get_account_id_from_seed::<sr25519::Public>(sun_seed),
					get_account_id_from_seed::<sr25519::Public>(wen_seed),
					get_account_id_from_seed::<sr25519::Public>(ming_seed),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		None,
		// Extensions
		None,
	))
}
```

- 导出链规范

```
cargo build --release && ./target/release/node-template build-spec --disable-default-bootnode --chain custom > customSpec.json

1. name 链的名称
2. chainType 只允许 Development, Local, Live, { "Custom": "Whatever you want" },
3. aura 添加 Sr25519 SS58 地址密钥来指定有权创建块的节点
4. grandpa 添加 Ed25519 SS58 地址密钥来指定有权完成区块的节点 第二个参数加权投票,默认权重为1票
5. balances 初始化账户余额
```

- 必须将其转换为原始规范格式才能使用

```
./target/release/node-template build-spec --chain=customSpec.json --raw --disable-default-bootnode > customSpecRaw.json
```

- 新增链启动参数 --chain custom

```
node/src/command.rs -> fn load_spec

"custom" => Box::new(chain_spec::custom_testnet_config()?),
```

## 将账户密钥加入到节点目录中

- aura 用于创建块
- gran 验证和完成块
- 清理缓存 `rm -rf /tmp/node01 && rm -rf /tmp/node02`

```
./target/release/node-template key insert --base-path /tmp/node01 \
    --chain customSpecRaw.json --scheme Sr25519 \
    --suri "attract broken prize foam expand clog scene net put broccoli whip bar" \
    --password-interactive --key-type aura
./target/release/node-template key insert --base-path /tmp/node01 \
    --chain customSpecRaw.json --scheme Sr25519 \
    --suri "0x7f97b4ac88cb629402cca06a69bb0eb67590d023e5fec1b0f219ff1d41dd9589" \
    --password-interactive --key-type aura
./target/release/node-template key insert --base-path /tmp/node01 \
    --chain customSpecRaw.json --scheme Ed25519 \
    --suri "0x7f97b4ac88cb629402cca06a69bb0eb67590d023e5fec1b0f219ff1d41dd9589" \
    --password-interactive --key-type gran
ls /tmp/node01/chains/custom_testnet/keystore

节点二
./target/release/node-template key insert --base-path /tmp/node02 \
    --chain customSpecRaw.json --scheme Sr25519 \
    --suri "0x55468d5b72d617bd46b2f2a0c962e00d7142bc854e04680f35751b4bd763a3ae" \
    --password-interactive --key-type aura
./target/release/node-template key insert --base-path /tmp/node02 \
    --chain customSpecRaw.json --scheme Ed25519 \
    --suri "0x55468d5b72d617bd46b2f2a0c962e00d7142bc854e04680f35751b4bd763a3ae" \
    --password-interactive --key-type gran
ls /tmp/node02/chains/custom_testnet/keystore
```

## 启动网络

启动完成可以看到块的产出

```
./target/release/node-template --base-path /tmp/node01 \
    --chain ./customSpecRaw.json \
    --port 30333 \
    --ws-port 9944 \
    --rpc-port 9933 \
    --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
    --node-key=2771a6fb1ee93a39773f4f26715966cad41db0d843c8e60f48b9e2cadf6b5906 \
    --rpc-methods Unsafe \
    --name root --validator \
    --password-interactive

./target/release/node-template --base-path /tmp/node02 \
    --chain ./customSpecRaw.json \
    --port 30334 \
    --ws-port 9945 \
    --rpc-port 9934 \
    --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
    --node-key=84decb33517c08018d8c7a18b597a5d8a2ce4cfe57d2ce1e97774da1368bb6a4 \
    --rpc-methods Unsafe \
    --name sun --validator \
    --password-interactive
```

## 将第三个节点加入the list of well-known nodes

> node-authorization 托盘使用链下工作者来配置节点连接.由于此节点的账户不在 node_authorization 创世存储中配置.
> 不是一个 well-known node,并将第四个节点网络配置为只读子节点. 必须在命令行选项以启用链外工作者程序(--offchain-worker
> always)

> 未在创世存储初始化的节点无法连接到 peers. 所以必须授权此节点才可连接.
> 调用 Sudo 托盘手动添加所有其他未创世存储的节点

```
./target/release/node-template --base-path /tmp/node03 \
    --chain ./customSpecRaw.json \
    --name wen --validator \
    --node-key=e4e6a2fe748c955b7306edfbe00c9868e7eab2b919427188dfe4bedb2de1d57c \
    --port 30335 \
    --ws-port 9946 \
    --offchain-worker always
```

### 授权第三个节点的访问

- 打开[polkadot.js.org](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer)

> Developer -> Sudo -> nodeAuthorization -> add_well_known_node(若 sudo 标签不存在,检查 accounts 中是否包含 sudo key
> 配置的账户).

- 参数 node:PeerId -> bs58 decoded的值 0x002408011220ebada9e91ff135d71f47fa3c4daf336fc569db64baf04999ed768998bf11b22e
- 参数 owner:AccountId -> 选择一个账户

> 这三个节点可以使用本地网络中默认启用的mDNS发现机制找到彼此,节点不在同一个本地网络上，您应该使用命令行选项--no-mdns来禁用它

## 添加子节点

> 子节点只能通过连接到 某个账户 拥有的节点才能访问网络.
> 父节点负责所有子节点的连接授权和访问控制(如果子节点需要移除或审计).

- 此处父节点使用上方的 wen 账户

```
./target/release/node-template --base-path /tmp/node04 \
    --chain ./customSpecRaw.json \
    --name ming --validator \
    --node-key=05e36c497df61895ca7a3a548dcaf475d22edc070416b910dba13b6b15d07888 \
    --port 30336 \
    --ws-port 9947 \
    --offchain-worker always
```

### ming 声明为节点的所有者

> ming 账户提交一个 nodeAuthorization -> claim_node

- node:PeerId = ming 节点的 0x002408011220f1294d52c768ebe73f54b9adfb3a022c0f4676a98fc2c67f541928ae07e2b524

### wen 配置允许来自 ming 的节点连接

> 在 Developer -> Extrinsic 中 wen 账户提交一个 nodeAuthorization -> addConnections

- node:PeerId = wen 节点的 0x002408011220ebada9e91ff135d71f47fa3c4daf336fc569db64baf04999ed768998bf11b22e
- connections: Vec<PeerId>(允许连接的节点数组) = ming 节点的
  0x002408011220f1294d52c768ebe73f54b9adfb3a022c0f4676a98fc2c67f541928ae07e2b524

### ming 配置允许来自 wen 的节点连接

> 在 Developer -> Extrinsic 中 ming 账户提交一个 nodeAuthorization -> addConnections

- node:PeerId = ming 节点的 0x002408011220f1294d52c768ebe73f54b9adfb3a022c0f4676a98fc2c67f541928ae07e2b524
- connections: Vec<PeerId>(允许连接的节点数组) = wen 节点的
  0x002408011220ebada9e91ff135d71f47fa3c4daf336fc569db64baf04999ed768998bf11b22e

> 现在应该看到 ming 正在追赶 blocks,并且仅仅只有一个 peer 属于 wen 节点！重新启动 ming 的节点，以防它没有立即与 web 连接

> 到此,任何运行的节点都可以发出 Extrinsic 影响其他节点的链上数据,并且您在 keystore 中拥有账户的签名密钥.
> 此演示中的所有节点都可以访问开发人员签名密钥,因此我们能够代表 wen 从我们网络上的任何连接节点发出影响 wen 子节点的命令.
> 在现实世界的应用程序中,节点操作员只能访问他们的节点密钥,并且是唯一能够正确签署和提交外部信息的人,很可能来自他们自己的节点，他们可以控制密钥的安全性。
