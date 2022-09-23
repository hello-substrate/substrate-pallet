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
	use sp_core::offchain::Duration;
	use sp_runtime::{
		offchain::{
			storage::StorageValueRef,
			storage_lock::{BlockAndTime, StorageLock},
		},
		traits::BlockNumberProvider,
	};
	use std::fmt;

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
	// We are fetching information from the github public API about
	// organization`substrate-developer-hub`.
	const HTTP_REMOTE_REQUEST: &str = "https://api.github.com/orgs/substrate-developer-hub";
	const HTTP_HEADER_USER_AGENT: &str = "sunwenming";

	const REQUEST_TIMEOUT_PERIOD: u64 = 3000; // in milli-seconds
	const LOCK_BLOCK_EXPIRATION: u32 = 3; // in block number
	const LOCK_TIMEOUT_EXPIRATION: u64 = REQUEST_TIMEOUT_PERIOD + 1000; // in milli-seconds
	const ONCHAIN_TX_KEY: &[u8] = b"pallet-example::storage::tx";

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

	// http 返回数据解析的 GithubInfo
	#[derive(serde::Deserialize, Encode, Decode, Default)]
	struct GithubInfo {
		// Specify our own deserializing function to convert JSON string to vector of bytes
		#[serde(deserialize_with = "de_string_to_bytes")]
		login: Vec<u8>,
		#[serde(deserialize_with = "de_string_to_bytes")]
		blog: Vec<u8>,
		public_repos: u32,
	}
	pub fn de_string_to_bytes<'de, D>(de: D) -> Result<Vec<u8>, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let s: &str = serde::Deserialize::deserialize(de)?;
		Ok(s.as_bytes().to_vec())
	}
	// GithubInfo 实现自定义 debug
	impl fmt::Debug for GithubInfo {
		// `fmt` converts the vector of bytes inside the struct back to string for
		//   more friendly display.
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			write!(
				f,
				"{{ login: {}, blog: {}, public_repos: {} }}",
				std::str::from_utf8(&self.login).map_err(|_| fmt::Error)?,
				std::str::from_utf8(&self.blog).map_err(|_| fmt::Error)?,
				&self.public_repos
			)
		}
	}

	/// off-chain index 传输的数据封装.
	#[derive(Debug, serde::Deserialize, Encode, Decode, Default)]
	struct IndexingData(Vec<u8>, u64);
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
		HttpFetchingError,
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
				0 => Self::fetch_github_info(),
				_ => Err(Error::<T>::NoOffchainFunc),
			};
			if let Err(e) = result {
				error!("---offchain_worker error: {:?}", e);
			}
			// 获取 off-chain indexing value. ocw 链下存储方法一样
			let key = Self::derived_index_key(block_number);
			let index_storage_info = StorageValueRef::persistent(&key);
			if let Ok(Some(data)) = index_storage_info.get::<IndexingData>() {
				info!(
					"off-chain indexing data: {:?}, {:?}",
					std::str::from_utf8(&data.0).unwrap_or("error"),
					data.1
				);
			} else {
				info!("no off-chain indexing data retrieved")
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

			// 链上设置链下存储数据
			let key = Self::derived_index_key(frame_system::Pallet::<T>::block_number());
			let data = IndexingData(b"submit_number_signed".to_vec(), number);
			sp_io::offchain_index::set(&key, &data.encode());

			Self::deposit_event(Event::NewNumber(Some(who), number));
			Ok(())
		}
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn submit_number_unsigned(origin: OriginFor<T>, number: u64) -> DispatchResult {
			ensure_none(origin)?;
			info!("---submit_number_unsigned: {}", number);
			Self::append_or_replace_number(number).map_err(|_| Error::<T>::NumbersOverflow)?;

			let key = Self::derived_index_key(frame_system::Pallet::<T>::block_number());
			let data = IndexingData(b"submit_number_unsigned".to_vec(), number);
			sp_io::offchain_index::set(&key, &data.encode());

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

			let key = Self::derived_index_key(frame_system::Pallet::<T>::block_number());
			let data = IndexingData(b"submit_number_unsigned_with_signed_payload".to_vec(), number);
			sp_io::offchain_index::set(&key, &data.encode());

			Self::deposit_event(Event::NewNumber(None, number));
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		// 根据 block_number 生产 off-chain index key
		fn derived_index_key(block_number: T::BlockNumber) -> Vec<u8> {
			block_number.using_encoded(|encoded_block_number| {
				ONCHAIN_TX_KEY
					.clone()
					.into_iter()
					.chain(b"/".into_iter()) //将两个迭代器链接在一起创建新的迭代器
					.chain(encoded_block_number)
					.copied() //复制所有元素到新创建新的迭代器中。这很有用,当您有一个基于 &T
					// 的迭代器时,但您需要一个基于 T 的迭代器.
					.collect::<Vec<u8>>() //获取任何可迭代的内容,并将其转换为相关的集合
			})
		}

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
		/// 检查我们以前是否获取过github信息。
		/// 如果是，我们可以使用在链下存储中缓存的数据
		/// 存储在链外工作存储器“存储”中。
		/// 如果没有，我们将获取远程信息并将信息写入存储器以备将来检索。
		fn fetch_github_info() -> Result<(), Error<T>> {
			// 创建一个本地存储
			// 本地存储对于所有 ocw 都是共有的, 最好加一个 pallet 名称区分
			let s_info = StorageValueRef::persistent(b"pallet-example::gh-info");
			// `mutate` func 以原子的方式执行写操作
			if let Ok(Some(gh_info)) = s_info.get::<GithubInfo>() {
				// 如果有数据 直接返回
				info!("cached gh-info: {:?}", gh_info);
				return Ok(())
			}

			// ocw 可以多次访问 off-chain storage, 在操作之前需要加锁
			// 4种方式加速:
			//   1) `new` - lock with default time and block exipration
			//   2) `with_deadline` - lock with default block but custom time expiration
			//   3) `with_block_deadline` - lock with default time but custom block expiration
			//   4) `with_block_and_time_deadline` - lock with custom time and block expiration
			// Here we choose the most custom one for demonstration purpose.
			let mut lock = StorageLock::<BlockAndTime<Self>>::with_block_and_time_deadline(
				b"pallet-example::lock",
				LOCK_BLOCK_EXPIRATION,
				sp_core::offchain::Duration::from_millis(LOCK_TIMEOUT_EXPIRATION),
			);
			// 尝试获取锁, 如果失败. 上一次 ocw fetch_n_parse 内部正在执行
			// 若未获取锁,则直接返回
			// 当 _guard 变量超出范围时，锁被释放。
			if let Ok(_guard) = lock.try_lock() {
				match Self::fetch_n_parse() {
					Ok(gh_info) => {
						s_info.set(&gh_info);
					},
					Err(err) => return Err(err),
				}
			}
			Ok(())
		}

		// 从远程获取数据并反序列化为 GithubInfo 结构
		fn fetch_n_parse() -> Result<GithubInfo, Error<T>> {
			let res_bytes = Self::fetch_from_remote().map_err(|e| {
				error!("---fetch_from_remote err {:?}", e);
				Error::<T>::HttpFetchingError
			})?;
			let res_str =
				std::str::from_utf8(&res_bytes).map_err(|_| Error::<T>::HttpFetchingError)?;
			info!("---{}", res_str);
			// 将 json 字符串转换成 GithubInfo 结构体
			let gh_info =
				serde_json::from_str(res_str).map_err(|_| Error::<T>::HttpFetchingError)?;
			Ok(gh_info)
		}

		/// 使用 offchain::http 获取服务器的数据, 将 json 转换为 vec 字节返回
		fn fetch_from_remote() -> Result<Vec<u8>, Error<T>> {
			info!("---sending request to: {}", HTTP_REMOTE_REQUEST);
			// 通过 url 初始化一个 get 请求
			let request = sp_runtime::offchain::http::Request::get(HTTP_REMOTE_REQUEST);
			// 限制 ocw 执行时间的合理性, 限制请求 3s 内结束
			let timeout =
				sp_io::offchain::timestamp().add(Duration::from_millis(REQUEST_TIMEOUT_PERIOD));

			// For github API request, we also need to specify `user-agent` in http request header.
			let pending = request
				.add_header("User-Agent", HTTP_HEADER_USER_AGENT)
				.deadline(timeout) // Setting the timeout time
				.send() // Sending the request out by the host
				.map_err(|_| Error::<T>::HttpFetchingError)?;
			// http request 是异步的, 在这等待执行结束
			// The returning value here is a `Result` of `Result`, so we are unwrapping it twice by
			// two `?`
			let response = pending
				.try_wait(timeout)
				.map_err(|_| Error::<T>::HttpFetchingError)?
				.map_err(|_| Error::<T>::HttpFetchingError)?;

			if response.code != 200 {
				error!("---Unexpected http request status code: {}", response.code);
				return Err(Error::<T>::HttpFetchingError)
			}
			// 接下来，我们完全读取响应正文并将其收集到字节向量中
			let body = response.body();
			info!("---body: {:?}", body);
			Ok(response.body().collect::<Vec<u8>>())
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
	// BlockNumberProvider
	impl<T: Config> BlockNumberProvider for Pallet<T> {
		type BlockNumber = T::BlockNumber;
		fn current_block_number() -> Self::BlockNumber {
			<frame_system::Pallet<T>>::block_number()
		}
	}
}
