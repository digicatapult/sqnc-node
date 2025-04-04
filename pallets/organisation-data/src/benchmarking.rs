#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::v1::{account, benchmarks};
use frame_system::RawOrigin;

const SEED: u32 = 0;

benchmarks! {
    set_value {
        let caller: T::AccountId = account("owner", 0, SEED);
        <OrgDataCount<T>>::set(caller.clone(), 0);

        let key: T::OrgDataKey = Default::default();
        let value: T::OrgDataValue = Default::default();
    }: _(RawOrigin::Signed(caller), key, value)

    impl_benchmark_test_suite!(
        OrganisationData,
        crate::mock::new_test_ext(),
        crate::mock::Test,
    );
}
