#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

/// pallet逻辑的定义, 在`runtime/src/lib.rs`通过`construct_runtime`聚合
#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::*,
		traits::{Currency, ReservableCurrency},
	};
	use frame_system::{
		offchain::{SendTransactionTypes, SubmitTransaction},
		pallet_prelude::*,
	};
	use sp_std::fmt::Debug;

	// ----------------------------------------------------------------
	/// 定义余额类型
	type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
	// ----------------------------------------------------------------

	/// pallet config trait, 所有的类型和常量`constant`在这里配置
	#[pallet::config]
	pub trait Config: frame_system::Config + SendTransactionTypes<Call<Self>> {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		// 自定义类型
		type CustomType: Parameter
		+ Member
		+ sp_runtime::traits::AtLeast32BitUnsigned
		+ codec::Codec
		+ Default
		+ Copy
		+ MaybeSerializeDeserialize
		+ Debug
		+ MaxEncodedLen
		+ TypeInfo;
		/// 金额
		#[pallet::constant]
		type Amount: Get<BalanceOf<Self>>;
		/// 可质押的货币
		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
	}

	// pallet 类型的简单声明。它是我们用来实现traits和method的占位符。
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SomethingStored(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
		OffchainUnsignedTxError,
	}
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(block_number: T::BlockNumber) {
			log::info!("-- Hello World from offchain workers!: {:?}", block_number);
			let res = Self::send_unsigned_tx(block_number);
			if let Err(e) = res {
				log::info!("---offchain_worker error: {:?}", e);
			}
			log::info!("Leave from offchain workers!: {:?}", block_number);
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// Update storage.
			<Something<T>>::put(something);
			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
		#[pallet::weight(0)]
		pub fn submit_unsigned_tx(origin: OriginFor<T>, number: u64) -> DispatchResultWithPostInfo {
			ensure_none(origin)?;
			log::info!("--in submit_unsigned_tx call: {:?}", number);
			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		fn send_unsigned_tx(block_number: T::BlockNumber) -> Result<(), Error<T>> {
			let number: u64 = block_number.try_into().unwrap_or(0);
			let call = Call::submit_unsigned_tx { number };
			SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into())
				.map_err(|_| Error::<T>::OffchainUnsignedTxError)
		}
	}

	// 默认情况下，所有未签名的交易都会在 Substrate 中被拒绝。
	// 要使 Substrate 能够接受某些未签名的交易，
	// 您必须为托盘实现 ValidateUnsigned trait。
	#[pallet::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;
		fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
			let valid_tx = |provide| {
				// 参数参考 https://github.com/paritytech/substrate/blob/master/frame/examples/offchain-worker/src/lib.rs
				ValidTransaction::with_tag_prefix("ExampleModule")
					.priority(TransactionPriority::MAX)
					.and_provides([&provide]) // 添加一个 TransactionTag
					.longevity(5) //交易的寿命。此处设置 5 blockNumber. 默认情况下，交易将被视为永久有效
					.propagate(true) //是否传播。如果交易不打算向其他节点传播，则设置为 false
					.build()
			};
			match call {
				//Call冒号后面就是具体的提交未签名交易的函数，
				Call::submit_unsigned_tx { number: _ } => valid_tx(b"submit_unsigned_tx".to_vec()),
				_ => InvalidTransaction::Call.into(),
			}
		}
	}
}
