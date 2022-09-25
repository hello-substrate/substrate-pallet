#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use sp_core::crypto::KeyTypeId;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod functions;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

// --------------------------------

pub enum TransactionType {
	Signed,
	UnsignedForAny,
	UnsignedForAll,
	Raw,
	None,
}

/// Defines application identifier for crypto keys of this module.
///
/// Every module that deals with signatures needs to declare its unique identifier for
/// its crypto keys.
/// When offchain worker is signing transactions it's going to request keys of type
/// `KeyTypeId` from the keystore and use the ones it finds to sign the transaction.
/// The keys can be inserted manually via RPC (see `author_insertKey`).
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"exam");

/// Based on the above `KeyTypeId` we need to generate a pallet-specific crypto type wrappers.
/// We can use from supported crypto kinds (`sr25519`, `ed25519` and `ecdsa`) and augment
/// the types with this pallet-specific identifier.
/// 固定写法
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

	// implemented for mock runtime in test
	impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
	for OcwAuthId
	{
		type RuntimeAppPublic = Public;
		type GenericPublic = sp_core::sr25519::Public;
		type GenericSignature = sp_core::sr25519::Signature;
	}
}
// --------------------------------

/// pallet逻辑的定义, 在`runtime/src/lib.rs`通过`construct_runtime`聚合
#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		pallet_prelude::*,
		traits::{Currency, ReservableCurrency},
	};
	use frame_system::{
		self as system,
		offchain::{AppCrypto, CreateSignedTransaction, SignedPayload, SigningTypes},
		pallet_prelude::*,
	};
	use sp_std::fmt::Debug;

	// ----------------------------------------------------------------
	/// 定义余额类型
	type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	/// Payload used by this example crate to hold price
	/// data required to submit a transaction.
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
	pub struct PricePayload<Public, BlockNumber> {
		pub block_number: BlockNumber,
		pub price: u32,
		pub public: Public,
	}
	impl<T: SigningTypes> SignedPayload<T> for PricePayload<T::Public, T::BlockNumber> {
		fn public(&self) -> T::Public {
			self.public.clone()
		}
	}
	// ----------------------------------------------------------------

	/// pallet config trait, 所有的类型和常量`constant`在这里配置
	#[pallet::config]
	pub trait Config: frame_system::Config + CreateSignedTransaction<Call<Self>> {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// The identifier type for an offchain worker.
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
		// Configuration parameters

		/// A grace period after we send transaction.
		///
		/// To avoid sending too many transactions, we only attempt to send one
		/// every `GRACE_PERIOD` blocks. We use Local Storage to coordinate
		/// sending between distinct runs of this offchain worker.
		#[pallet::constant]
		type GracePeriod: Get<Self::BlockNumber>;
		/// Number of blocks of cooldown after unsigned transaction is included.
		///
		/// This ensures that we only accept unsigned transactions once, every `UnsignedInterval`
		/// blocks.
		#[pallet::constant]
		type UnsignedInterval: Get<Self::BlockNumber>;
		/// A configuration for base priority of unsigned transactions.
		///
		/// This is exposed so that it can be tuned for particular runtime, when
		/// multiple pallets send unsigned transactions.
		#[pallet::constant]
		type UnsignedPriority: Get<TransactionPriority>;
		/// Maximum number of prices.
		#[pallet::constant]
		type MaxPrices: Get<u32>;

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

	/// A vector of recently submitted prices.
	///
	/// This is used to calculate average price, should have bounded size.
	#[pallet::storage]
	#[pallet::getter(fn prices)]
	pub(super) type Prices<T: Config> = StorageValue<_, BoundedVec<u32, T::MaxPrices>, ValueQuery>;

	/// Defines the block when next unsigned transaction will be accepted.
	///
	/// To prevent spam of unsigned (and unpayed!) transactions on the network,
	/// we only allow one transaction every `T::UnsignedInterval` blocks.
	/// This storage entry defines when new transaction is going to be accepted.
	#[pallet::storage]
	#[pallet::getter(fn next_unsigned_at)]
	pub(super) type NextUnsignedAt<T: Config> = StorageValue<_, T::BlockNumber, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SomethingStored(u32, T::AccountId),
		/// Event generated when new price is accepted to contribute to the average.
		NewPrice {
			price: u32,
			maybe_who: Option<T::AccountId>,
		},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		/// Offchain Worker entry point.
		///
		/// By implementing `fn offchain_worker` you declare a new offchain worker.
		/// This function will be called when the node is fully synced and a new best block is
		/// succesfuly imported.
		/// Note that it's not guaranteed for offchain workers to run on EVERY block, there might
		/// be cases where some blocks are skipped, or for some the worker runs twice (re-orgs),
		/// so the code should be able to handle that.
		/// You can use `Local Storage` API to coordinate runs of the worker.
		fn offchain_worker(block_number: T::BlockNumber) {
			// Note that having logs compiled to WASM may cause the size of the blob to increase
			// significantly. You can use `RuntimeDebug` custom derive to hide details of the types
			// in WASM. The `sp-api` crate also provides a feature `disable-logging` to disable
			// all logging and thus, remove any logging from the WASM.
			// #[derive(Debug)] -> #[derive(RuntimeDebug)] test by --execution Native
			log::info!("Hello World from offchain workers!");
			// Since off-chain workers are just part of the runtime code, they have direct access
			// to the storage and other included pallets.
			//
			// We can easily import `frame_system` and retrieve a block hash of the parent block.
			let parent_hash = <system::Pallet<T>>::block_hash(block_number - 1u32.into());
			log::debug!("Current block: {:?} (parent hash: {:?})", block_number, parent_hash);
			// Here we call a helper function to calculate current average price.
			// This function reads storage entries of the current state.
			let average: Option<u32> = Self::average_price();
			log::debug!("Current price: {:?}", average);
			// For this example we are going to send both signed and unsigned transactions
			// depending on the block number.
			let should_send = Self::choose_transaction_type(block_number);
			let res = match should_send {
				TransactionType::Signed => Self::fetch_price_and_send_signed(),
				TransactionType::UnsignedForAny =>
					Self::fetch_price_and_send_unsigned_for_any_account(block_number),
				TransactionType::UnsignedForAll =>
					Self::fetch_price_and_send_unsigned_for_all_accounts(block_number),
				TransactionType::Raw => Self::fetch_price_and_send_raw_unsigned(block_number),
				TransactionType::None => Ok(()),
			};
			if let Err(e) = res {
				log::error!("Error: {}", e);
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Submit new price to the list.
		///
		/// This method is a public function of the module and can be called from within
		/// a transaction. It appends given `price` to current list of prices.
		/// In our example the `offchain worker` will create, sign & submit a transaction that
		/// calls this function passing the price.
		///
		/// The transaction needs to be signed (see `ensure_signed`) check, so that the caller
		/// pays a fee to execute it.
		/// This makes sure that it's not easy (or rather cheap) to attack the chain by submitting
		/// excesive transactions, but note that it doesn't ensure the price oracle is actually
		/// working and receives (and provides) meaningful data.
		/// This example is not focused on correctness of the oracle itself, but rather its
		/// purpose is to showcase offchain worker capabilities.
		#[pallet::weight(0)]
		pub fn submit_price(origin: OriginFor<T>, price: u32) -> DispatchResultWithPostInfo {
			// Retrieve sender of the transaction.
			let who = ensure_signed(origin)?;
			// Add the price to the on-chain list.
			Self::add_price(Some(who), price);
			Ok(().into())
		}

		#[pallet::weight(0)]
		pub fn submit_price_unsigned_with_signed_payload(
			origin: OriginFor<T>,
			payload: PricePayload<T::Public, T::BlockNumber>,
			_signature: T::Signature,
		) -> DispatchResultWithPostInfo {
			// This ensures that the function can only be called via unsigned transaction.
			ensure_none(origin)?;
			// Add the price to the on-chain list, but mark it as coming from an empty address.
			Self::add_price(None, payload.price);
			// now increment the block number at which we expect next unsigned transaction.
			let current_block = <system::Pallet<T>>::block_number();
			<NextUnsignedAt<T>>::put(current_block + T::UnsignedInterval::get());
			Ok(().into())
		}
		#[pallet::weight(0)]
		pub fn submit_price_unsigned(
			origin: OriginFor<T>,
			_block_number: T::BlockNumber,
			price: u32,
		) -> DispatchResultWithPostInfo {
			// This ensures that the function can only be called via unsigned transaction.
			ensure_none(origin)?;
			// Add the price to the on-chain list, but mark it as coming from an empty address.
			Self::add_price(None, price);
			// now increment the block number at which we expect next unsigned transaction.
			let current_block = <system::Pallet<T>>::block_number();
			<NextUnsignedAt<T>>::put(current_block + T::UnsignedInterval::get());
			Ok(().into())
		}
	}

	#[pallet::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;
		/// Validate unsigned call to this module.
		///
		/// By default unsigned transactions are disallowed, but implementing the validator
		/// here we make sure that some particular calls (the ones produced by offchain worker)
		/// are being whitelisted and marked as valid.
		fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
			match call {
				Call::submit_price_unsigned_with_signed_payload { ref payload, ref signature } => {
					let signature_valid =
						SignedPayload::<T>::verify::<T::AuthorityId>(payload, signature.clone());
					if !signature_valid {
						return InvalidTransaction::BadProof.into()
					}
					Self::validate_transaction_parameters(&payload.block_number, &payload.price)
				},
				Call::submit_price_unsigned { block_number, price: new_price } =>
					Self::validate_transaction_parameters(block_number, new_price),
				_ => InvalidTransaction::Call.into(),
			}
		}
	}
}
