#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, sp_runtime::traits::Zero};
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		#[pallet::constant]
		/// Maximum amount added per invocation.
		type MaxAddend: Get<u32>;
		/// Frequency with which the stored value is deleted.
		type ClearFrequency: Get<Self::BlockNumber>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// 声明SingleValue每个块周期都会修改的值。
	#[pallet::storage]
	#[pallet::getter(fn single_value)]
	pub type SingleValue<T> = StorageValue<_, u32, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// SingleValue 加上新值 （初始值,添加值,最终值)
		Added(u32, u32, u32),
		//// 被清除 (删除之前的值)
		Cleared(u32),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// 值溢出
		Overflow,
	}

	/// SingleValue在块执行结束时运行ClearFrequency的函数中的每个块数都设置为 0
	/// 在#[pallet::hooks]属性on_finalize下指定此逻辑：
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_finalize(n: BlockNumberFor<T>) {
			// 如果当前块 % 清理周期 = 0 则清除
			if (n % T::ClearFrequency::get()).is_zero() {
				// 获取当前数据
				let old_value = SingleValue::<T>::get();
				// 清理
				SingleValue::<T>::put(0);
				// 发送事件
				Self::deposit_event(Event::Cleared(old_value));
			}
		}
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1_000)]
		pub fn add_value(origin: OriginFor<T>, val_to_add: u32) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			// 判断添加的值是否符合
			ensure!(val_to_add <= T::MaxAddend::get(), "value must be <= MaxAddend");

			let old_val = SingleValue::<T>::get();
			let new_val = old_val.checked_add(val_to_add).ok_or(Error::<T>::Overflow)?;
			SingleValue::<T>::put(new_val);
			Self::deposit_event(Event::Added(old_val, val_to_add, new_val));
			Ok(())
		}
	}
}
