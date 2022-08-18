use crate::{mock::*, Error, FundInfo};
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
	new_test_ext().execute_with(|| {
		let fund_id = 1;
		// 创建一个基金
		assert_ok!(ExampleModule::create_fund(Origin::signed(1), 2, 1000, 9));
		// 基金总数是否增加
		assert_eq!(ExampleModule::get_fund_count(), 0);
		// 基金信息是否一致
		let fund_info = FundInfo {
			beneficiary_account_id: 2,
			deposit: 1,
			total_raised: 0,
			end_block: 9,
			goal_raise: 1000,
		};
		assert_eq!(ExampleModule::get_funds(fund_id), Some(fund_info));
		// 押金是否抵扣成功
		assert_eq!(Balances::free_balance(1), 999);
		// 押金是否放在基金账户中
		assert_eq!(Balances::free_balance(ExampleModule::get_fund_account_id(fund_id)), 1);
	});
}

// 创建基金的账户金额不够
#[test]
fn create_insufficient() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			ExampleModule::create_fund(Origin::signed(7), 2, 1000, 9),
			BalancesError::<Test>::InsufficientBalance
		);
	});
}

// 捐赠
#[test]
fn contribute() {
	new_test_ext().execute_with(|| {
		let fund_id = 1;
		// 创建基金
		assert_ok!(ExampleModule::create_fund(Origin::signed(1), 2, 1000, 9));
		// 押金是否抵扣成功
		assert_eq!(Balances::free_balance(1), 999);
		// 押金是否放在基金账户中
		assert_eq!(Balances::free_balance(ExampleModule::get_fund_account_id(fund_id)), 1);
		// user 1 还未捐赠
		assert_eq!(Balances::free_balance(ExampleModule::contribute_get(fund_id, &1)), 0);
		// user 1 给自己创建的基金捐赠
		assert_ok!(ExampleModule::contribute(Origin::signed(1), fund_id, 49));
		// 是否捐赠成功,查看余额
		assert_eq!(Balances::free_balance(1), 950);
		// 在子存储中查看捐赠金额
		assert_eq!(ExampleModule::contribute_get(fund_id, &1), 49);
		// 检查基金账户中的余额
		assert_eq!(Balances::free_balance(ExampleModule::get_fund_account_id(fund_id)), 50);
		// 基金信息中总筹集的金额
		assert_eq!(ExampleModule::get_funds(fund_id).unwrap().total_raised, 49);
	});
}
// 捐赠基本错误
#[test]
fn contribute_error() {
	new_test_ext().execute_with(|| {
		// 不能向不存在的基金捐赠
		assert_noop!(
			ExampleModule::contribute(Origin::signed(1), 12, 1000),
			Error::<Test>::FundNotFound
		);
		// 未达到最低捐赠金额
		assert_noop!(
			ExampleModule::contribute(Origin::signed(1), 12, 9),
			Error::<Test>::ContributeTooSmall
		);
		// 创建基金
		let fund_id = 1;
		assert_ok!(ExampleModule::create_fund(Origin::signed(1), 2, 1000, 9));
		assert_ok!(ExampleModule::contribute(Origin::signed(1), fund_id, 100));
		// 移动到基金结束的块
		run_to_block(10);
		// 捐赠已经结束的基金
		assert_noop!(
			ExampleModule::contribute(Origin::signed(1), fund_id, 1000),
			Error::<Test>::FundIsEnd
		);
	});
}

// 提取
#[test]
fn withdraw() {
	new_test_ext().execute_with(|| {
		// 创建基金
		let fund_id = 1;
		assert_ok!(ExampleModule::create_fund(Origin::signed(1), 2, 1000, 9));
		// 捐赠
		assert_ok!(ExampleModule::contribute(Origin::signed(1), fund_id, 100));
		assert_ok!(ExampleModule::contribute(Origin::signed(2), fund_id, 200));
		assert_ok!(ExampleModule::contribute(Origin::signed(3), fund_id, 300));
		// 移动到基金过期的块
		run_to_block(10);
		// 用户可以提取所有的余额
		assert_eq!(Balances::free_balance(1), 899);
		assert_ok!(ExampleModule::withdraw(Origin::signed(1), fund_id));
		assert_eq!(Balances::free_balance(1), 999);
		assert_ok!(ExampleModule::withdraw(Origin::signed(2), fund_id));
		assert_eq!(Balances::free_balance(2), 2000);
		assert_ok!(ExampleModule::withdraw(Origin::signed(3), fund_id));
		assert_eq!(Balances::free_balance(3), 3000);
	});
}

// 提取错误
#[test]
fn withdraw_error() {
	new_test_ext().execute_with(|| {
		// 创建基金
		let fund_id = 1;
		assert_ok!(ExampleModule::create_fund(Origin::signed(1), 2, 1000, 9));
		// 捐赠
		assert_ok!(ExampleModule::contribute(Origin::signed(1), fund_id, 100));
		assert_ok!(ExampleModule::contribute(Origin::signed(2), fund_id, 200));
		assert_ok!(ExampleModule::contribute(Origin::signed(3), fund_id, 300));
		// 移动到块 5
		run_to_block(5);
		// 结束前捐赠者不能提取
		assert_noop!(
			ExampleModule::withdraw(Origin::signed(1), fund_id),
			Error::<Test>::FundNotEnd
		);
		// 移动到基金过期的块
		run_to_block(10);
		// 未捐赠者不能提取
		assert_noop!(
			ExampleModule::withdraw(Origin::signed(7), fund_id),
			Error::<Test>::NoContribute
		);
		// 不能提取一个不存在的基金
		assert_noop!(
			ExampleModule::withdraw(Origin::signed(7), fund_id + 1),
			Error::<Test>::FundNotFound
		);
	});
}

