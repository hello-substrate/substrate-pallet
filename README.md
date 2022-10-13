# substrate-node-template 集成 pallet-contract

[下方文档地址](https://github.com/hello-substrate/hello-substrate/blob/main/docs/substrate/contracts/添加合约pallet-contracts.md)

[官网文档](https://docs.substrate.io/tutorials/work-with-pallets/contracts-pallet/) & [官方文档 2](https://docs.substrate.io/reference/how-to-guides/pallet-design/add-contracts-pallet/)

# 开始之前

- [ ] 按照 [构建本地区块链](https://github.com/hello-substrate/hello-substrate/blob/main/docs/substrate/get-start/构建本地区块链.md) 下载编译了 node-template

# 添加 pallet 依赖

1. 打开 `runtime/Cargo.toml`

2. `[dependencies]`新增

   ```
   # 注意 v0.9.28 版本必须与其他 pallet 使用的一致
   pallet-contracts = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v0.9.28" }
   pallet-contracts-primitives = { version = "6.0.0", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v0.9.28" }
   ```

3. `[features]` -> `std = [...]` 中新增
   ```
   "pallet-contracts/std",
   "pallet-contracts-primitives/std",
   ```

# 实现 Contracts config trait

1. 打开 `runtime/src/lib.rs`
2. 导入 `pallet-contracts` 默认的 contract weight
   ```
   use pallet_contracts::DefaultContractAccessWeight;
   ```
3. 实现 `pallet_contracts::Config`

   ```
   // pallet-contracts
   // Contracts price units.
   pub const MILLICENTS: Balance = 1_000_000_000;
   pub const CENTS: Balance = 1_000 * MILLICENTS;
   pub const DOLLARS: Balance = 100 * CENTS;
   const fn deposit(items: u32, bytes: u32) -> Balance {
      items as Balance * 15 * CENTS + (bytes as Balance) * 6 * CENTS
   }
   const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(10);
   parameter_types! {
      pub const DepositPerItem: Balance = deposit(1, 0);
      pub const DepositPerByte: Balance = deposit(0, 1);
      pub const DeletionQueueDepth: u32 = 128;
      pub DeletionWeightLimit: Weight = AVERAGE_ON_INITIALIZE_RATIO * BlockWeights::get().max_block;
      pub Schedule: pallet_contracts::Schedule<Runtime> = Default::default();
   }

   impl pallet_contracts::Config for Runtime {
      type Time = Timestamp;
      type Randomness = RandomnessCollectiveFlip;
      type Currency = Balances;
      type Event = Event;
      type Call = Call;
      type CallFilter = frame_support::traits::Nothing;
      type WeightPrice = pallet_transaction_payment::Pallet<Self>;
      type WeightInfo = pallet_contracts::weights::SubstrateWeight<Self>;
      type ChainExtension = ();
      type Schedule = Schedule;
      type CallStack = [pallet_contracts::Frame<Self>; 31];
      type DeletionQueueDepth = DeletionQueueDepth;
      type DeletionWeightLimit = DeletionWeightLimit;
      type DepositPerByte = DepositPerByte;
      type DepositPerItem = DepositPerItem;
      type AddressGenerator = pallet_contracts::DefaultAddressGenerator;
      type ContractAccessWeight = DefaultContractAccessWeight<BlockWeights>;
      type MaxCodeLen = ConstU32<{ 256 * 1024 }>;
      type RelaxedMaxCodeLen = ConstU32<{ 512 * 1024 }>;
      type MaxStorageKeyLen = ConstU32<{ 512 * 1024 }>;
   }
   ```

4. 添加 pallet_contracts 到 construct_runtime!宏。
   ```
   Contracts: pallet_contracts,
   ```

# 暴露 Contracts RPC API

1. 打开 `runtime/Cargo.toml`
2. `[dependencies]` 新增
   ```
   pallet-contracts-rpc-runtime-api = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v0.9.28" }
   ```
3. `[features]` -> `std = [...]` 中新增
   ```
   "pallet-contracts-rpc-runtime-api/std",
   ```
4. 打开 `runtime/src/lib.rs`
5. 添加以下常量启用合约调试
   ```
   const CONTRACTS_DEBUG_OUTPUT: bool = true;
   ```
6. 在 `impl_runtime_apis!` 中实现 contracts runtime API

   ```
   impl pallet_contracts_rpc_runtime_api::ContractsApi<Block, AccountId, Balance, BlockNumber, Hash> for Runtime {
      fn call(
         origin: AccountId,
         dest: AccountId,
         value: Balance,
         gas_limit: u64,
         storage_deposit_limit: Option<Balance>,
         input_data: Vec<u8>,
      ) -> pallet_contracts_primitives::ContractExecResult<Balance> {
         Contracts::bare_call(origin, dest, value, gas_limit, storage_deposit_limit, input_data, CONTRACTS_DEBUG_OUTPUT)
      }

      fn instantiate(
         origin: AccountId,
         value: Balance,
         gas_limit: u64,
         storage_deposit_limit: Option<Balance>,
         code: pallet_contracts_primitives::Code<Hash>,
         data: Vec<u8>,
         salt: Vec<u8>,
      ) -> pallet_contracts_primitives::ContractInstantiateResult<AccountId, Balance> {
         Contracts::bare_instantiate(origin, value, gas_limit, storage_deposit_limit, code, data, salt, CONTRACTS_DEBUG_OUTPUT)
      }

      fn upload_code(
         origin: AccountId,
         code: Vec<u8>,
         storage_deposit_limit: Option<Balance>,
      ) -> pallet_contracts_primitives::CodeUploadResult<Hash, Balance> {
         Contracts::bare_upload_code(origin, code, storage_deposit_limit)
      }

      fn get_storage(
         address: AccountId,
         key: Vec<u8>,
      ) -> pallet_contracts_primitives::GetStorageResult {
         Contracts::get_storage(address, key)
      }
   }
   ```

# 更新外部节点

> 至此，您已完成将 pallet-contracts 添加到运行时。现在，您需要考虑外部节点是否需要任何相应的更新。Substrate 提供了一个 RPC 服务器来与节点交互。但是，默认的 RPC 服务器不提供对 pallet-contracts 的访问。要与 pallet-contracts 交互，您必须扩展现有的 RPC 服务器以包含 pallet-contracts 和 Contracts RPC API。要让 pallet-contracts 利用 RPC endpoint API，您需要将自定义 RPC endpoint 添加到外部节点配置中。

1. 打开 `node/Cargo.toml`
2. `[dependencies]` 新增
   ```
   pallet-contracts = { version = "4.0.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v0.9.28" }
   pallet-contracts-rpc = { version = "4.0.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v0.9.28" }
   ```
3. 打开 `node/src/rpc.rs`
4. 添加 `BlockNumber` 和 `Hash` 到 `use node_template_runtime` 中
   ```
   use node_template_runtime::{opaque::Block, AccountId, Balance, Index, BlockNumber, Hash};
   ```
5. 在 `create_full` 方法中添加 `pallet_contracts_rpc`
   ```
   use pallet_contracts_rpc::{Contracts, ContractsApiServer};
   ```
6. 在 `create_full` 方法 `where` 中添加 `pallet-contracts-rpc`
   ```
   C::Api: pallet_contracts_rpc::ContractsRuntimeApi<Block, AccountId, Balance, BlockNumber, Hash>,
   ```
7. 添加 Contracts RPC API 的扩展。
   ```
   //上面的 client 记得是 client.clone()
   module.merge(Contracts::new(client.clone()).into_rpc())?;
   ```
8. 检查 runtime 是否正确编译
   ```
   cargo check -p node-template-runtime --release
   cargo build --release
   ```
