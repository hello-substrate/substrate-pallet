use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

// 常用功能
// mock.rs use frame_system as system;
// System::set_block_number(1)

// System::assert_has_event(Event::Balances(crate::Event::Deposit { who: 1, amount: 42 }));
// System::assert_last_event(Event::Balances(crate::Event::Deposit { who: 1, amount: 10 }));
// assert_eq!(
// 	events(),
// 	[
// 		Event::System(system::Event::NewAccount { account: 1 }),
// 		Event::Balances(crate::Event::Endowed { account: 1, free_balance: 100 }),
// 		Event::Balances(crate::Event::BalanceSet { who: 1, free: 100, reserved: 0 }),
// 	]
// );



#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(ExampleModule::do_something(Origin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(ExampleModule::something(), Some(42));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(ExampleModule::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
	});
}
