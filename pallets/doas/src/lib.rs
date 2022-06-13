#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use sp_runtime::{traits::StaticLookup, DispatchResult};
use sp_std::prelude::*;

use frame_support::{
    dispatch::DispatchResultWithPostInfo,
    traits::{Get, UnfilteredDispatchable},
    weights::{GetDispatchInfo, Pays, Weight}
};
use frame_support::{traits::EnsureOrigin, Parameter};

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {

    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// A sudo-able call.
        type Call: Parameter + UnfilteredDispatchable<Origin = Self::Origin> + GetDispatchInfo;

        /// An Origin that is permitted to perform Doas operations
        type DoasOrigin: EnsureOrigin<Self::Origin>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A doas_root just took place. \[result\]
        DidAsRoot(DispatchResult),
        /// A doas just took place. \[result\]
        DidAs(DispatchResult)
    }

    // The pallet's dispatchable functions.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Authenticates the sudo key and dispatches a function call with `Root` origin.
        ///
        /// The dispatch origin for this call must be _Signed_.
        ///
        /// # <weight>
        /// - O(1).
        /// - Limited storage reads.
        /// - One DB write (event).
        /// - Weight of derivative `call` execution + 10,000.
        /// # </weight>
        #[pallet::weight({
          let dispatch_info = call.get_dispatch_info();
          (dispatch_info.weight.saturating_add(10_000), dispatch_info.class)
        })]
        pub(super) fn doas_root(origin: OriginFor<T>, call: Box<<T as Config>::Call>) -> DispatchResultWithPostInfo {
            // This is a public call, so we ensure that the origin is some signed account.
            T::DoasOrigin::ensure_origin(origin)?;

            let res = call.dispatch_bypass_filter(frame_system::RawOrigin::Root.into());
            Self::deposit_event(Event::DidAsRoot(res.map(|_| ()).map_err(|e| e.error)));
            // Sudo user does not pay a fee.
            Ok(Pays::No.into())
        }

        /// Authenticates the sudo key and dispatches a function call with `Root` origin.
        /// This function does not check the weight of the call, and instead allows the
        /// Sudo user to specify the weight of the call.
        ///
        /// The dispatch origin for this call must be _Signed_.
        ///
        /// # <weight>
        /// - O(1).
        /// - The weight of this call is defined by the caller.
        /// # </weight>
        #[pallet::weight((*_weight, call.get_dispatch_info().class))]
        pub(super) fn doas_root_unchecked_weight(
            origin: OriginFor<T>,
            call: Box<<T as Config>::Call>,
            _weight: Weight
        ) -> DispatchResultWithPostInfo {
            // This is a public call, so we ensure that the origin is some signed account.
            T::DoasOrigin::ensure_origin(origin)?;

            let res = call.dispatch_bypass_filter(frame_system::RawOrigin::Root.into());
            Self::deposit_event(Event::DidAsRoot(res.map(|_| ()).map_err(|e| e.error)));
            // Sudo user does not pay a fee.
            Ok(Pays::No.into())
        }

        /// Authenticates the sudo key and dispatches a function call with `Signed` origin from
        /// a given account.
        ///
        /// The dispatch origin for this call must be _Signed_.
        ///
        /// # <weight>
        /// - O(1).
        /// - Limited storage reads.
        /// - One DB write (event).
        /// - Weight of derivative `call` execution + 10,000.
        /// # </weight>
        #[pallet::weight({
          let dispatch_info = call.get_dispatch_info();
          (
            dispatch_info.weight
              .saturating_add(10_000)
              // AccountData for inner call origin accountdata.
              .saturating_add(T::DbWeight::get().reads_writes(1, 1)),
            dispatch_info.class,
          )
        })]
        pub(super) fn doas(
            origin: OriginFor<T>,
            who: <T::Lookup as StaticLookup>::Source,
            call: Box<<T as Config>::Call>
        ) -> DispatchResultWithPostInfo {
            // This is a public call, so we ensure that the origin is some signed account.
            T::DoasOrigin::ensure_origin(origin)?;

            let who = T::Lookup::lookup(who)?;

            let res = call.dispatch_bypass_filter(frame_system::RawOrigin::Signed(who).into());

            Self::deposit_event(Event::DidAs(res.map(|_| ()).map_err(|e| e.error)));
            // Doas user does not pay a fee.
            Ok(Pays::No.into())
        }
    }
}
