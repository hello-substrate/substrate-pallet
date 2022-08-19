use super::*;

#[allow(unused)]
use crate::Pallet as Example;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

// 模拟运行时执行为 `pallet-example` 生成的基准单元测试
// cargo test --package pallet-example --features runtime-benchmarks
// cargo build --release --features runtime-benchmarks
// ./target/release/node-template benchmark pallet --help
benchmarks! {
  // 在此处添加单独的基准
  create_fund {
	let b in 1 .. 1000;
	 /* 1.初始化需用到的数据 */
	let caller: T::AccountId = whitelisted_caller();
	let beneficiary_account_id: T::AccountId = frame_benchmarking::account("beneficiary_account_id",0,0);
	// 给账户打钱
	let goal_raise:BalanceOf<T>= 1000u32.into();
	let end_block:T::BlockNumber= 9u32.into();
	// BalanceOf<T> = 2u32.into()
	// let _ = T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value(100));
  }: {
	 /* 2.调用调度函数 */
	// origin: OriginFor<T>,
	// 		beneficiary_account_id: AccountIDOf<T>,
	// 		goal_raise: BalanceOf<T>,
	// 		end_block: T::BlockNumber,
	let _ = Example::<T>::create_fund(RawOrigin::Signed(caller).into(),beneficiary_account_id,
			goal_raise,end_block);
  }
  verify {
	 /* 3.进行验证(可选) */
	assert_eq!(Example::<T>::get_fund_count(), 1);

  }

  // 使用mock中的new_test_ext
  impl_benchmark_test_suite!(Example, crate::mock::new_test_ext(), crate::mock::Test);
}
