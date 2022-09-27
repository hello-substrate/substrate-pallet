# offchain-worker 练习

## ppt

- [ppt](./offchain-worker文档.pptx)
- [code](https://github.com/miketang84/oneblockplus_offchain_lesson)

## 知识点

### Offchain Features 三大组件

- https://docs.substrate.io/fundamentals/offchain-operations/
- Offchain Worker
- Offchain Storage
- Offchain Indexing

### 补充说明

- Offchain Worker 可以直接读链上的数据（状态），但不能写
- 链上代码可以向 Offchain Storage 中写数据，但不能读
- 外层 node 和 rpc 可以直接读链上存储的数据，和 Offchain Storage 中存储的数据
  - api.rpc.offchain.localStorageGet
- 外层 node 和 rpc 可以直接向 Offchain Storage 中写数据，但不能直接向链上存储写数据（必须通过发交易更改链上状态）
  - api.rpc.offchain.localStorageSet

## offchain_worker 打印日志

- 运行: `make 1`
- 给 pallet 添加 offchain worker hook
- 在 offchain worker 中打印信息，观察日志出现的时机

## offchain_worker 的生命周期

### 理解 offchain worker 的执行时机

- 运行: `make 2`
- 同时打开多个 hooks，on_initialize, on_finalize, on_idle
- 在各个 hooks 中打印信息，观察日志出现的时机，理解 offchain worker 的执行时机

### 观察 offchain worker 的跨块执行效果

- `make 3`
- 在 offchain worker 中 sleep 一段时间，观察 offchain worker 的跨块执行效果

## Local Storage 存储

> - Offchain Worker 可直接读写 Local Storage
> - 链上代码可通过 Offchain Indexing 功能直接向 Local Storage 写入数据，但是不能读
> - 可用于 Offchain Worker tasks 之间的通信和协调，注意由于可能同时存在多个 Offchain worker，因此可能对存储的访问需要 lock

- `make 4`
- 设计为在奇数块向 Local Storage 写数据，偶数块读取数据，并检查
- 可以学到：如何获取链下随机数，如何对 BlockNumer 类型进行数学运算，如何获取链下时间，如何生成存储 key，如何写链下存储，如何读链下存储，如何清理存储 key

## mutate 方法对数据进行原子更改

- `make 5`
- 在示例 4 的基础上，使用 mutate 方法对数据进行原子更改
- 可以学到：新的原子操作修改方法（不再使用之前手动锁的方式），学习配套的错误处理模式

## 外部 Http 接口 Json 数据

- `make 6`
- 学会在 Offchain Worker 中发起 https 请求，获取数据（boilerplate）
- 学习如何使用 serde_json 解析获取到的 json 数据
- 学习 serde 的类型转换和调试相关知识

## 向链上发送签名交易

- `make 7`
- Node, runtime, pallet 中都需要改，地方有点多
- 大部分都是 boilerplate 代码，非常固定，不需要理解每个地方。重点要理解原理和整个流程
- 签名交易，需要有账户给它签名。交易的执行会向这个账户收 tx fee
- 本示例提供了能运行的最小化实例，没有多余的代码干扰
- 多查阅 https://crates.parity.io/sc_service/index.html

### 新生成一个账户放在 keystore 中

在 ./node/src/service.rs 中 搜索 offchain_worker 添加代码

```
if config.offchain_worker.enabled {
  let keystore = keystore_container.sync_keystore();
  // Initialize seed for signing transaction using off-chain workers
  sp_keystore::SyncCryptoStore::sr25519_generate_new(
      &*keystore,
      node_template_runtime::pallet_example::KEY_TYPE,// 将账户注入 pallet 中使用
      Some("//Alice"),
  )
  .expect("Creating key with account Alice should succeed.");
}
```

### runtime/src/libs

```
// 签名交易模版代码
use codec::Encode;
use sp_runtime::SaturatedConversion; // saturated_into 操作依赖
impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Runtime
where
	Call: From<LocalCall>,
{
	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: Call,
		public: <Signature as sp_runtime::traits::Verify>::Signer,
		account: AccountId,
		nonce: Index,
	) -> Option<(Call, <UncheckedExtrinsic as sp_runtime::traits::Extrinsic>::SignaturePayload)> {
		let tip = 0;

		let period =
			BlockHashCount::get().checked_next_power_of_two().map(|c| c / 2).unwrap_or(2) as u64;
		let current_block = System::block_number().saturated_into::<u64>().saturating_sub(1);
		let era = generic::Era::mortal(period, current_block);
		let extra = (
			frame_system::CheckNonZeroSender::<Runtime>::new(),
			frame_system::CheckSpecVersion::<Runtime>::new(),
			frame_system::CheckTxVersion::<Runtime>::new(),
			frame_system::CheckGenesis::<Runtime>::new(),
			frame_system::CheckEra::<Runtime>::from(era),
			frame_system::CheckNonce::<Runtime>::from(nonce),
			frame_system::CheckWeight::<Runtime>::new(),
			// pallet_asset_tx_payment::ChargeAssetTxPayment::<Runtime>::from(tip, None),
			pallet_transaction_payment::ChargeTransactionPayment::<Runtime>::from(tip),
		);
		let raw_payload = SignedPayload::new(call, extra)
			.map_err(|_| {
				//log::warn!("Unable to create signed payload: {:?}", e);
			})
			.ok()?;
		let signature = raw_payload.using_encoded(|payload| C::sign(payload, public))?;
		let address = account;
		let (call, extra, _) = raw_payload.deconstruct();
		Some((call, (sp_runtime::MultiAddress::Id(address), signature.into(), extra)))
	}
}

impl frame_system::offchain::SigningTypes for Runtime {
	type Public = <Signature as sp_runtime::traits::Verify>::Signer;
	type Signature = Signature;
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for Runtime
where
	Call: From<C>,
{
	type Extrinsic = UncheckedExtrinsic;
	type OverarchingCall = Call;
}

// local or new pallet
impl pallet_example::Config for Runtime {
	type Event = Event;
	type AuthorityId = pallet_example::crypto::OcwAuthId;
	...
}
```

## 未签名交易

- `make 8`
- 要改 runtime 和 pallet，比签名交易代码要少一些
- 大部分都是 boilerplate 代码，非常固定。重点要理解原理和整个流程
- 未签名交易，无 tx fee，因为找不到人收费
- #[pallet::validate_unsigned], TransactionValidity, ValidTransaction, ensure_none 等
- 每个块的未签名交易不能太多，Substrate 不鼓励发未签名交易

### runtime/src/libs

```
// 未签名交易
impl<C> frame_system::offchain::SendTransactionTypes<C> for Runtime
	where
		Call: From<C>,
{
	type Extrinsic = UncheckedExtrinsic;
	type OverarchingCall = Call;
}
```

## 未签名交易并带有签名负载

在 [向链上发送签名交易](https://github.com/hello-substrate/substrate-pallet/tree/offchain-worker#向链上发送签名交易)
基础上修改代码

- `make 9`

## 使用 offchain index 从链上向链下存储写数据

- `make 10`

## 在 front 前端中 获取链下存储的数据

- [front](./substrate-front-end-template)

## 完整的案例

- https://github.com/paritytech/substrate/blob/master/frame/examples/offchain-worker/src/lib.rs
- `make 11`
