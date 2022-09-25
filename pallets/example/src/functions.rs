use super::*;
use frame_support::inherent::Vec;
use frame_system::{
	self as system,
	offchain::{SendSignedTransaction, SendUnsignedTransaction, Signer, SubmitTransaction},
};
use lite_json::JsonValue;
use sp_runtime::{
	offchain::{
		http,
		storage::{MutateStorageError, StorageRetrievalError, StorageValueRef},
		Duration,
	},
	traits::Get,
	transaction_validity::{InvalidTransaction, TransactionValidity, ValidTransaction},
};

impl<T: Config> Pallet<T> {
	/// A helper function to fetch the price and send a raw unsigned transaction.
	pub fn fetch_price_and_send_raw_unsigned(
		block_number: T::BlockNumber,
	) -> Result<(), &'static str> {
		// Make sure we don't fetch the price if unsigned transaction is going to be rejected
		// anyway.
		let next_unsigned_at = <NextUnsignedAt<T>>::get();
		if next_unsigned_at > block_number {
			return Err("Too early to send unsigned transaction")
		}
		// Make an external HTTP request to fetch the current price.
		// Note this call will block until response is received.
		let price = Self::fetch_price().map_err(|_| "Failed to fetch price")?;
		// Received price is wrapped into a call to `submit_price_unsigned` public function of this
		// pallet. This means that the transaction, when executed, will simply call that function
		// passing `price` as an argument.
		let call = Call::submit_price_unsigned { block_number, price };
		// Now let's create a transaction out of this call and submit it to the pool.
		// Here we showcase two ways to send an unsigned transaction / unsigned payload (raw)
		//
		// By default unsigned transactions are disallowed, so we need to whitelist this case
		// by writing `UnsignedValidator`. Note that it's EXTREMELY important to carefuly
		// implement unsigned validation logic, as any mistakes can lead to opening DoS or spam
		// attack vectors. See validation logic docs for more details.
		SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into())
			.map_err(|()| "Unable to submit unsigned transaction.")?;
		Ok(())
	}
	/// A helper function to fetch the price, sign payload and send an unsigned transaction
	pub fn fetch_price_and_send_unsigned_for_all_accounts(
		block_number: T::BlockNumber,
	) -> Result<(), &'static str> {
		// Make sure we don't fetch the price if unsigned transaction is going to be rejected
		// anyway.
		let next_unsigned_at = <NextUnsignedAt<T>>::get();
		if next_unsigned_at > block_number {
			return Err("Too early to send unsigned transaction")
		}
		// Make an external HTTP request to fetch the current price.
		// Note this call will block until response is received.
		let price = Self::fetch_price().map_err(|_| "Failed to fetch price")?;
		// -- Sign using all accounts
		let transaction_results = Signer::<T, T::AuthorityId>::all_accounts()
			.send_unsigned_transaction(
				|account| PricePayload { price, block_number, public: account.public.clone() },
				|payload, signature| Call::submit_price_unsigned_with_signed_payload {
					payload,
					signature,
				},
			);
		for (_account_id, result) in transaction_results.into_iter() {
			if result.is_err() {
				return Err("Unable to submit transaction")
			}
		}

		Ok(())
	}

	/// A helper function to fetch the price, sign payload and send an unsigned transaction
	pub fn fetch_price_and_send_unsigned_for_any_account(
		block_number: T::BlockNumber,
	) -> Result<(), &'static str> {
		// Make sure we don't fetch the price if unsigned transaction is going to be rejected
		// anyway.
		let next_unsigned_at = <NextUnsignedAt<T>>::get();
		if next_unsigned_at > block_number {
			return Err("Too early to send unsigned transaction")
		}
		// Make an external HTTP request to fetch the current price.
		// Note this call will block until response is received.
		let price = Self::fetch_price().map_err(|_| "Failed to fetch price")?;
		// -- Sign using any account
		let (_, result) = Signer::<T, T::AuthorityId>::any_account()
			.send_unsigned_transaction(
				|account| PricePayload { price, block_number, public: account.public.clone() },
				|payload, signature| Call::submit_price_unsigned_with_signed_payload {
					payload,
					signature,
				},
			)
			.ok_or("No local accounts accounts available.")?;
		result.map_err(|()| "Unable to submit transaction")?;

		Ok(())
	}
	/// A helper function to fetch the price and send signed transaction.
	pub fn fetch_price_and_send_signed() -> Result<(), &'static str> {
		let signer = Signer::<T, T::AuthorityId>::all_accounts();
		if !signer.can_sign() {
			return Err(
				"No local accounts available. Consider adding one via `author_insertKey` RPC.",
			)
		}
		// Make an external HTTP request to fetch the current price.
		// Note this call will block until response is received.
		let price = Self::fetch_price().map_err(|_| "Failed to fetch price")?;
		// Using `send_signed_transaction` associated type we create and submit a transaction
		// representing the call, we've just created.
		// Submit signed will return a vector of results for all accounts that were found in the
		// local keystore with expected `KEY_TYPE`.
		let results = signer.send_signed_transaction(|_account| {
			// Received price is wrapped into a call to `submit_price` public function of this
			// pallet. This means that the transaction, when executed, will simply call that
			// function passing `price` as an argument.
			Call::submit_price { price }
		});
		for (acc, res) in &results {
			match res {
				Ok(()) => log::info!("[{:?}] Submitted price of {} cents", acc.id, price),
				Err(e) => log::error!("[{:?}] Failed to submit transaction: {:?}", acc.id, e),
			}
		}
		Ok(())
	}
	/// Fetch current price and return the result in cents.
	fn fetch_price() -> Result<u32, http::Error> {
		// We want to keep the offchain worker execution time reasonable, so we set a hard-coded
		// deadline to 2s to complete the external call.
		// You can also wait idefinitely for the response, however you may still get a timeout
		// coming from the host machine.
		let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(2_000));
		// Initiate an external HTTP GET request.
		// This is using high-level wrappers from `sp_runtime`, for the low-level calls that
		// you can find in `sp_io`. The API is trying to be similar to `reqwest`, but
		// since we are running in a custom WASM execution environment we can't simply
		// import the library here.
		let request =
			http::Request::get("https://min-api.cryptocompare.com/data/price?fsym=BTC&tsyms=USD");
		// We set the deadline for sending of the request, note that awaiting response can
		// have a separate deadline. Next we send the request, before that it's also possible
		// to alter request headers or stream body content in case of non-GET requests.
		let pending = request.deadline(deadline).send().map_err(|_| http::Error::IoError)?;
		// The request is already being processed by the host, we are free to do anything
		// else in the worker (we can send multiple concurrent requests too).
		// At some point however we probably want to check the response though,
		// so we can block current thread and wait for it to finish.
		// Note that since the request is being driven by the host, we don't have to wait
		// for the request to have it complete, we will just not read the response.
		let response = pending.try_wait(deadline).map_err(|_| http::Error::DeadlineReached)??;
		// Let's check the status code before we proceed to reading the response.
		if response.code != 200 {
			log::warn!("Unexpected status code: {}", response.code);
			return Err(http::Error::Unknown)
		}
		// Next we want to fully read the response body and collect it to a vector of bytes.
		// Note that the return object allows you to read the body in chunks as well
		// with a way to control the deadline.
		let body = response.body().collect::<Vec<u8>>();
		// Create a str slice from the body.
		let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
			log::warn!("No UTF8 body");
			http::Error::Unknown
		})?;
		let price = match Self::parse_price(body_str) {
			Some(price) => Ok(price),
			None => {
				log::warn!("Unable to extract price from the response: {:?}", body_str);
				Err(http::Error::Unknown)
			},
		}?;

		log::warn!("Got price: {} cents", price);

		Ok(price)
	}

	/// Add new price to the list.
	pub fn add_price(maybe_who: Option<T::AccountId>, price: u32) {
		log::info!("Adding to the average: {}", price);
		<Prices<T>>::mutate(|prices| {
			if prices.try_push(price).is_err() {
				prices[(price % T::MaxPrices::get()) as usize] = price;
			}
		});

		let average = Self::average_price()
			.expect("The average is not empty, because it was just mutated; qed");
		log::info!("Current average price is: {}", average);
		// here we are raising the NewPrice event
		Self::deposit_event(Event::NewPrice { price, maybe_who });
	}

	/// Parse the price from the given JSON string using `lite-json`.
	///
	/// Returns `None` when parsing failed or `Some(price in cents)` when parsing is successful.
	fn parse_price(price_str: &str) -> Option<u32> {
		let val = lite_json::parse_json(price_str);
		let price = match val.ok()? {
			JsonValue::Object(obj) => {
				let (_, v) = obj.into_iter().find(|(k, _)| k.iter().copied().eq("USD".chars()))?;
				match v {
					JsonValue::Number(number) => number,
					_ => return None,
				}
			},
			_ => return None,
		};

		let exp = price.fraction_length.saturating_sub(2);
		Some(price.integer as u32 * 100 + (price.fraction / 10_u64.pow(exp)) as u32)
	}
	/// Calculate current average price.
	pub fn average_price() -> Option<u32> {
		let prices = <Prices<T>>::get();
		if prices.is_empty() {
			None
		} else {
			Some(prices.iter().fold(0_u32, |a, b| a.saturating_add(*b)) / prices.len() as u32)
		}
	}
	/// Chooses which transaction type to send.
	/// Returns a type of transaction that should be produced in current run.
	pub fn choose_transaction_type(block_number: T::BlockNumber) -> TransactionType {
		/// A friendlier name for the error that is going to be returned in case we are in the
		/// grace period.
		const RECENTLY_SENT: () = ();
		// Start off by creating a reference to Local Storage value.
		// Since the local storage is common for all offchain workers, it's a good practice
		// to prepend your entry with the module name.
		let val = StorageValueRef::persistent(b"example_ocw::last_send");
		// The Local Storage is persisted and shared between runs of the offchain workers,
		// and offchain workers may run concurrently. We can use the `mutate` function, to
		// write a storage entry in an atomic fashion. Under the hood it uses `compare_and_set`
		// low-level method of local storage API, which means that only one worker
		// will be able to "acquire a lock" and send a transaction if multiple workers
		// happen to be executed concurrently.
		let result =
			val.mutate(|last_send: Result<Option<T::BlockNumber>, StorageRetrievalError>| {
				match last_send {
					// If we already have a value in storage and the block number is recent
					// enough we avoid sending another transaction at this time.
					Ok(Some(block)) if block_number < block + T::GracePeriod::get() =>
						Err(RECENTLY_SENT),
					// In every other case we attempt to acquire the lock and send a
					// transaction.
					_ => Ok(block_number),
				}
			});
		// The result of `mutate` call will give us a nested `Result` type.
		// The first one matches the return of the closure passed to `mutate`, i.e.
		// if we return `Err` from the closure, we get an `Err` here.
		// In case we return `Ok`, here we will have another (inner) `Result` that indicates
		// if the value has been set to the storage correctly - i.e. if it wasn't
		// written to in the meantime.
		match result {
			Ok(block_number) => {
				// let transaction_type = block_number % 3u32.into();
				let block_number: u32 = block_number.try_into().unwrap_or(0);
				let transaction_type = block_number % 3;

				match transaction_type {
					0 => TransactionType::Signed,
					1 => TransactionType::UnsignedForAny,
					2 => TransactionType::UnsignedForAll,
					_ => TransactionType::Raw,
				}
			},
			// We are in the grace period, we should not send a transaction this time.
			Err(MutateStorageError::ValueFunctionFailed(RECENTLY_SENT)) => TransactionType::None,
			// We wanted to send a transaction, but failed to write the block number (acquire a
			// lock). This indicates that another offchain worker that was running concurrently
			// most likely executed the same logic and succeeded at writing to storage.
			// Thus we don't really want to send the transaction, knowing that the other run
			// already did.
			Err(MutateStorageError::ConcurrentModification(_)) => TransactionType::None,
		}
	}
	pub fn validate_transaction_parameters(
		block_number: &T::BlockNumber,
		new_price: &u32,
	) -> TransactionValidity {
		// Now let's check if the transaction has any chance to succeed.
		let next_unsigned_at = <NextUnsignedAt<T>>::get();
		if &next_unsigned_at > block_number {
			return InvalidTransaction::Stale.into()
		}
		// Let's make sure to reject transactions from the future.
		let current_block = <system::Pallet<T>>::block_number();
		if &current_block < block_number {
			return InvalidTransaction::Future.into()
		}

		// We prioritize transactions that are more far away from current average.
		//
		// Note this doesn't make much sense when building an actual oracle, but this example
		// is here mostly to show off offchain workers capabilities, not about building an
		// oracle.
		let avg_price = Self::average_price()
			.map(|price| if &price > new_price { price - new_price } else { new_price - price })
			.unwrap_or(0);

		ValidTransaction::with_tag_prefix("ExampleOffchainWorker")
			// We set base priority to 2**20 and hope it's included before any other
			// transactions in the pool. Next we tweak the priority depending on how much
			// it differs from the current average. (the more it differs the more priority it
			// has).
			.priority(T::UnsignedPriority::get().saturating_add(avg_price as _))
			// This transaction does not require anything else to go before into the pool.
			// In theory we could require `previous_unsigned_at` transaction to go first,
			// but it's not necessary in our case.
			//.and_requires() todo!();
			// We set the `provides` tag to be the same as `next_unsigned_at`. This makes
			// sure only one transaction produced after `next_unsigned_at` will ever
			// get to the transaction pool and will end up in the block.
			// We can still have multiple transactions compete for the same "spot",
			// and the one with higher priority will replace other one in the pool.
			.and_provides(next_unsigned_at)
			// The transaction is only valid for next 5 blocks. After that it's
			// going to be revalidated by the pool.
			.longevity(5)
			// It's fine to propagate that transaction to other peers, which means it can be
			// created even by nodes that don't produce blocks.
			// Note that sometimes it's better to keep it for yourself (if you are the block
			// producer), since for instance in some schemes others may copy your solution and
			// claim a reward.
			.propagate(true)
			.build()
	}
}
