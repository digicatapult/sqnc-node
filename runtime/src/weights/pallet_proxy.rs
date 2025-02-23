
//! Autogenerated weights for `pallet_proxy`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 43.0.0
//! DATE: 2025-02-10, STEPS: `50`, REPEAT: `1000`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `JCGQY2L5VK.local`, CPU: `<UNKNOWN>`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: 1024

// Executed Command:
// ./target/production/sqnc-node
// benchmark
// pallet
// --pallet
// pallet_proxy
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

/// Weight functions for `pallet_proxy`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_proxy::WeightInfo for WeightInfo<T> {
	/// Storage: `Proxy::Proxies` (r:1 w:0)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(1241), added: 3716, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[1, 31]`.
	fn proxy(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `127 + p * (37 ±0)`
		//  Estimated: `4706`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(13_392_187, 0)
			.saturating_add(Weight::from_parts(0, 4706))
			// Standard Error: 329
			.saturating_add(Weight::from_parts(33_830, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:0)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(1241), added: 3716, mode: `MaxEncodedLen`)
	/// Storage: `Proxy::Announcements` (r:1 w:1)
	/// Proof: `Proxy::Announcements` (`max_values`: None, `max_size`: Some(2233), added: 4708, mode: `MaxEncodedLen`)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn proxy_announced(a: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `314 + a * (68 ±0) + p * (37 ±0)`
		//  Estimated: `5698`
		// Minimum execution time: 18_000_000 picoseconds.
		Weight::from_parts(20_334_472, 0)
			.saturating_add(Weight::from_parts(0, 5698))
			// Standard Error: 345
			.saturating_add(Weight::from_parts(65_608, 0).saturating_mul(a.into()))
			// Standard Error: 357
			.saturating_add(Weight::from_parts(5_749, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Proxy::Announcements` (r:1 w:1)
	/// Proof: `Proxy::Announcements` (`max_values`: None, `max_size`: Some(2233), added: 4708, mode: `MaxEncodedLen`)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn remove_announcement(a: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `229 + a * (68 ±0)`
		//  Estimated: `5698`
		// Minimum execution time: 8_000_000 picoseconds.
		Weight::from_parts(8_831_769, 0)
			.saturating_add(Weight::from_parts(0, 5698))
			// Standard Error: 247
			.saturating_add(Weight::from_parts(103_201, 0).saturating_mul(a.into()))
			// Standard Error: 255
			.saturating_add(Weight::from_parts(23_446, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Proxy::Announcements` (r:1 w:1)
	/// Proof: `Proxy::Announcements` (`max_values`: None, `max_size`: Some(2233), added: 4708, mode: `MaxEncodedLen`)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn reject_announcement(a: u32, _p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `229 + a * (68 ±0)`
		//  Estimated: `5698`
		// Minimum execution time: 8_000_000 picoseconds.
		Weight::from_parts(10_768_727, 0)
			.saturating_add(Weight::from_parts(0, 5698))
			// Standard Error: 231
			.saturating_add(Weight::from_parts(69_452, 0).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:0)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(1241), added: 3716, mode: `MaxEncodedLen`)
	/// Storage: `Proxy::Announcements` (r:1 w:1)
	/// Proof: `Proxy::Announcements` (`max_values`: None, `max_size`: Some(2233), added: 4708, mode: `MaxEncodedLen`)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn announce(a: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `246 + a * (68 ±0) + p * (37 ±0)`
		//  Estimated: `5698`
		// Minimum execution time: 14_000_000 picoseconds.
		Weight::from_parts(17_439_235, 0)
			.saturating_add(Weight::from_parts(0, 5698))
			// Standard Error: 342
			.saturating_add(Weight::from_parts(64_530, 0).saturating_mul(a.into()))
			// Standard Error: 353
			.saturating_add(Weight::from_parts(22_210, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:1)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(1241), added: 3716, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[1, 31]`.
	fn add_proxy(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `127 + p * (37 ±0)`
		//  Estimated: `4706`
		// Minimum execution time: 10_000_000 picoseconds.
		Weight::from_parts(11_448_622, 0)
			.saturating_add(Weight::from_parts(0, 4706))
			// Standard Error: 288
			.saturating_add(Weight::from_parts(45_764, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:1)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(1241), added: 3716, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[1, 31]`.
	fn remove_proxy(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `127 + p * (37 ±0)`
		//  Estimated: `4706`
		// Minimum execution time: 10_000_000 picoseconds.
		Weight::from_parts(11_722_608, 0)
			.saturating_add(Weight::from_parts(0, 4706))
			// Standard Error: 336
			.saturating_add(Weight::from_parts(29_306, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:1)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(1241), added: 3716, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[1, 31]`.
	fn remove_proxies(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `127 + p * (37 ±0)`
		//  Estimated: `4706`
		// Minimum execution time: 8_000_000 picoseconds.
		Weight::from_parts(8_895_682, 0)
			.saturating_add(Weight::from_parts(0, 4706))
			// Standard Error: 288
			.saturating_add(Weight::from_parts(17_785, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:1)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(1241), added: 3716, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[1, 31]`.
	fn create_pure(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `139`
		//  Estimated: `4706`
		// Minimum execution time: 11_000_000 picoseconds.
		Weight::from_parts(12_626_502, 0)
			.saturating_add(Weight::from_parts(0, 4706))
			// Standard Error: 357
			.saturating_add(Weight::from_parts(3_874, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:1)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(1241), added: 3716, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[0, 30]`.
	fn kill_pure(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `164 + p * (37 ±0)`
		//  Estimated: `4706`
		// Minimum execution time: 8_000_000 picoseconds.
		Weight::from_parts(9_257_892, 0)
			.saturating_add(Weight::from_parts(0, 4706))
			// Standard Error: 247
			.saturating_add(Weight::from_parts(33_729, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
