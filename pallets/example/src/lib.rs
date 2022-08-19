#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
// weights
pub mod weights;
pub use weights::*;

use frame_support::{traits::Currency, PalletId};

// 类型别名与结构体声明处
/// pallet identifier
const PALLET_ID: PalletId = PalletId(*b"ex/cfund");

/// 基金ID
pub type FundID = u32;
/// 账户ID
type AccountIDOf<T> = <T as frame_system::Config>::AccountId;
/// 余额
type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIDOf<T>>>::Balance;
/// 基金信息
type FundInfoOf<T> =
	FundInfo<AccountIDOf<T>, BalanceOf<T>, <T as frame_system::Config>::BlockNumber>;

/// pallet逻辑的定义, 在`runtime/src/lib.rs`通过`construct_runtime`聚合
#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		ensure,
		inherent::Vec,
		pallet_prelude::*,
		sp_runtime::traits::Zero,
		storage::child,
		traits::{Currency, ExistenceRequirement, ReservableCurrency, WithdrawReasons},
	};
	use frame_system::{ensure_signed, pallet_prelude::*};
	use sp_core::Hasher;
	use sp_runtime::traits::{AccountIdConversion, Saturating};

	/// 基金信息
	#[derive(Clone, Encode, Decode, Eq, PartialEq, Default, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	#[cfg_attr(feature = "std", derive(Debug))]
	pub struct FundInfo<AccountID, Balance, BlockNumber> {
		/// 接受基金的账户ID
		pub beneficiary_account_id: AccountID,
		/// 押金金额
		pub deposit: Balance,
		/// 筹集的总金额
		pub total_raised: Balance,
		/// 截止日期(block number)
		pub end_block: BlockNumber,
		/// 众筹的目标金额
		pub goal_raise: Balance,
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
		type ExpirePeriod: Get<Self::BlockNumber>;
		type WeightInfo: WeightInfo;
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
		/// 捐赠
		Contributed(FundID, T::AccountId, BalanceOf<T>, T::BlockNumber),
		/// 提取
		Withdrew(FundID, T::AccountId, BalanceOf<T>, T::BlockNumber),
		// 清理过期的基金
		Dissolved(FundID, T::AccountId, T::BlockNumber),
		// 分配基金奖励
		Dispensed(FundID, T::AccountId, T::BlockNumber),
	}

	// 错误
	#[pallet::error]
	pub enum Error<T> {
		/// 结束太早
		EndTooEarly,
		/// 基金总数溢出
		FundCountOverflow,
		/// 捐赠金额太少
		ContributeTooSmall,
		/// 未找到基金信息
		FundNotFound,
		/// 基金已经结束
		FundIsEnd,
		/// 基金未结束
		FundNotEnd,
		/// 捐赠者才可提取
		NoContribute,
		/// 基金未失效
		FundNotInvalid,
		/// 筹集失败
		UnsuccessfulFund,
	}

	// 调度函数
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// 创建一个基金
		#[pallet::weight(T::WeightInfo::create_fund())]
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
			FundCount::<T>::put(fund_id);
			// 创建账户不需要支付手续费,不使用`transfer`
			T::Currency::resolve_creating(&Self::get_fund_account_id(fund_id), imbalance);
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
		/// 捐赠基金
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn contribute(
			origin: OriginFor<T>,
			fund_id: FundID,
			value: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// 判断捐赠金额达标
			ensure!(value >= T::MinContribution::get(), Error::<T>::ContributeTooSmall);
			// 获取基金信息
			let mut fund_info = Funds::<T>::get(fund_id).ok_or(Error::<T>::FundNotFound)?;
			// 确保基金尚未结束
			let now_block = frame_system::Pallet::<T>::block_number();
			ensure!(fund_info.end_block >= now_block, Error::<T>::FundIsEnd);
			// 捐赠逻辑
			T::Currency::transfer(
				&who,
				&Self::get_fund_account_id(fund_id),
				value,
				ExistenceRequirement::AllowDeath,
			)?;
			// 更新基金信息
			fund_info.total_raised = fund_info.total_raised.saturating_add(value);
			Funds::<T>::insert(fund_id, &fund_info);
			// 更新捐赠者的捐赠金额
			let balance = Self::contribute_get(fund_id, &who);
			let balance = balance.saturating_add(value);
			Self::contribute_put(fund_id, &who, balance);
			// 发送事件
			Self::deposit_event(Event::Contributed(fund_id, who, value, now_block));
			Ok(())
		}

		/// 基金捐赠者提取
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn withdraw(origin: OriginFor<T>, fund_id: FundID) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// 确保基金未结束
			let fund_info = Funds::<T>::get(fund_id).ok_or(Error::<T>::FundNotFound)?;
			let now_block = frame_system::Pallet::<T>::block_number();
			ensure!(now_block > fund_info.end_block, Error::<T>::FundNotEnd);
			// 确保捐赠的金额大于0
			let balance = Self::contribute_get(fund_id, &who);
			ensure!(balance > Zero::zero(), Error::<T>::NoContribute);
			// 将基金发给受益者,不收取手续费
			let _ = T::Currency::resolve_into_existing(
				&who,
				T::Currency::withdraw(
					&Self::get_fund_account_id(fund_id),
					balance,
					WithdrawReasons::TRANSFER,
					ExistenceRequirement::AllowDeath,
				)?,
			);
			// 删除捐赠信息
			Self::contribute_kill(fund_id, &who);
			// 更新 total_raised
			fund_info.total_raised.saturating_sub(balance);
			Funds::<T>::insert(fund_id, &fund_info);
			Self::deposit_event(Event::Withdrew(fund_id, who, balance, now_block));
			Ok(())
		}

		/// 在基金过期之后，人人都可解散基金,并获取押金奖励
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn dissolve(origin: OriginFor<T>, fund_id: FundID) -> DispatchResult {
			let caller = ensure_signed(origin)?;
			// 获取基金信息,判断基金是否过期
			let fund_info = Funds::<T>::get(fund_id).ok_or(Error::<T>::FundNotFound)?;
			let now_block = frame_system::Pallet::<T>::block_number();
			ensure!(
				now_block >= fund_info.end_block + T::ExpirePeriod::get(),
				Error::<T>::FundNotInvalid
			);
			// 将基金剩余余额奖励给调用者
			let _ = T::Currency::resolve_creating(
				&caller,
				T::Currency::withdraw(
					&Self::get_fund_account_id(fund_id),
					fund_info.deposit + fund_info.total_raised,
					WithdrawReasons::TRANSFER,
					ExistenceRequirement::AllowDeath,
				)?,
			);
			// 清理存储数据
			Funds::<T>::remove(fund_id);
			// 删除众筹(一次写即可删除众筹的信息, 因为使用了 child tree)
			Self::crowdfund_kill(fund_id);
			Self::deposit_event(Event::<T>::Dissolved(fund_id, caller, now_block));
			Ok(())
		}

		/// 基金成功筹集.
		/// 分配捐赠的基金给受益者.
		/// 分配押金奖励给调用者清理众筹存储空间
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn dispense(origin: OriginFor<T>, fund_id: FundID) -> DispatchResult {
			let caller = ensure_signed(origin)?;
			// 获取基金信息,判断基金是否结束
			let fund_info = Funds::<T>::get(fund_id).ok_or(Error::<T>::FundNotFound)?;
			let now_block = frame_system::Pallet::<T>::block_number();
			ensure!(now_block >= fund_info.end_block, Error::<T>::FundNotEnd);
			// 确保基金众筹成功
			ensure!(fund_info.total_raised >= fund_info.goal_raise, Error::<T>::UnsuccessfulFund);
			let fund_account = Self::get_fund_account_id(fund_id);
			// 受益者分配捐赠的基金
			T::Currency::resolve_creating(
				&fund_info.beneficiary_account_id,
				T::Currency::withdraw(
					&fund_account,
					fund_info.total_raised,
					WithdrawReasons::TRANSFER,
					ExistenceRequirement::AllowDeath,
				)?,
			);
			// 调用者分配押金
			T::Currency::resolve_creating(
				&caller,
				T::Currency::withdraw(
					&fund_account,
					fund_info.deposit,
					WithdrawReasons::TRANSFER,
					ExistenceRequirement::AllowDeath,
				)?,
			);
			// 清理存储数据
			Funds::<T>::remove(fund_id);
			// 删除众筹(一次写即可删除众筹的信息, 因为使用了 child tree)
			Self::crowdfund_kill(fund_id);
			Self::deposit_event(Event::<T>::Dispensed(fund_id, caller, now_block));
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// 基金的账户ID
		pub fn get_fund_account_id(id: FundID) -> T::AccountId {
			PALLET_ID.into_sub_account_truncating(id)
		}
		/// 实例化子存储
		pub fn get_child_from_id(id: FundID) -> child::ChildInfo {
			let mut buf = Vec::new();
			buf.extend_from_slice(b"crowdfund");
			buf.extend_from_slice(&id.to_le_bytes());
			child::ChildInfo::new_default(T::Hashing::hash(&buf[..]).as_ref())
		}
		/// 在 child trie 中 记录捐赠金额
		pub fn contribute_put(id: FundID, who: &T::AccountId, balance: BalanceOf<T>) {
			let child_info = Self::get_child_from_id(id);
			// 将 who 转换为切片，然后用它调用给定的闭包。
			who.using_encoded(|who| child::put(&child_info, who, &balance))
		}
		/// 获取捐赠金额
		pub fn contribute_get(id: FundID, who: &T::AccountId) -> BalanceOf<T> {
			let child_info = Self::get_child_from_id(id);
			who.using_encoded(|who| child::get_or_default(&child_info, who))
		}
		/// 删除捐赠金额
		pub fn contribute_kill(id: FundID, who: &T::AccountId) {
			let child_info = Self::get_child_from_id(id);
			who.using_encoded(|who| child::kill(&child_info, who));
		}
		/// 删除捐赠的信息
		pub fn crowdfund_kill(id: FundID) {
			let child_info = Self::get_child_from_id(id);
			let _ = child::clear_storage(&child_info, None, None);
		}
	}
}
