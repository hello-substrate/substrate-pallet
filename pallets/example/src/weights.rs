
//! Autogenerated weights for `pallet_example`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-08-19, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `macbookdeMacBook-Pro.local`, CPU: `<UNKNOWN>`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/node-template
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet_example
// --extrinsic
// *
// --steps
// 20
// --repeat
// 10
// --output
// pallets/example/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_example`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_example::WeightInfo for WeightInfo<T> {
	// Storage: ExampleModule FundCount (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	// Storage: ExampleModule Funds (r:0 w:1)
	fn create_fund() -> Weight {
		(14_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}
