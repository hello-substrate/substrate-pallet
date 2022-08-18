use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

/// 测试初始化的数据
#[test]
fn basic_setup_works() {
	new_test_ext().execute_with(|| {
		assert_eq!(System::block_number(), 0);
		assert_eq!(ExampleModule::get_fund_count(), 0);
		assert_eq!(ExampleModule::get_funds(0), None);
		assert_eq!(ExampleModule::contribute_get(0, &1), 0);
	});
}


