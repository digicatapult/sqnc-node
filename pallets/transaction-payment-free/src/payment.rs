use crate::Config;
use codec::FullCodec;
use frame_support::{
    traits::{Currency, Imbalance, OnUnbalanced},
    unsigned::TransactionValidityError
};
use sp_runtime::{
    traits::{AtLeast32BitUnsigned, DispatchInfoOf, MaybeSerializeDeserialize, Zero},
    transaction_validity::InvalidTransaction
};
use sp_std::{fmt::Debug, marker::PhantomData};

type NegativeImbalanceOf<C, T> = <C as Currency<<T as frame_system::Config>::AccountId>>::NegativeImbalance;

pub trait OnFreeTransaction<T: Config> {
    /// The underlying integer type in which fees are calculated.
    type Balance: AtLeast32BitUnsigned
        + FullCodec
        + Copy
        + MaybeSerializeDeserialize
        + Debug
        + Default
        + scale_info::TypeInfo;
    type LiquidityInfo: Default;

    fn zero_fee(
        who: &T::AccountId,
        call: &T::Call,
        dispatch_info: &DispatchInfoOf<T::Call>,
        fee: Self::Balance,
        tip: Self::Balance
    ) -> Result<Self::LiquidityInfo, TransactionValidityError>;
}

/// Implements the transaction payment for a pallet implementing the `Currency`
/// trait (eg. the pallet_balances) using an unbalance handler (implementing
/// `OnUnbalanced`).
pub struct CurrencyAdapter<C, OU>(PhantomData<(C, OU)>);

/// Default implementation for a Currency and an OnUnbalanced handler.
impl<T, C, OU> OnFreeTransaction<T> for CurrencyAdapter<C, OU>
where
    T: Config,
    C: Currency<<T as frame_system::Config>::AccountId>,
    C::PositiveImbalance:
        Imbalance<<C as Currency<<T as frame_system::Config>::AccountId>>::Balance, Opposite = C::NegativeImbalance>,
    C::NegativeImbalance:
        Imbalance<<C as Currency<<T as frame_system::Config>::AccountId>>::Balance, Opposite = C::PositiveImbalance>,
    OU: OnUnbalanced<NegativeImbalanceOf<C, T>>
{
    type LiquidityInfo = Option<NegativeImbalanceOf<C, T>>;
    type Balance = <C as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    /// Check the account has balance and set fee to 0.
    fn zero_fee(
        who: &T::AccountId,
        _call: &T::Call,
        _info: &DispatchInfoOf<T::Call>,
        _fee: Self::Balance,
        _tip: Self::Balance
    ) -> Result<Self::LiquidityInfo, TransactionValidityError> {
        let balance = C::total_balance(who);

        if balance == Zero::zero() {
            return Err(InvalidTransaction::Payment.into());
        }
        Ok(None)
    }
}
