//! # Organisation Data Pallet
//!
//! The Organisation Data Pallet allows setting values against a configured set of keys

#![cfg_attr(not(feature = "std"), no_std)]

mod benchmarking;
mod mock;
mod tests;

pub mod migrations;
pub mod weights;

use frame_support::pallet_prelude::*;
use frame_support::traits::{ChangeMembers, InitializeMembers};
use frame_system::pallet_prelude::*;
use parity_scale_codec::MaxEncodedLen;
use sp_io::MultiRemovalResults;
use sp_runtime::DispatchResult;

pub use pallet::*;
pub use weights::*;

pub const LOG_TARGET: &'static str = "runtime::organisation-data";

#[frame_support::pallet]
pub mod pallet {

    use super::*;

    /// Configure the pallet by specifying the parameters and types on which it
    /// depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The Event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type OrgDataKey: Parameter + Default + MaxEncodedLen + Ord;
        type OrgDataValue: Parameter + Default + MaxEncodedLen;

        #[pallet::constant]
        type MaxOrgMemberEntries: Get<u32>;

        /// Information on runtime weights.
        type WeightInfo: WeightInfo;
    }

    /// The in-code storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn org_data)]
    pub type OrgData<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        <T as frame_system::Config>::AccountId,
        Blake2_128Concat,
        T::OrgDataKey,
        T::OrgDataValue,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn org_data_count)]
    pub type OrgDataCount<T: Config> =
        StorageMap<_, Blake2_128Concat, <T as frame_system::Config>::AccountId, u32, ValueQuery>;

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A member has updated a metadata value at the specified key
        UpdateOrgData(T::AccountId, T::OrgDataKey),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Attempted to set value from an origin that is not a registered member organisation
        NotMember,
        /// Maximum entry count exceeded,
        TooManyEntries,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Sets a value for a member at the supplied key
        ///
        #[pallet::call_index(0)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::set_value())]
        pub fn set_value(origin: OriginFor<T>, key: T::OrgDataKey, value: T::OrgDataValue) -> DispatchResult {
            // Check it was signed and get the signer
            let sender = ensure_signed(origin)?;

            let prev_entry_count = <OrgDataCount<T>>::try_get(sender.clone()).map_err(|_| Error::<T>::NotMember)?;

            // if we are a member find out if there's a current value and calculate the new entry count
            let maybe_prev_entry = <OrgData<T>>::try_get(sender.clone(), key.clone());
            let new_entry_count = match &maybe_prev_entry {
                Ok(_) => prev_entry_count,
                Err(_) => prev_entry_count + 1,
            };

            // check this member won't exceed the maximum entry count
            ensure!(
                new_entry_count <= T::MaxOrgMemberEntries::get(),
                Error::<T>::TooManyEntries
            );

            // update the count and set the new value
            <OrgData<T>>::set(sender.clone(), key.clone(), value);
            <OrgDataCount<T>>::set(sender.clone(), new_entry_count);

            Self::deposit_event(Event::UpdateOrgData(sender.clone(), key.clone()));

            Ok(())
        }
    }
}

impl<T: Config> ChangeMembers<T::AccountId> for Pallet<T> {
    fn change_members_sorted(incoming: &[T::AccountId], outgoing: &[T::AccountId], sorted_new: &[T::AccountId]) {
        log::debug!(
            target: LOG_TARGET,
            "membership has changed to {:?}", sorted_new
        );

        incoming.iter().for_each(|m| {
            <OrgDataCount<T>>::set(m, 0u32);
        });

        outgoing.iter().for_each(|m| {
            <OrgDataCount<T>>::remove(m);
            let MultiRemovalResults { maybe_cursor, .. } =
                <OrgData<T>>::clear_prefix(m, T::MaxOrgMemberEntries::get(), None);

            if maybe_cursor.is_some() {
                log::error!(
                    target: LOG_TARGET,
                    "Unexpectedly did not fully clear org data for member {:?}", m
                );
            }
        });
    }
}

impl<T: Config> InitializeMembers<T::AccountId> for Pallet<T> {
    fn initialize_members(members: &[T::AccountId]) {
        log::debug!(
            target: LOG_TARGET,
            "New members initialised {:?}", members
        );

        members.iter().for_each(|m| {
            <OrgDataCount<T>>::set(m, 0u32);
        });
    }
}
