//! Autogenerated weights for `pallet_process_validation`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-04-08, STEPS: `50`, REPEAT: `1000`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-172-31-24-170.eu-west-2.compute.internal`, CPU: `AMD EPYC 7571`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: 1024

// Executed Command:
// ./target/production/sqnc-node
// benchmark
// pallet
// --pallet
// pallet_process_validation
// --extrinsic
// *
// --repeat
// 1000
// --output
// ./weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use sqnc_pallet_traits::ValidateProcessWeights;
use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

use crate::Config;

pub trait WeightInfo {
    fn create_process(i: u32) -> Weight;
    fn disable_process() -> Weight;
}

/// Weight functions for `pallet_process_validation`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config + Config> ValidateProcessWeights<u32> for SubstrateWeight<T> {
	/// Storage: `ProcessValidation::ProcessModel` (r:1 w:0)
	/// Proof: `ProcessValidation::ProcessModel` (`max_values`: None, `max_size`: Some(38148), added: 40623, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 251]`.
	fn validate_process(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `136 + r * (14 ±0)`
		//  Estimated: `41613`
		// Minimum execution time: 15_801_000 picoseconds.
		Weight::from_parts(21_168_387, 0)
			.saturating_add(Weight::from_parts(0, 41613))
			// Standard Error: 73
			.saturating_add(Weight::from_parts(188_381, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `ProcessValidation::ProcessModel` (r:1 w:0)
	/// Proof: `ProcessValidation::ProcessModel` (`max_values`: None, `max_size`: Some(38148), added: 40623, mode: `MaxEncodedLen`)
	fn validate_process_min() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `146`
		//  Estimated: `41613`
		// Minimum execution time: 15_750_000 picoseconds.
		Weight::from_parts(16_210_000, 0)
			.saturating_add(Weight::from_parts(0, 41613))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `ProcessValidation::ProcessModel` (r:1 w:0)
	/// Proof: `ProcessValidation::ProcessModel` (`max_values`: None, `max_size`: Some(38148), added: 40623, mode: `MaxEncodedLen`)
	fn validate_process_max() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3650`
		//  Estimated: `41613`
		// Minimum execution time: 66_430_000 picoseconds.
		Weight::from_parts(67_511_000, 0)
			.saturating_add(Weight::from_parts(0, 41613))
			.saturating_add(T::DbWeight::get().reads(1))
	}
}

impl<T: frame_system::Config + Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `ProcessValidation::VersionModel` (r:1 w:1)
	/// Proof: `ProcessValidation::VersionModel` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	/// Storage: `ProcessValidation::ProcessModel` (r:1 w:1)
	/// Proof: `ProcessValidation::ProcessModel` (`max_values`: None, `max_size`: Some(38148), added: 40623, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 251]`.
	fn create_process(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `131`
		//  Estimated: `41613`
		// Minimum execution time: 35_961_000 picoseconds.
		Weight::from_parts(39_837_523, 0)
			.saturating_add(Weight::from_parts(0, 41613))
			// Standard Error: 60
			.saturating_add(Weight::from_parts(343_178, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `ProcessValidation::ProcessModel` (r:1 w:1)
	/// Proof: `ProcessValidation::ProcessModel` (`max_values`: None, `max_size`: Some(38148), added: 40623, mode: `MaxEncodedLen`)
	/// Storage: `ProcessValidation::VersionModel` (r:1 w:0)
	/// Proof: `ProcessValidation::VersionModel` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn disable_process() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `189`
		//  Estimated: `41613`
		// Minimum execution time: 44_400_000 picoseconds.
		Weight::from_parts(45_690_000, 0)
			.saturating_add(Weight::from_parts(0, 41613))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}

impl WeightInfo for () {
    fn create_process(_: u32) -> Weight {
        Weight::from_parts(0, 0)
    }
    fn disable_process() -> Weight {
        Weight::from_parts(0, 0)
    }
}
