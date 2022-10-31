use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
    fn create_process(i: usize) -> Weight;
    fn disable_process() -> Weight;
}

pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    // Storage: ProcessValidation VersionModel (r:1 w:1)
    // Storage: ProcessValidation ProcessModel (r:1 w:1)
    /// The range of component `i` is `[1, 200]`.
    fn create_process(i: usize) -> Weight {
        Weight::from_ref_time(25_000_000 as u64)
            // Standard Error: 156
            .saturating_add(Weight::from_ref_time(91_785 as u64).saturating_mul(i as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
    // Storage: ProcessValidation ProcessModel (r:1 w:1)
    // Storage: ProcessValidation VersionModel (r:1 w:0)
    fn disable_process() -> Weight {
        Weight::from_ref_time(34_000_000 as u64)
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
}

impl WeightInfo for () {
    fn create_process(_: usize) -> Weight {
        Weight::from_ref_time(0 as u64)
    }
    fn disable_process() -> Weight {
        Weight::from_ref_time(0 as u64)
    }
}
