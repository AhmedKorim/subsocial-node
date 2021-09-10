
//! Autogenerated weights for pallet_claims
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-09-10, STEPS: [20, ], REPEAT: 10, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/subsocial-node
// benchmark
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// Compiled
// --pallet
// pallet_claims
// --extrinsic
// *
// --steps
// 20
// --repeat
// 10
// --raw
// --output
// ./


#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_claims.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_claims::WeightInfo for WeightInfo<T> {
	fn claim_tokens() -> Weight {
		(96_330_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn set_rewards_sender() -> Weight {
		(25_381_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn add_eligible_accounts() -> Weight {
		(64_998_166_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
