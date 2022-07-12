#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::weights::{DispatchInfo, PostDispatchInfo};
use scale_info::TypeInfo;
use sp_std::prelude::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

use sp_runtime::{
    traits::{DispatchInfoOf, Dispatchable, SignedExtension, Zero},
    transaction_validity::TransactionValidityError,
    FixedPointOperand
};

mod payment;

pub use pallet::*;
pub use payment::*;

type BalanceOf<T> = <<T as Config>::OnFreeTransaction as OnFreeTransaction<T>>::Balance;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    /// The pallet's configuration trait.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type OnFreeTransaction: OnFreeTransaction<Self>;
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {}
}

/// Require the transactor have balance. All transactions are free - they have no fee
#[derive(Encode, Decode, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct ChargeTransactionPayment<T: Config>(#[codec(compact)] BalanceOf<T>);

impl<T: Config> ChargeTransactionPayment<T>
where
    T::Call: Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo>,
    BalanceOf<T>: Send + Sync + FixedPointOperand
{
    /// utility constructor. Used only in client/factory code.
    pub fn from(fee: BalanceOf<T>) -> Self {
        Self(fee)
    }

    fn zero_fee(
        &self,
        who: &T::AccountId,
        call: &T::Call,
        info: &DispatchInfoOf<T::Call>,
        _len: usize
    ) -> Result<
        (
            BalanceOf<T>,
            <<T as Config>::OnFreeTransaction as OnFreeTransaction<T>>::LiquidityInfo
        ),
        TransactionValidityError
    > {
        <<T as Config>::OnFreeTransaction as OnFreeTransaction<T>>::zero_fee(
            who,
            call,
            info,
            Zero::zero(),
            Zero::zero()
        )
        .map(|i| (Zero::zero(), i))
    }
}

impl<T: Config> sp_std::fmt::Debug for ChargeTransactionPayment<T> {
    #[cfg(feature = "std")]
    fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
        write!(f, "ChargeTransactionPayment<{:?}>", self.0)
    }
    #[cfg(not(feature = "std"))]
    fn fmt(&self, _: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
        Ok(())
    }
}

impl<T: Config> SignedExtension for ChargeTransactionPayment<T>
where
    BalanceOf<T>: Send + Sync + From<u64> + FixedPointOperand,
    T::Call: Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo>
{
    const IDENTIFIER: &'static str = "ChargeTransactionPayment";
    type AccountId = T::AccountId;
    type Call = T::Call;
    type AdditionalSigned = ();
    type Pre = (
        BalanceOf<T>,
        Self::AccountId,
        <<T as Config>::OnFreeTransaction as OnFreeTransaction<T>>::LiquidityInfo
    );
    fn additional_signed(&self) -> sp_std::result::Result<(), TransactionValidityError> {
        Ok(())
    }

    fn pre_dispatch(
        self,
        who: &Self::AccountId,
        call: &Self::Call,
        info: &DispatchInfoOf<Self::Call>,
        len: usize
    ) -> Result<Self::Pre, TransactionValidityError> {
        let (_fee, imbalance) = self.zero_fee(who, call, info, len)?;
        Ok((self.0, who.clone(), imbalance))
    }
}
