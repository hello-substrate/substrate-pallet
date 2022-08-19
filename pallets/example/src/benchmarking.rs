use super::*;

#[allow(unused)]
use crate::Pallet as Example;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_support::traits::{OnFinalize, OnInitialize};
use frame_system::{EventRecord, Pallet as System, RawOrigin};
use sp_runtime::traits::{Bounded, One};

/* https://docs.substrate.io/reference/how-to-guides/weights/add-benchmarks/
// 模拟运行时验证 `pallet-example` 生成的基准单元测试
cargo test --package pallet-example --features runtime-benchmarks
cargo build --release --features runtime-benchmarks
mkdir .maintain && cd .maintain
#此处的substrate同官方的repo仓库
cp ../../substrate/.maintain/frame-weight-template.hbs .
./target/release/node-template benchmark pallet --help

./target/release/node-template benchmark pallet \
  --chain=dev \
  --execution=wasm \
  --wasm-execution=compiled \
  --pallet=pallet_example \
  --extrinsic="*" \
  --steps=100
  --repeat=10 \
  --template=./.maintain/frame-weight-template.hbs \
  --output=./pallets/example/src/weights.rs

./target/release/node-template benchmark pallet \
--chain dev \
--pallet pallet_example \
--extrinsic '*' \
--steps 20 \
--repeat 10 \
--output pallets/example/src/weights.rs

*/

// https://github.com/paritytech/substrate/blob/master/frame/transaction-storage/src/benchmarking.rs
// 检查最后一个 event 是否一致
// assert_last_event::<T>(Event::Renewed { index: 0 }.into());
fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	let events = System::<T>::events();
	let system_event: <T as frame_system::Config>::Event = generic_event.into();
	let EventRecord { event, .. } = &events[events.len() - 1];
	assert_eq!(event, &system_event);
}
// 跳转指定块
pub fn run_to_block<T: Config>(n: T::BlockNumber) {
	while frame_system::Pallet::<T>::block_number() < n {
		Example::<T>::on_finalize(frame_system::Pallet::<T>::block_number());
		frame_system::Pallet::<T>::on_finalize(frame_system::Pallet::<T>::block_number());
		frame_system::Pallet::<T>::set_block_number(
			frame_system::Pallet::<T>::block_number() + One::one(),
		);
		frame_system::Pallet::<T>::on_initialize(frame_system::Pallet::<T>::block_number());
		Example::<T>::on_initialize(frame_system::Pallet::<T>::block_number());
	}
}

benchmarks! {
  // // 在此处添加单独的基准
  create_fund {
	 /* 1.初始化需用到的数据 */
	// 声明函数用到的参数
	let caller: T::AccountId = whitelisted_caller();
	let beneficiary_account_id: T::AccountId = frame_benchmarking::account("beneficiary_account_id",0,0);
	let goal_raise:BalanceOf<T>= 1000u32.into();
	let end_block:T::BlockNumber= 9u32.into();
	// 给账户打钱
	T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
	// run_to_block::<T>(1u32.into());
	// 先创建一个
	// Example::<T>::create_fund(RawOrigin::Signed(caller.clone()).into(),beneficiary_account_id.clone(),goal_raise,end_block)?;
	// BalanceOf<T> = 2u32.into()
  }: {
	 /* 2.调用调度函数 */
		Example::<T>::create_fund(RawOrigin::Signed(caller).into(),beneficiary_account_id,goal_raise,end_block)?;
	}
  verify {
	 /* 3.进行验证(可选) */
	assert_eq!(FundCount::<T>::get(), 1);
	assert_last_event::<T>(Event::Created(1, 1u32.into()).into());
  }

  // 使用mock中的new_test_ext
  impl_benchmark_test_suite!(Example, crate::mock::new_test_ext(), crate::mock::Test);
}
