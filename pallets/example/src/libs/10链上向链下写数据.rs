#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use sp_core::crypto::KeyTypeId;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

/// pallet逻辑的定义, 在`runtime/src/lib.rs`通过`construct_runtime`聚合
#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		pallet_prelude::*,
		traits::{Currency, ReservableCurrency},
	};
	use frame_system::{
		offchain::{
			AppCrypto, CreateSignedTransaction, SendUnsignedTransaction, SignedPayload, Signer,
			SigningTypes,
		},
		pallet_prelude::*,
	};
	use sp_std::fmt::Debug;

	// ----------------------------------------------------------------
	/// 定义余额类型
	type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
	/// off-chain index 传输的数据封装.
	#[derive(Debug, Encode, Decode, Default)]
	struct IndexingData(Vec<u8>, u64);
	// ----------------------------------------------------------------

	/// pallet config trait, 所有的类型和常量`constant`在这里配置
	#[pallet::config]
	pub trait Config: frame_system::Config + CreateSignedTransaction<Call<Self>> {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;

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
		AccountNotSign,
		NoLocalAcctForSigning,
		OffchainSignedTxError,
		NoOffchainFunc,
	}
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(block_number: T::BlockNumber) {
			log::info!("-- Hello World from offchain workers!: {:?}", block_number);
			Self::print_offchian_index_data();
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

		/// 链上设置链下存储数据
		#[pallet::weight(0)]
		pub fn set_offchain_storage(
			origin: OriginFor<T>,
			payload: u32,
		) -> DispatchResultWithPostInfo {
			let _who = ensure_signed(origin)?;
			let key = Self::derived_index_key(frame_system::Pallet::<T>::block_number());
			let data = IndexingData(b"set_offchain_storage".to_vec(), number);
			sp_io::offchain_index::set(&key, &data.encode());
			Ok(().into())
		}
	}
	impl<T: Config> Pallet<T> {
		// 根据 block_number 生产 off-chain key
		fn derived_index_key(block_number: T::BlockNumber) -> Vec<u8> {
			block_number.using_encoded(|encoded_block_number| {
				b"pallet-example::storage::"
					.iter()
					.chain(encoded_block_number) //将两个迭代器链接在一起创建新的迭代器
					.copied() //复制所有元素到新创建新的迭代器中。这很有用,当您有一个基于 &T
					// 的迭代器时,但您需要一个基于 T 的迭代器.
					.collect::<Vec<u8>>()
			})
		}

		fn print_offchian_index_data() {
			// 获取 off-chain indexing 数据.
			let key = Self::derived_key(block_number);
			let index_storage_info = StorageValueRef::persistent(&key);
			if let Ok(Some(data)) = index_storage_info.get::<IndexingData>() {
				log::info!(
					"--off-chain indexing data: {:?}, {:?}",
					sp_std::str::from_utf8(&data.0).unwrap_or("error"),
					data.1
				);
			} else {
				log::info!("--no off-chain indexing data retrieved")
			}
		}
	}
}
