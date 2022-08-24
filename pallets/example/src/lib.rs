#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

//=========================
// 添加一个`sr25519`签名密钥的`crypto`模块, 确保`pallet`拥有一个可签名交易的账户
use sp_core::crypto::KeyTypeId;
// 一种加密密钥的标识符,pallet 底层存储中应用密钥前缀
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"demo");
pub mod crypto {
	use super::KEY_TYPE;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		MultiSignature, MultiSigner,
	};
	//
	// 声明一个账户用`sr25519`签名,`KEY_TYPE`作为标识. 仅仅声明不会创建新的账户
	// 在 hooks offchain_worker中创建账户
	app_crypto!(sr25519, KEY_TYPE);

	// Config:AuthorityId 的 offchain worker 标识实现
	pub struct TestAuthId;
	// implemented for runtime,
	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
		type RuntimeAppPublic = Public;
		type GenericPublic = sp_core::sr25519::Public;
		type GenericSignature = sp_core::sr25519::Signature;
	}
}
//==========================

/// pallet逻辑的定义, 在`runtime/src/lib.rs`通过`construct_runtime`聚合
#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		log::{error, info},
		pallet_prelude::*,
	};
	use frame_system::{
		offchain::{
			AppCrypto, CreateSignedTransaction, SendSignedTransaction, SendUnsignedTransaction,
			SignedPayload, Signer, SigningTypes, SubmitTransaction,
		},
		pallet_prelude::*,
	};

	/// pallet config trait, 所有的类型和常量`constant`在这里配置
	/// 添加 CreateSignedTransaction trait 和 AuthorityId type
	/// 告诉 runtime 此 pallet 可以创建已签名的交易
	#[pallet::config]
	pub trait Config: frame_system::Config + CreateSignedTransaction<Call<Self>> {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// offchain worker 标识
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
		/// numbers 的最大长度
		#[pallet::constant]
		type MaxNumbers: Get<u32>;
	}

	// pallet 类型的简单声明。它是我们用来实现traits和method的占位符。
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// ====================
	// 可被签名的数据结构负载,需实现 SignedPayload trait
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
	pub struct Payload<Public> {
		number: u64,
		public: Public,
	}

	impl<T: SigningTypes> frame_system::offchain::SignedPayload<T> for Payload<T::Public> {
		fn public(&self) -> T::Public {
			self.public.clone()
		}
	}
	// ====================

	#[pallet::storage]
	#[pallet::getter(fn get_numbers)]
	pub type Numbers<T: Config> = StorageValue<_, BoundedVec<u64, T::MaxNumbers>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// 接受一个新 number 时
		NewNumber(Option<T::AccountId>, u64),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		NoLocalAcctForSigning,
		OffchainSignedTxError,
		OffchainUnsignedTxError,
		OffchainUnsignedTxSignedPayloadError,
		NumbersOverflow,
		// 没有可执行的函数
		NoOffchainFunc,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		// 链下工作者入口
		fn offchain_worker(block_number: BlockNumberFor<T>) {
			info!("---Entering off-chain worker");
			// 使用 off-chain workers 的方法
			// 1. Sending signed transaction from ocw
			// 2. Sending unsigned transaction from ocw
			// 3. Sending unsigned transactions with signed payloads from ocw
			// 4. Fetching JSON via http requests in ocw
			const TRANSACTION_TYPES: usize = 4;
			let result = match block_number.try_into().unwrap_or(0) % TRANSACTION_TYPES {
				1 => Self::offchain_signed_tx(block_number),
				2 => Self::offchain_unsigned_tx(block_number),
				3 => Self::offchain_unsigned_tx_signed_payload(block_number),
				0 => Ok(()),
				_ => Err(Error::<T>::NoOffchainFunc),
			};
			if let Err(e) = result {
				error!("---offchain_worker error: {:?}", e);
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn submit_number_signed(origin: OriginFor<T>, number: u64) -> DispatchResult {
			let who = ensure_signed(origin)?;
			info!("---submit_number_signed: {}", number);
			Self::append_or_replace_number(number).map_err(|_| Error::<T>::NumbersOverflow)?;
			Self::deposit_event(Event::NewNumber(Some(who), number));
			Ok(())
		}
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn submit_number_unsigned(origin: OriginFor<T>, number: u64) -> DispatchResult {
			ensure_none(origin)?;
			info!("---submit_number_unsigned: {}", number);
			Self::append_or_replace_number(number).map_err(|_| Error::<T>::NumbersOverflow)?;
			Self::deposit_event(Event::NewNumber(None, number));
			Ok(())
		}
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn submit_number_unsigned_with_signed_payload(
			origin: OriginFor<T>,
			payload: Payload<T::Public>,
			_signature: T::Signature,
		) -> DispatchResult {
			ensure_none(origin)?;
			// 不需要在此验证 已在 validate_unsigned 方法中验证了
			let Payload { number, public } = payload;
			info!("---submit_number_unsigned_with_signed_payload: ({}, {:?})", number, public);
			Self::append_or_replace_number(number).map_err(|_| Error::<T>::NumbersOverflow)?;
			Self::deposit_event(Event::NewNumber(None, number));
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// numbers 列表中添加一个新的 block_number,如果超出长度,则删除第一个
		fn append_or_replace_number(number: u64) -> Result<(), ()> {
			Numbers::<T>::try_mutate(|vec| -> Result<(), ()> {
				if vec.len() == T::MaxNumbers::get() as usize {
					vec.remove(0);
				}
				vec.try_push(number)
			})
		}

		fn offchain_signed_tx(block_number: T::BlockNumber) -> Result<(), Error<T>> {
			// 获取任何一个可用的密钥进行签名。
			// all_accounts() 是所有的账户都执行一次交易 返回 Vec<(Account<T>, Result<(), ()>)>
			let signer = Signer::<T, T::AuthorityId>::any_account();
			// 如果有多个键，并且我们想要精确定位它，`with_filter（）`可以被链接，
			// signer.with_filter(vec![0xf2.into(), 0xf1.into()]);
			// 将当前区块编号转换为数字并在链上提交
			let number: u64 = block_number.try_into().unwrap_or(0);
			// `result` is in the type of `Option<(Account<T>, Result<(), ()>)>`. It is:
			//   - `None`: no account is available for sending transaction
			//   - `Some((account, Err(())))`: error occured when sending the transaction
			//   - `Some((account, Ok(())))`: transaction is successfully sent
			// if let Some(res) = res { // 遍历已签名的 accounts 返回执行成功的 account 和结果
			// 	return Some((account, res))
			// }
			// 发送链上签名的交易,最终调用已 runtime 声明的
			// frame_system::offchain::CreateSignedTransaction::create_transaction()
			let result = signer.send_signed_transaction(|_acct|
				// This is the on-chain function
				Call::submit_number_signed { number });
			// 如果签名发送失败，则显示错误
			match result {
				Some((acc, res)) => res.map_err(|_| {
					error!("---submit_number_signed fail call: check error and offchain_signed_tx account: {:?}", acc.id);
					Error::<T>::OffchainSignedTxError
				}),
				None => {
					// The case of `None`: no account is available for sending
					error!(
						"submit_number_signed: Add a account to ocw. No local account available."
					);
					Err(Error::<T>::NoLocalAcctForSigning)
				},
			}
		}
		fn offchain_unsigned_tx(block_number: T::BlockNumber) -> Result<(), Error<T>> {
			let number: u64 = block_number.try_into().unwrap_or(0);
			let call = Call::submit_number_unsigned { number };
			// `submit_unsigned_transaction` returns a type of `Result<(), ()>`
			// 提供在链上直接提交签名和未签名交易的能力
			SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into())
				.map_err(|_| Error::<T>::OffchainUnsignedTxError)
		}
		// 提交一个未签名交易并带有签名负载
		// 此操作不会向签名者账户收取交易费用
		fn offchain_unsigned_tx_signed_payload(
			block_number: T::BlockNumber,
		) -> Result<(), Error<T>> {
			// 获取签名者用来签名负载
			let signer = Signer::<T, T::AuthorityId>::any_account();
			let number: u64 = block_number.try_into().unwrap_or(0);
			// `send_unsigned_transaction` is returning a type of `Option<(Account<T>, Result<(),
			// ()>)>`.   Similar to `send_signed_transaction`, they account for:
			//   - `None`: no account is available for sending transaction
			//   - `Some((account, Ok(())))`: transaction is successfully sent
			//   - `Some((account, Err(())))`: error occured when sending the transaction
			let result = signer.send_unsigned_transaction(
				// 准备和返回 Payload
				|acct| Payload { number, public: acct.public.clone() },
				|payload, signature| Call::submit_number_unsigned_with_signed_payload {
					payload,
					signature,
				},
			);
			match result {
				Some((_, res)) => res.map_err(|_| {
					error!("---submit_number_unsigned_with_signed_payload fail call: check error");
					Error::<T>::OffchainUnsignedTxSignedPayloadError
				}),
				None => {
					error!("---submit_number_unsigned_with_signed_payload: Add a account to ocw. No local account available.");
					Err(Error::<T>::NoLocalAcctForSigning)
				},
			}
		}
	}

	// 默认情况下，所有未签名的交易都会在 Substrate 中被拒绝。
	// 要使 Substrate 能够接受某些未签名的交易，
	// 您必须为托盘实现 ValidateUnsigned trait。
	#[pallet::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;
		fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
			//Call冒号后面就是具体的提交未签名交易的函数，
			//需要对此交易进行验证
			let valid_tx = |provide| {
				ValidTransaction::with_tag_prefix("ExampleModule")
					.priority(TransactionPriority::MAX)
					.and_provides([&provide]) // 添加一个 TransactionTag
					.longevity(5) //设置事务的寿命。此处设置 5 blockNumber. 默认情况下，交易将被视为永久有效
					.propagate(true) //设置传播标志。如果交易不打算向对等方传播，则设置为 false
					.build()
			};
			match call {
				Call::submit_number_unsigned { number: _ } =>
					valid_tx(b"submit_number_unsigned".to_vec()),
				Call::submit_number_unsigned_with_signed_payload { ref payload, ref signature } => {
					if !SignedPayload::<T>::verify::<T::AuthorityId>(payload, signature.clone()) {
						return InvalidTransaction::BadProof.into()
					}
					valid_tx(b"submit_number_unsigned_with_signed_payload".to_vec())
				},
				_ => InvalidTransaction::Call.into(),
			}
		}
	}
}
