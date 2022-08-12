#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, Blake2_128Concat};
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Claims<T: Config> =
		StorageMap<_, Blake2_128Concat, T::Hash, (T::AccountId, T::BlockNumber)>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SomethingStored(u32, T::AccountId),
		// 文件证明创建
		ClaimCreated { who: T::AccountId, claim: T::Hash },
		// 文件证明取消
		ClaimRevoked { who: T::AccountId, claim: T::Hash },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// wneji
		StorageOverflow,
		/// 文件已被声明
		AlreadyClaimed,
		/// 文件未声明
		NoSuchClaim,
		/// 文件不属于自己
		NoClaimOwner,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// 创建文件的声明
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_claim(origin: OriginFor<T>, file_hash: T::Hash) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// 判断文件是否已存在
			log::info!("{}", Claims::<T>::contains_key(file_hash));
			ensure!(!Claims::<T>::contains_key(file_hash), Error::<T>::AlreadyClaimed);
			// 获取当前区块
			let cur_block_number = frame_system::Pallet::<T>::block_number();
			// 声明文件
			Claims::<T>::insert(file_hash, (&who, cur_block_number));
			// Emit an event.
			Self::deposit_event(Event::ClaimCreated { who, claim: file_hash });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// 取消文件的声明
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn revoke_claim(origin: OriginFor<T>, file_hash: T::Hash) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// 判断这个文件是否已经声明
			let (owner, _) = Claims::<T>::get(file_hash).ok_or(Error::<T>::NoSuchClaim)?;
			// 判断这个文件是否自己的?
			ensure!(who == owner, Error::<T>::NoClaimOwner);
			// 删除
			Claims::<T>::remove(file_hash);
			Self::deposit_event(Event::ClaimRevoked { who, claim: file_hash });
			Ok(())
		}
	}
}
