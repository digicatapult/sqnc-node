#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::dispatch::{DispatchInfo, PostDispatchInfo};
use frame_support::weights::Weight;
use log;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::prelude::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

use sp_runtime::{
    traits::{Dispatchable, TransactionExtension},
    transaction_validity::{TransactionSource, TransactionValidityError, ValidTransaction},
    FixedPointOperand,
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
    #[pallet::without_storage_info]
    pub struct Pallet<T>(PhantomData<T>);

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
    T::RuntimeCall: Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo>,
    BalanceOf<T>: Send + Sync + FixedPointOperand,
{
    /// utility constructor. Used only in client/factory code.
    pub fn from(fee: BalanceOf<T>) -> Self {
        Self(fee)
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

impl<T: Config> TransactionExtension<T::RuntimeCall> for ChargeTransactionPayment<T>
where
    T::RuntimeCall: Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo>,
    BalanceOf<T>: Send + Sync + From<u64> + FixedPointOperand,
{
    const IDENTIFIER: &'static str = "ChargeTransactionPayment";
    type Implicit = ();
    type Val = ();
    type Pre = ();

    fn weight(&self, _: &T::RuntimeCall) -> Weight {
        log::debug!(target: "runtime", "ChargeTransactionPayment::weight - returning zero");
        Weight::zero()
    }

    fn validate(
        &self,
        origin: <T::RuntimeCall as Dispatchable>::RuntimeOrigin,
        _call: &T::RuntimeCall,
        _info: &DispatchInfo,
        _len: usize,
        _: (),
        _implication: &impl Encode,
        _source: TransactionSource,
    ) -> Result<
        (
            ValidTransaction,
            Self::Val,
            <T::RuntimeCall as Dispatchable>::RuntimeOrigin,
        ),
        TransactionValidityError,
    > {
        log::debug!(target: "runtime", "ChargeTransactionPayment::validate - skipping fee check");
        Ok((ValidTransaction::default(), (), origin))
    }

    fn prepare(
        self,
        _val: Self::Val,
        _origin: &<T::RuntimeCall as Dispatchable>::RuntimeOrigin,
        _call: &T::RuntimeCall,
        _info: &DispatchInfo,
        _len: usize,
    ) -> Result<Self::Pre, TransactionValidityError> {
        log::debug!(target: "runtime", "ChargeTransactionPayment::prepare - no-op");
        Ok(())
    }

    fn post_dispatch_details(
        _pre: Self::Pre,
        _info: &DispatchInfo,
        _post_info: &PostDispatchInfo,
        _len: usize,
        _result: &sp_runtime::DispatchResult,
    ) -> Result<Weight, TransactionValidityError> {
        log::debug!(target: "runtime", "ChargeTransactionPayment::post_dispatch_details - no refund/adjustment needed");
        Ok(Weight::zero())
    }
}
