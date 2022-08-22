# 升级运行时
https://docs.substrate.io/tutorials/get-started/forkless-upgrade/#schedule-an-upgrade

## 使用 Sudo 托盘授权升级
- 添加依赖 `runtime/Cargo.toml`
```
[dependencies]
pallet-scheduler = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v0.9.26" }

[features]
default = ["std"]
std = [
 "pallet-scheduler/std",
]
```
- 添加调度程序托盘到`runtime/src/lib.rs`
```
parameter_types! {
 pub MaximumSchedulerWeight: Weight = 10_000_000;
 pub const MaxScheduledPerBlock: u32 = 50;
}

pub use frame_support::traits::EqualPrivilegeOnly;

impl pallet_scheduler::Config for Runtime {
 type Event = Event;
 type Origin = Origin;
 type PalletsOrigin = OriginCaller;
 type Call = Call;
 type MaximumWeight = MaximumSchedulerWeight;
 type ScheduleOrigin = frame_system::EnsureRoot<AccountId>;
 type MaxScheduledPerBlock = MaxScheduledPerBlock;
 type WeightInfo = ();
 type OriginPrivilegeCmp = EqualPrivilegeOnly;
 type PreimageProvider = ();
 type NoPreimagePostponement = ();
}

construct_runtime!(
 pub enum Runtime where
 Block = Block,
 NodeBlock = opaque::Block,
 UncheckedExtrinsic = UncheckedExtrinsic
 {
   /*** snip ***/
   Scheduler: pallet_scheduler,
 }
);
```
- 更新`runtime/src/lib.rs` 的 `spec_version` +=1
```
pub const VERSION: RuntimeVersion = RuntimeVersion {
 spec_name: create_runtime_str!("node-template"),
 impl_name: create_runtime_str!("node-template"),
 authoring_version: 1,
 spec_version: 101,  // *Increment* this value.
 impl_version: 1,
 apis: RUNTIME_API_VERSIONS,
 transaction_version: 1,
};
```
- 构建升级的运行时
`cargo build --release -p node-template-runtime`
- 选择 Root 用户 如`Alice` 调用 `sudo` pallet 中的 `sudoUncheckedWeight`
  - 参数
  - weight：默认0
  - call: 选择 system 的 set_code 上传构建的 `/target/release/wbuild/node-template-runtime/node_template_runtime.compact.compressed.wasm`
- Polkadot JS Apps UI 左上角的版本号应该反映运行时版本为 now 101

## 升级运行时

链中有了 `pallet-scheduler` 后可以调用`Scheduler` 升级运行时
- [打开本地区块浏览器](https://polkadot.js.org/apps/#/sudo?rpc=ws://127.0.0.1:9944)
选择`Scheduler`-> `schedule` 函数

- 参数
  - maybe_periodic: 默认空
  - priority：默认0
  - call: 选择 system 的 set_code 上传构建的 `/target/release/wbuild/node-template-runtime/node_template_runtime.compact.compressed.wasm`
  - when: 执行之前当前块(如100)向后推迟10个左右(则填写110)

当 `when` 所填区块完成后.