use super::*;
use frame_support::traits::OnRuntimeUpgrade;

/// The log target.
const TARGET: &'static str = "runtime::symmetric-key::migration";

pub mod v1 {
    use super::*;
    use frame_support::pallet_prelude::*;

    /// Migrate the symmetric-key pallet from V0 to V1.
    pub struct MigrateToV1<T>(core::marker::PhantomData<T>);

    impl<T: Config> OnRuntimeUpgrade for MigrateToV1<T> {
        fn on_runtime_upgrade() -> Weight {
            let version = StorageVersion::get::<Pallet<T>>();
            if version != 0 {
                log::warn!(
                  target: TARGET,
                  "skipping v0 to v1 migration: executed on wrong storage version. Expected version 0, found {:?}",
                  version,
                );
                return T::DbWeight::get().reads(1);
            }

            Pallet::<T>::migrate_v0_to_v1() + T::DbWeight::get().reads(1)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tests::*;
    use frame_support::pallet_prelude::*;
    use hex_literal::hex;

    #[test]
    fn migration_v0_to_v1_works() {
        new_test_ext().execute_with(|| {
            // Assume that we are at V0
            StorageVersion::new(0).put::<SymmetricKey>();

            // initialise scheduleId with value form KEY_ROTATE_ID
            let v0_id: BoundedVec<u8, ConstU32<32>> = BoundedVec::truncate_from(KEY_ROTATE_ID.encode());
            <KeyScheduleId<Test>>::put(Some(v0_id));

            SymmetricKey::migrate_v0_to_v1();

            let new_id = <KeyScheduleId<Test>>::get();
            let expected_id = BoundedVec::truncate_from(
                hex!("241E5837D85F8900111D972625D1EA963ECC5352AE5A1A12F0F218EB1F362CD4").encode(),
            );
            assert_eq!(new_id, Some(expected_id));
        })
    }
}
