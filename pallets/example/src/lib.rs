#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::PalletId;
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

/// pallet逻辑的定义, 在`runtime/src/lib.rs`通过`construct_runtime`聚合
#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		pallet_prelude::*,
		sp_runtime::traits::Zero,
		traits::{Currency, ExistenceRequirement, ReservableCurrency, WithdrawReasons},
	};
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::AccountIdConversion;

	// 类型别名与结构体声明处
	/// pallet identifier
	const PALLET_ID: PalletId = PalletId(*b"ex/cfund");

	/// 基金ID
	type FundID = u32;
	/// 账户ID
	type AccountIDOf<T> = <T as frame_system::Config>::AccountId;
	/// 余额
	type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIDOf<T>>>::Balance;
	/// 基金信息
	type FundInfoOf<T> =
		FundInfo<AccountIDOf<T>, BalanceOf<T>, <T as frame_system::Config>::BlockNumber>;

	/// 基金信息
	#[derive(Clone, Encode, Decode, Eq, PartialEq, Default, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	#[cfg_attr(feature = "std", derive(Debug))]
	pub struct FundInfo<AccountID, Balance, BlockNumber> {
		/// 接受基金的账户ID
		beneficiary_account_id: AccountID,
		/// 押金金额
		deposit: Balance,
		/// 筹集的总金额
		total_raised: Balance,
		/// 截止日期(block number)
		end_block: BlockNumber,
		/// 众筹的目标金额
		goal_raise: Balance,
	}

	/// pallet config trait, 所有的类型和常量`constant`在这里配置
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// 货币
		type Currency: ReservableCurrency<Self::AccountId>;
		/// 众筹发起者的保证金(押金)
		type SubmitterDeposit: Get<BalanceOf<Self>>;
		/// 最小捐款金额
		type MinContribution: Get<BalanceOf<Self>>;
		/// 众筹失败后可清理的时间限制(以块为单位),在这之前可以提前基金,超过时间限制则会失去
		type FailTakePeriod: Get<Self::BlockNumber>;
	}

	// pallet 类型的简单声明。它是我们用来实现traits和method的占位符。
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// 存储
	/// 所有的基金信息
	#[pallet::storage]
	#[pallet::getter(fn get_funds)]
	pub type Funds<T: Config> = StorageMap<_, Blake2_128Concat, FundID, FundInfoOf<T>>;

	/// 基金总数
	#[pallet::storage]
	#[pallet::getter(fn get_fund_count)]
	pub type FundCount<T> = StorageValue<_, FundID, ValueQuery>;

	// 事件
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// 创建基金 [fund_id, BlockNumber]
		Created(FundID, T::BlockNumber),
	}

	// 错误
	#[pallet::error]
	pub enum Error<T> {
		/// 结束太早
		EndTooEarly,
		/// 基金总数溢出
		FundCountOverflow,
	}

	// 调度函数
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// 创建一个基金
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_fund(
			origin: OriginFor<T>,
			beneficiary_account_id: AccountIDOf<T>,
			goal_raise: BalanceOf<T>,
			end_block: T::BlockNumber,
		) -> DispatchResult {
			let caller = ensure_signed(origin)?;
			// 获取现在区块
			let now_block = <frame_system::Pallet<T>>::block_number();
			// 检查结束块是否早与当前块
			ensure!(end_block > now_block, Error::<T>::EndTooEarly);
			// 提取创建押金
			let submitter_deposit = T::SubmitterDeposit::get();
			let imbalance = T::Currency::withdraw(
				&caller,
				submitter_deposit,
				WithdrawReasons::TRANSFER,
				ExistenceRequirement::AllowDeath,
			)?;
			// 基金 id 自增1
			let fund_id = FundCount::<T>::get();
			let fund_id = fund_id.checked_add(1).ok_or(Error::<T>::FundCountOverflow)?;
			// 创建账户不需要支付手续费,不使用`transfer`
			T::Currency::resolve_creating(&Self::gen_fund_account_id(fund_id), imbalance);
			// 基金信息入库
			Funds::<T>::insert(
				fund_id,
				FundInfo {
					beneficiary_account_id,
					deposit: submitter_deposit,
					total_raised: Zero::zero(),
					end_block,
					goal_raise,
				},
			);
			// 发送事件
			Self::deposit_event(Event::Created(fund_id, now_block));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn gen_fund_account_id(id: FundID) -> T::AccountId {
			PALLET_ID.into_sub_account_truncating(id)
		}
	}
}
