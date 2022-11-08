use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;
// weights change
pub trait WeightInfo {
    fn create_process(i: u32) -> Weight;
    fn disable_process() -> Weight;
}

pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    // Storage: ProcessValidation VersionModel (r:1 w:1)
    // Storage: ProcessValidation ProcessModel (r:1 w:1)
    /// The range of component `r` is `[1, 101]`.
    fn create_process(r: u32) -> Weight {
        Weight::from_ref_time(28_000_000 as u64)
            // Standard Error: 1_911
            .saturating_add(Weight::from_ref_time(147_095 as u64).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
    // Storage: ProcessValidation ProcessModel (r:1 w:1)
    // Storage: ProcessValidation VersionModel (r:1 w:0)
    fn disable_process() -> Weight {
        Weight::from_ref_time(33_000_000 as u64)
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
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
