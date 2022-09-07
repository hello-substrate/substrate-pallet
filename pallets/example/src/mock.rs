use crate as pallet_example;
use frame_support::traits::{ConstU128, ConstU16, ConstU64, OnFinalize, OnInitialize};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		ExampleModule: pallet_example::{Pallet, Call, Storage, Event<T>},
	}
);

pub type Balance = u128;

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = Balance;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ConstU128<1>;
	type AccountStore = System;
	type WeightInfo = ();
}

impl pallet_example::Config for Test {
	type Event = Event;
	type CustomType = u32;
	type Amount = ConstU128<500>;
	type Currency = Balances;
}

// 根据我们想要的模型构建一个创世存储键值存储
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	GenesisConfig {
		// 初始化账户余额
		balances: BalancesConfig { balances: vec![(1, 1000), (2, 2000), (3, 3000), (4, 4000)] },
		..Default::default()
	}
	.assimilate_storage(&mut t) // use sp_runtime::BuildStorage;
	.unwrap();
	// pallet_balances::GenesisConfig::<Test> {
	// 	// 初始化账户余额
	// 	balances: vec![(1, 1000), (2, 2000), (3, 3000), (4, 4000)],
	// }
	// .assimilate_storage(&mut t)
	// .unwrap();
	let ext = sp_io::TestExternalities::new(t);
	// ext.execute_with(|| System::set_block_number(1));
	ext
}

// 获取所有的事件
fn events() -> Vec<Event> {
	let evt = System::events().into_iter().map(|evt| evt.event).collect::<Vec<_>>();
	System::reset_events();
	evt
}

// 跳转到指定块 先进后出执行顺序
pub fn run_to_block(n: u64) {
	while System::block_number() < n {
		ExampleModule::on_finalize(System::block_number());
		Balances::on_finalize(System::block_number());
		System::on_finalize(System::block_number());
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		Balances::on_initialize(System::block_number());
		ExampleModule::on_initialize(System::block_number());
	}
}
