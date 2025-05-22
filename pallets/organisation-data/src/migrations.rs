use super::*;
use frame_support::traits::OnRuntimeUpgrade;

/// The log target.
const TARGET: &'static str = "runtime::organisation-data::migration";

pub mod v1 {
    use super::*;
    use frame_support::pallet_prelude::*;

    /// Migrate the org-data pallet from V0 to V1.
    pub struct MigrateToV1<T, I>(core::marker::PhantomData<(T, I)>);

    impl<T: Config + pallet_membership::Config<I>, I: 'static> OnRuntimeUpgrade for MigrateToV1<T, I> {
        fn on_runtime_upgrade() -> Weight {
            let version = StorageVersion::get::<Pallet<T>>();
            let existing_members = <OrgDataCount<T>>::iter().count();
            if existing_members != 0 || version != 1 {
                log::warn!(
                    target: TARGET,
                    "skipping v0 to v1 migration: executed on wrong storage version. Expected version 0, found {:?}",
                    version,
                );
                return T::DbWeight::get().reads(1);
            }
            let members = pallet_membership::Pallet::<T, I>::members();

            log::debug!(target: TARGET, "Running migration of organisation data pallet version 0 -> 1. Members are {:?}", members.as_slice());

            Pallet::<T>::initialize_members(members.as_slice());
            T::DbWeight::get().reads(1) + T::DbWeight::get().writes(members.len() as u64)
        }
    }
}

#[cfg(test)]
mod test {
    use super::{v1::MigrateToV1, *};
    use crate::mock::*;
    use frame_support::pallet_prelude::*;

    #[test]
    fn migration_v0_to_v1_works() {
        new_test_ext().execute_with(|| {
            // Assume that we are at V1 as that's the initial state
            StorageVersion::new(1).put::<OrganisationData>();
            Membership::add_member(RuntimeOrigin::root(), 0u64).unwrap();
            Membership::add_member(RuntimeOrigin::root(), 1u64).unwrap();
            Membership::add_member(RuntimeOrigin::root(), 2u64).unwrap();

            // do the runtime upgrade
            MigrateToV1::<Test, ()>::on_runtime_upgrade();

            let mut values = <OrgDataCount<Test>>::iter().collect::<Vec<_>>();
            values.sort_by_key(|(m, _)| *m);
            assert_eq!(values, vec![(0, 0), (1, 0), (2, 0)]);
        })
    }

    #[test]
    fn migration_v0_to_v1_doesnt_overwrite_incorrect_version() {
        new_test_ext().execute_with(|| {
            // Assume that we are at V1 as that's the initial state
            StorageVersion::new(2).put::<OrganisationData>();
            Membership::add_member(RuntimeOrigin::root(), 0u64).unwrap();
            Membership::add_member(RuntimeOrigin::root(), 1u64).unwrap();
            Membership::add_member(RuntimeOrigin::root(), 2u64).unwrap();

            // do the runtime upgrade
            MigrateToV1::<Test, ()>::on_runtime_upgrade();

            let mut values = <OrgDataCount<Test>>::iter().collect::<Vec<_>>();
            values.sort_by_key(|(m, _)| *m);
            assert_eq!(values, vec![]);
        })
    }

    #[test]
    fn migration_v0_to_v1_doesnt_overwrite_existing_members() {
        new_test_ext().execute_with(|| {
            // Assume that we are at V1 as that's the initial state
            StorageVersion::new(1).put::<OrganisationData>();
            <OrgDataCount<Test>>::set(0, 0);
            <OrgDataCount<Test>>::set(1, 0);

            Membership::add_member(RuntimeOrigin::root(), 0u64).unwrap();
            Membership::add_member(RuntimeOrigin::root(), 1u64).unwrap();
            Membership::add_member(RuntimeOrigin::root(), 2u64).unwrap();

            // do the runtime upgrade
            MigrateToV1::<Test, ()>::on_runtime_upgrade();

            let mut values = <OrgDataCount<Test>>::iter().collect::<Vec<_>>();
            values.sort_by_key(|(m, _)| *m);
            assert_eq!(values, vec![(0, 0), (1, 0)]);
        })
    }
}
