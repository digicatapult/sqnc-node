#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::dispatch::{DispatchInfo, PostDispatchInfo};
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::prelude::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

use sp_runtime::{
    traits::{DispatchInfoOf, Dispatchable, Zero},
    transaction_validity::{InvalidTransaction, TransactionSource, TransactionValidityError, ValidTransaction},
    FixedPointOperand,
};

use frame_support::weights::Weight;
use frame_system::RawOrigin;
use sp_runtime::traits::Implication;
use sp_runtime::traits::TransactionExtension;

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

    fn zero_fee(
        &self,
        who: &T::AccountId,
        call: &T::RuntimeCall,
        info: &DispatchInfoOf<T::RuntimeCall>,
        _len: usize,
    ) -> Result<
        (
            BalanceOf<T>,
            <<T as Config>::OnFreeTransaction as OnFreeTransaction<T>>::LiquidityInfo,
        ),
        TransactionValidityError,
    > {
        <<T as Config>::OnFreeTransaction as OnFreeTransaction<T>>::zero_fee(
            who,
            call,
            info,
            Zero::zero(),
            Zero::zero(),
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

impl<T: Config> TransactionExtension<T::RuntimeCall> for ChargeTransactionPayment<T>
where
    T::RuntimeCall: Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo>,
    BalanceOf<T>: Send + Sync + From<u64> + FixedPointOperand,
{
    const IDENTIFIER: &'static str = "ChargeTransactionPayment";

    type Val = BalanceOf<T>;
    type Pre = (
        BalanceOf<T>,
        T::AccountId,
        <<T as Config>::OnFreeTransaction as OnFreeTransaction<T>>::LiquidityInfo,
    );
    type Implicit = ();

    fn validate(
        &self,
        origin: <T::RuntimeCall as Dispatchable>::RuntimeOrigin,
        _call: &T::RuntimeCall,
        _info: &<T::RuntimeCall as Dispatchable>::Info,
        _len: usize,
        _implicit: Self::Implicit,
        _implication: &impl Implication,
        _source: TransactionSource,
    ) -> Result<
        (
            ValidTransaction,
            Self::Val,
            <T::RuntimeCall as Dispatchable>::RuntimeOrigin,
        ),
        TransactionValidityError,
    > {
        Ok((
            ValidTransaction {
                priority: 0,
                requires: vec![],
                provides: vec![],
                longevity: u64::MAX,
                propagate: true,
            },
            self.0,
            origin,
        ))
    }

    fn weight(&self, _call: &T::RuntimeCall) -> Weight {
        Weight::zero()
    }

    fn prepare(
        self,
        val: Self::Val,
        origin: &<T::RuntimeCall as Dispatchable>::RuntimeOrigin,
        call: &T::RuntimeCall,
        info: &<T::RuntimeCall as Dispatchable>::Info,
        len: usize,
    ) -> Result<Self::Pre, TransactionValidityError> {
        let who = match origin.clone().into() {
            Ok(RawOrigin::Signed(who)) => who,
            _ => return Err(TransactionValidityError::Invalid(InvalidTransaction::BadSigner)),
        };

        let (_fee, imbalance) = self.zero_fee(&who, call, info, len)?;
        Ok((val, who, imbalance))
    }
}
