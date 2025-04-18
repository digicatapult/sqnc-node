
//! Autogenerated weights for `pallet_organisation_data`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 43.0.0
//! DATE: 2025-04-04, STEPS: `50`, REPEAT: `1000`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `FNQGF7746D.local`, CPU: `<UNKNOWN>`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: 1024

// Executed Command:
// ./target/production/sqnc-node
// benchmark
// pallet
// --pallet
// pallet-organisation-data
// --extrinsic
// *
// --repeat
// 1000
// --output
// ./runtime/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_organisation_data`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_organisation_data::WeightInfo for WeightInfo<T> {
	/// Storage: `OrganisationData::OrgDataCount` (r:1 w:1)
	/// Proof: `OrganisationData::OrgDataCount` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `OrganisationData::OrgData` (r:1 w:1)
	/// Proof: `OrganisationData::OrgData` (`max_values`: None, `max_size`: Some(99), added: 2574, mode: `MaxEncodedLen`)
	fn set_value() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `258`
		//  Estimated: `3564`
		// Minimum execution time: 11_000_000 picoseconds.
		Weight::from_parts(12_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3564))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
