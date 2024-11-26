use frame_support::weights::Weight;

use super::pallet_process_validation::WeightInfo;
use sqnc_pallet_traits::ValidateProcessWeights;

impl<T: frame_system::Config> ValidateProcessWeights<u32> for WeightInfo<T> {
    fn validate_process(p: u32) -> Weight {
        <Self as pallet_process_validation::WeightInfo>::validate_process(p)
    }

    fn validate_process_min() -> Weight {
        <Self as pallet_process_validation::WeightInfo>::validate_process_min()
    }

    fn validate_process_max() -> Weight {
        <Self as pallet_process_validation::WeightInfo>::validate_process_max()
    }
}
