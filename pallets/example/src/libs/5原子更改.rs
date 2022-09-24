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
		inherent::Vec,
		pallet_prelude::*,
		traits::{Currency, ReservableCurrency},
	};
	use frame_system::pallet_prelude::*;
	use sp_runtime::offchain::storage::{MutateStorageError, StorageRetrievalError, StorageValueRef};
	use sp_std::fmt::Debug;

	// ----------------------------------------------------------------
	/// 定义余额类型
	type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
	// ----------------------------------------------------------------

	/// pallet config trait, 所有的类型和常量`constant`在这里配置
	#[pallet::config]
	pub trait Config: frame_system::Config {
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
	}
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(block_number: T::BlockNumber) {
			log::info!("-- Hello World from offchain workers!: {:?}", block_number);
			// 奇数块向 Local Storage 写数据，偶数块读取数据，并检查
			if block_number % 2u32.into() != sp_runtime::traits::Zero::zero() {
				let key = Self::get_key(block_number);
				let val_ref = StorageValueRef::persistent(&key);
				// get a local random value
				let random_slice = sp_io::offchain::random_seed();
				// get current timestamp
				let timestamp = sp_io::offchain::timestamp().unix_millis();
				let value = (random_slice, timestamp);
				log::info!("-- in odd block,value: {:?}", value);

				struct StateError;

				//  write or mutate tuple content to key
				let res = val_ref.mutate(|val: Result<Option<([u8;32], u64)>, StorageRetrievalError>| -> Result<_, StateError> {
					match val {
						Ok(Some(_)) => Ok(value),
						_ => Ok(value),
					}
				});

				match res {
					Ok(value) => {
						log::info!("in odd block, mutate successfully: {:?}", value);
					},
					Err(MutateStorageError::ValueFunctionFailed(_)) => (),
					Err(MutateStorageError::ConcurrentModification(_)) => (),
				}
			} else {
				let key = Self::get_key(block_number - 1u32.into());
				let mut val_ref = StorageValueRef::persistent(&key);

				if let Ok(Some(value)) = val_ref.get::<([u8; 32], u64)>() {
					log::info!("-- in even block,value: {:?}", value);
					// clear
					val_ref.clear();
				}
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
	}

	impl<T: Config> Pallet<T> {
		fn get_key(block_number: T::BlockNumber) -> Vec<u8> {
			block_number.using_encoded(|val| {
				b"pallet-example::storage::".iter().chain(val).copied().collect::<Vec<u8>>()
			})
		}
	}
}
