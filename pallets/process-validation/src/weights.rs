use dscp_pallet_traits::ValidateProcessWeights;
use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

use crate::Config;

pub trait WeightInfo {
    fn create_process(i: u32) -> Weight;
    fn disable_process() -> Weight;
}

pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config + Config> ValidateProcessWeights<u32> for SubstrateWeight<T> {
    /// Storage: ProcessValidation ProcessModel (r:1 w:0)
    /// Proof: ProcessValidation ProcessModel (max_values: None, max_size: Some(15348), added: 17823, mode: MaxEncodedLen)
    /// The range of component `r` is `[1, 101]`.
    fn validate_process(r: u32) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `131 + r * (14 ±0)`
        //  Estimated: `17823`
        // Minimum execution time: 8_000 nanoseconds.
        Weight::from_ref_time(8_836_060)
            .saturating_add(Weight::from_proof_size(17823))
            // Standard Error: 218
            .saturating_add(Weight::from_ref_time(163_940).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(1))
    }
    /// Storage: ProcessValidation ProcessModel (r:1 w:0)
    /// Proof: ProcessValidation ProcessModel (max_values: None, max_size: Some(15348), added: 17823, mode: MaxEncodedLen)
    fn validate_process_min() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `146`
        //  Estimated: `17823`
        // Minimum execution time: 8_000 nanoseconds.
        Weight::from_ref_time(9_000_000)
            .saturating_add(Weight::from_proof_size(17823))
            .saturating_add(T::DbWeight::get().reads(1))
    }
    /// Storage: ProcessValidation ProcessModel (r:1 w:0)
    /// Proof: ProcessValidation ProcessModel (max_values: None, max_size: Some(15348), added: 17823, mode: MaxEncodedLen)
    fn validate_process_max() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1582`
        //  Estimated: `17823`
        // Minimum execution time: 25_000 nanoseconds.
        Weight::from_ref_time(26_000_000)
            .saturating_add(Weight::from_proof_size(17823))
            .saturating_add(T::DbWeight::get().reads(1))
    }
}

impl<T: frame_system::Config + Config> WeightInfo for SubstrateWeight<T> {
    /// Storage: ProcessValidation VersionModel (r:1 w:1)
    /// Proof: ProcessValidation VersionModel (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
    /// Storage: ProcessValidation ProcessModel (r:1 w:1)
    /// Proof: ProcessValidation ProcessModel (max_values: None, max_size: Some(15348), added: 17823, mode: MaxEncodedLen)
    /// The range of component `r` is `[1, 101]`.
    fn create_process(r: u32) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `131`
        //  Estimated: `20351`
        // Minimum execution time: 17_000 nanoseconds.
        Weight::from_ref_time(17_903_220)
            .saturating_add(Weight::from_proof_size(20351))
            // Standard Error: 146
            .saturating_add(Weight::from_ref_time(218_780).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    /// Storage: ProcessValidation ProcessModel (r:1 w:1)
    /// Proof: ProcessValidation ProcessModel (max_values: None, max_size: Some(15348), added: 17823, mode: MaxEncodedLen)
    /// Storage: ProcessValidation VersionModel (r:1 w:0)
    /// Proof: ProcessValidation VersionModel (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
    fn disable_process() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `189`
        //  Estimated: `20351`
        // Minimum execution time: 21_000 nanoseconds.
        Weight::from_ref_time(23_000_000)
            .saturating_add(Weight::from_proof_size(20351))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(1))
    }
}

impl WeightInfo for () {
    fn create_process(_: u32) -> Weight {
        Weight::from_ref_time(0 as u64)
    }
    fn disable_process() -> Weight {
        Weight::from_ref_time(0 as u64)
    }
}
