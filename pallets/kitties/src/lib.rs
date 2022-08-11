#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::*,
		traits::{tokens::ExistenceRequirement, Currency, Randomness},
	};
	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;
	use sp_io::hashing::blake2_128;

	#[cfg(feature = "std")]
	use frame_support::serde::{Deserialize, Serialize};

	// 定义账户ID与余额 类型

	// kitty的信息
	// 唯一标识 dna
	// 价格 price 可为 None 代表未出售
	// 性别 gender
	// 小猫的所有者 owner
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	// kitty的性别枚举
	// Male Female
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		// 货币处理 Currency

		// 每人最多拥有小猫数量

		// 处理随机类型 Randomness
	}

	// Errors.
	#[pallet::error]
	pub enum Error<T> {
		// 新增小猫但总数量(u64)溢出
		// 超出最多拥有小猫数量
		// 小猫不能自己买自己的
		// 小猫不能自己转给自己
		// 小猫是否存在
		// 小猫是不是自己的
		// 小猫是否上架
		// 价格是否超过上架价格
		// 账户是否有足够的钱购买小猫
	}

	// Events.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// 小猫创建成功
		// 设置价格成功
		// 成功转让小猫
		// 购买小猫成功
	}

	// Storage items.
	#[pallet::storage]
	#[pallet::getter(fn kitty_cnt)]
	// 小猫的总数量
	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	// 存放小猫数据的集合
	#[pallet::storage]
	#[pallet::getter(fn kitties_owned)]
	// 存放每人已拥有的小猫dna BoundedVec

	// Our pallet's genesis configuration.
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		// 小猫集合 (拥有者,dna,gender)
	}

	// Required to implement default for GenesisConfig.
	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> GenesisConfig<T> {}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			// 从创世配置中 mint 小猫
		}
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new unique kitty.
		#[pallet::weight(0)]
		pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult {
			Ok(())
		}

		/// Breed a Kitty.
		/// Breed two Kitties to give birth to a new Kitty.
		#[pallet::weight(0)]
		pub fn breed_kitty(
			origin: OriginFor<T>,
			parent_1: [u8; 16],
			parent_2: [u8; 16],
		) -> DispatchResult {
			Ok(())
		}

		/// Directly transfer a kitty to another recipient.
		#[pallet::weight(0)]
		pub fn transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			kitty_id: [u8; 16],
		) -> DispatchResult {
			Ok(())
		}

		/// Buy a saleable Kitty. The bid price provided from the buyer has to be equal or higher
		/// than the ask price from the seller.
		#[pallet::weight(0)]
		pub fn buy_kitty(
			origin: OriginFor<T>,
			kitty_id: [u8; 16],
			bid_price: BalanceOf<T>,
		) -> DispatchResult {
			Ok(())
		}

		/// Set the price for a Kitty.
		#[pallet::weight(0)]
		pub fn set_price(
			origin: OriginFor<T>,
			kitty_id: [u8; 16],
			new_price: Option<BalanceOf<T>>,
		) -> DispatchResult {
			Ok(())
		}
	}

	//** Our helper functions.**//

	impl<T: Config> Pallet<T> {
		// Generates and returns DNA and Gender
		fn gen_dna() -> ([u8; 16], Gender) {}

		// Picks from existing DNA.
		fn mutate_dna_fragment(dna_fragment1: u8, dna_fragment2: u8, new_dna_fragment: u8) -> u8 {}

		// Generates a new Kitty using existing Kitties.
		pub fn breed_dna(parent1: &[u8; 16], parent2: &[u8; 16]) -> ([u8; 16], Gender) {}

		// Helper to mint a Kitty.
		pub fn mint(
			owner: &T::AccountId,
			dna: [u8; 16],
			gender: Gender,
		) -> Result<[u8; 16], Error<T>> {
			Ok(dna)
		}

		// Check whether Kitty is owner by the breeder.
		pub fn check_owner(kitty_dna: &[u8; 16], breeder: &T::AccountId) -> bool {}

		// Update storage to transfer kitty.
		pub fn transfer_kitty_to(kitty_id: &[u8; 16], to: &T::AccountId) -> Result<(), Error<T>> {
			Ok(())
		}
	}
}
