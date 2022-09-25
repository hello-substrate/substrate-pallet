#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use sp_core::crypto::KeyTypeId;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"ocwd");
pub mod crypto {
	use super::KEY_TYPE;
	use sp_core::sr25519::Signature as Sr25519Signature;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		traits::Verify,
		MultiSignature, MultiSigner,
	};
	app_crypto!(sr25519, KEY_TYPE);

	pub struct OcwAuthId;

	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for OcwAuthId {
		type RuntimeAppPublic = Public;
		type GenericPublic = sp_core::sr25519::Public;
		type GenericSignature = sp_core::sr25519::Signature;
	}

	impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
		for OcwAuthId
	{
		type RuntimeAppPublic = Public;
		type GenericPublic = sp_core::sr25519::Public;
		type GenericSignature = sp_core::sr25519::Signature;
	}
}

/// pallet逻辑的定义, 在`runtime/src/lib.rs`通过`construct_runtime`聚合
#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		inherent::Vec,
		pallet_prelude::*,
		traits::{Currency, ReservableCurrency},
	};
	use frame_system::{
		offchain::{AppCrypto, CreateSignedTransaction, SendSignedTransaction, Signer},
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
		NoAccountCanSign,
		NoLocalAcctForSigning,
		OffchainSignedTxError,
		NoOffchainFunc,
	}
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(block_number: T::BlockNumber) {
			log::info!("-- Hello World from offchain workers!: {:?}", block_number);
			let payload = sp_std::vec![1, 2, 3, 4, 5, 6, 7, 8];
			let number: u32 = block_number.try_into().unwrap_or(0);
			let result = match number % 2 {
				0 => Self::send_signed_tx_all(payload),
				1 => Self::send_signed_tx_any(payload),
				_ => Err(Error::<T>::NoOffchainFunc),
			};
			if let Err(e) = result {
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
			pub fn submit_signed_tx(
				origin: OriginFor<T>,
				payload: Vec<u8>,
			) -> DispatchResultWithPostInfo {
				let _who = ensure_signed(origin)?;
				log::info!("--in submit_signed_tx call: {:?}", payload);
				Ok(().into())
			}
	}
	impl<T: Config> Pallet<T> {
		/// all_accounts() 所有的账户都执行一次交易
		fn send_signed_tx_all(payload: Vec<u8>) -> Result<(), Error<T>> {
			// 在 sp_keystore 中寻找账户
			let signer = Signer::<T, T::AuthorityId>::all_accounts();
			if !signer.can_sign() {
				return Err(Error::<T>::NoAccountCanSign)
			}
			// 发送链上签名的交易,最终调用已 runtime 声明的
			// frame_system::offchain::CreateSignedTransaction::create_transaction()
			let results = signer.send_signed_transaction(|_account| Call::submit_signed_tx {
				payload: payload.clone(),
			});
			// Vec<(Account<T>, Result<(), ()>)>;
			for (acc, res) in &results {
				match res {
					Ok(()) => log::info!(
						"--send_signed_tx_all: [{:?}] Submitted data:{:?}",
						acc.id,
						payload
					),
					Err(e) => log::error!(
						"--send_signed_tx_all: [{:?}] Failed to submit transaction: {:?}",
						acc.id,
						e
					),
				}
			}
			Ok(())
		}
		/// any_account() 使用任何可用密钥进行签名
		fn send_signed_tx_any(payload: Vec<u8>) -> Result<(), Error<T>> {
			let signer = Signer::<T, T::AuthorityId>::any_account();
			if !signer.can_sign() {
				return Err(Error::<T>::NoAccountCanSign)
			}
			let result = signer.send_signed_transaction(|_account| Call::submit_signed_tx {
				payload: payload.clone(),
			});
			// `result` is in the type of `Option<(Account<T>, Result<(), ()>)>`. It is:
			//   - `None`: no account is available for sending transaction
			//   - `Some((account, Err(())))`: error occured when sending the transaction
			//   - `Some((account, Ok(())))`: transaction is successfully sent
			// 如果签名发送失败，则显示错误
			match result {
				Some((acc, res)) => match res {
					Ok(()) => {
						log::info!(
							"--send_signed_tx_any: [{:?}] Submitted data:{:?}",
							acc.id,
							payload
						);
						Ok(())
					},
					Err(e) => {
						log::error!(
							"--send_signed_tx_any: [{:?}] Failed to submit transaction: {:?}",
							acc.id,
							e
						);
						Err(Error::<T>::OffchainSignedTxError)
					},
				},
				None => {
					// The case of `None`: no account is available for sending
					log::info!(
						"--send_signed_tx_any: Add a account to ocw. No local account available."
					);
					Err(Error::<T>::NoLocalAcctForSigning)
				},
			}
		}
	}
}
