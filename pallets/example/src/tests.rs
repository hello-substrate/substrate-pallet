use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use pallet_balances::Error as BalancesError;

/// 测试初始化的数据
#[test]
fn setup() {
	new_test_ext().execute_with(|| {
		assert_eq!(System::block_number(), 0);
		assert_eq!(ExampleModule::get_fund_count(), 0);
		assert_eq!(ExampleModule::get_funds(0), None);
		assert_eq!(ExampleModule::contribute_get(0, &1), 0);
	});
}

/// 创建基金
#[test]
fn create() {
	// 创建一个基金
	// assert_ok!()
	// 基金总数是否增加
	// 基金信息是否一致
	// 押金是否抵扣成功
	// 押金是否放在基金账户中
}

// 创建基金的账户金额不够
#[test]
fn create_insufficient() {
	new_test_ext().execute_with(|| {});
}

// 捐赠
#[test]
fn contribute() {
	new_test_ext().execute_with(|| {
		// 创建基金
		// 是否还未捐赠
		// user 1 给自己创建的基金捐赠
		// 是否捐赠成功,查看余额
		// 在子存储中查看捐赠信息
		// 检查基金账户中的余额
		// 基金信息总筹集的金额
	});
}
// 捐赠基本错误
#[test]
fn contribute_error() {
	new_test_ext().execute_with(|| {
		// 不能向不存在的基金捐赠
		// 未达到最低捐赠金额
		// 创建基金
		// 移动到基金结束的块
		// 捐赠已经结束的基金
	});
}

// 提取
#[test]
fn withdraw() {
	new_test_ext().execute_with(|| {
		// 创建基金
		// 捐赠
		// 移动到基金过期的块,并捐赠失败
		// 用户可以提取所有的余额
	});
}

// 提取错误
#[test]
fn withdraw_error() {
	new_test_ext().execute_with(|| {
		// 创建基金
		// 捐赠
		// 移动到块 5
		// 结束前捐赠者不能提取
		// 移动到基金过期的块,并捐赠失败
		// 未捐赠者不能提取
		// 不能提取一个不存在的基金
	});
}

// 删除基金
#[test]
fn dissolve() {
	new_test_ext().execute_with(|| {
		// 创建基金
		// 捐赠
		// 移动到基金过期的块,并捐赠失败
		// 检查创建者的余额
		// 检查现在的基金金额(捐赠的+押金)
		// 账户7解散众筹基金，并获得剩余资金
		// 基金账户已清空
		// 解散者帐户7 获得奖励
		// 子存储移除
		// 基金存储移除
	});
}

// 删除基金错误
#[test]
fn dissolve_error() {
	new_test_ext().execute_with(|| {
		// 创建基金
		// 捐赠
		// 解散不存在的基金
		// 无法解散未结束的基金
		// 移动到基金结束的块
		// 无法解散已结束但尚未过期的基金
	});
}

// 分配基金
#[test]
fn dispense() {
	new_test_ext().execute_with(|| {
		// 创建基金
		// 捐赠
		// 移动到基金过期的块,并捐赠失败
		// 检查创建者的余额
		// 检查现在的基金金额(捐赠的+押金)
		// 账户7分配众筹基金
		// 基金账户已清空
		// 受益者账户获得基金
		// 调用者获得押金
		// 子存储移除
		// 基金存储移除
	});
}

// 分配基金错误
#[test]
fn dispense_error() {
	new_test_ext().execute_with(|| {
		// 创建基金
		// 捐赠
		// 不能分配不存在的基金id
		// 不能分配未结束的基金
		// 移动到基金过期的块,并捐赠失败
		// 无法分配已结束但未成功的基金
	});
}