// 删除基金
#[test]
fn dissolve() {
	new_test_ext().execute_with(|| {
		// 创建基金
		let fund_id = 1;
		assert_ok!(ExampleModule::create_fund(Origin::signed(1), 2, 1000, 9));
		// 捐赠
		assert_ok!(ExampleModule::contribute(Origin::signed(1), fund_id, 100));
		assert_ok!(ExampleModule::contribute(Origin::signed(2), fund_id, 200));
		assert_ok!(ExampleModule::contribute(Origin::signed(3), fund_id, 300));
		// 移动到基金过期的块
		run_to_block(50);
		// 捐赠的金额
		assert_eq!(ExampleModule::contribute_get(fund_id, &1), 100);
		// 检查创建者的余额
		assert_eq!(Balances::free_balance(1), 899);
		// 检查现在的基金金额(捐赠的+押金)
		assert_eq!(Balances::free_balance(ExampleModule::get_fund_account_id(fund_id)), 601);
		// 账户7解散众筹基金，并获得剩余资金
		assert_ok!(ExampleModule::dissolve(Origin::signed(7), fund_id));
		// 基金账户已清空
		assert_eq!(Balances::free_balance(ExampleModule::get_fund_account_id(fund_id)), 0);
		// 解散者帐户7 获得奖励
		assert_eq!(Balances::free_balance(7), 601);
		// 子存储移除和基金账户清零
		assert_eq!(Balances::free_balance(ExampleModule::get_fund_account_id(fund_id)), 0);
		assert_eq!(ExampleModule::contribute_get(fund_id, &0), 0);
		// 基金存储移除
		assert_eq!(ExampleModule::get_funds(fund_id), None);
	});
}

// 删除基金错误
#[test]
fn dissolve_error() {
	new_test_ext().execute_with(|| {
		// 创建基金
		let fund_id = 1;
		assert_ok!(ExampleModule::create_fund(Origin::signed(1), 2, 1000, 9));
		// 捐赠
		assert_ok!(ExampleModule::contribute(Origin::signed(1), fund_id, 100));
		assert_ok!(ExampleModule::contribute(Origin::signed(2), fund_id, 200));
		assert_ok!(ExampleModule::contribute(Origin::signed(3), fund_id, 300));
		// 解散不存在的基金
		assert_noop!(
			ExampleModule::dissolve(Origin::signed(1), fund_id + 1),
			Error::<Test>::FundNotFound
		);
		// 无法解散未结束的基金
		assert_noop!(
			ExampleModule::dissolve(Origin::signed(1), fund_id),
			Error::<Test>::FundNotInvalid
		);
		// 移动到基金结束的块
		run_to_block(10);
		// 无法解散已结束但尚未过期的基金
		assert_noop!(
			ExampleModule::dissolve(Origin::signed(1), fund_id),
			Error::<Test>::FundNotInvalid
		);
	});
}

// 分配基金
#[test]
fn dispense() {
	new_test_ext().execute_with(|| {
		// 创建基金
		let fund_id = 1;
		assert_ok!(ExampleModule::create_fund(Origin::signed(1), 22, 1000, 9));
		// 捐赠
		assert_ok!(ExampleModule::contribute(Origin::signed(1), fund_id, 100));
		assert_ok!(ExampleModule::contribute(Origin::signed(2), fund_id, 200));
		assert_ok!(ExampleModule::contribute(Origin::signed(3), fund_id, 300));
		assert_ok!(ExampleModule::contribute(Origin::signed(3), fund_id, 400));
		// 移动到基金过期的块
		run_to_block(50);
		// 检查创建者的余额
		assert_eq!(Balances::free_balance(1), 899);
		// 检查现在的基金金额(捐赠的+押金)
		assert_eq!(Balances::free_balance(ExampleModule::get_fund_account_id(fund_id)), 1001);
		// 账户7分配众筹基金
		assert_ok!(ExampleModule::dispense(Origin::signed(7), fund_id));
		// 基金账户已清空
		assert_eq!(Balances::free_balance(ExampleModule::get_fund_account_id(fund_id)), 0);
		// 受益者账户获得基金
		assert_eq!(Balances::free_balance(22), 1000);
		// 调用者获得押金
		assert_eq!(Balances::free_balance(7), 1);
		// 子存储移除和基金账户清零
		assert_eq!(ExampleModule::contribute_get(fund_id, &0), 0);
		// 基金存储移除
		assert_eq!(ExampleModule::get_funds(fund_id), None);
	});
}

// 分配基金错误
#[test]
fn dispense_error() {
	new_test_ext().execute_with(|| {
		// 创建基金
		let fund_id = 1;
		assert_ok!(ExampleModule::create_fund(Origin::signed(1), 2, 1000, 9));
		// 捐赠
		assert_ok!(ExampleModule::contribute(Origin::signed(1), fund_id, 100));
		assert_ok!(ExampleModule::contribute(Origin::signed(2), fund_id, 200));
		assert_ok!(ExampleModule::contribute(Origin::signed(3), fund_id, 300));
		// 不能分配不存在的基金id
		assert_noop!(
			ExampleModule::dispense(Origin::signed(1), fund_id + 1),
			Error::<Test>::FundNotFound
		);
		// 不能分配未结束的基金
		assert_noop!(
			ExampleModule::dispense(Origin::signed(1), fund_id),
			Error::<Test>::FundNotEnd
		);
		// 移动到基金过期的块
		run_to_block(50);
		// 无法分配已结束但未成功的基金
		assert_noop!(
			ExampleModule::dispense(Origin::signed(1), fund_id),
			Error::<Test>::UnsuccessfulFund
		);
	});
}
