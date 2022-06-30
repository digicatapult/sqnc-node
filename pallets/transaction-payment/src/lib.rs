#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
    decl_module,
    traits::Get,
    weights::{DispatchClass, DispatchInfo, PostDispatchInfo}
};
use sp_std::prelude::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

use sp_runtime::{
    traits::{DispatchInfoOf, Dispatchable, SaturatedConversion, Saturating, SignedExtension, Zero},
    transaction_validity::{TransactionPriority, TransactionValidity, TransactionValidityError, ValidTransaction},
    FixedPointOperand, FixedU128
};

mod payment;

pub use payment::*;

/// Fee multiplier.
pub type Multiplier = FixedU128;

type BalanceOf<T> = <<T as Config>::OnChargeTransaction as OnChargeTransaction<T>>::Balance;

pub trait Config: frame_system::Config {
    /// Handler for withdrawing, refunding and depositing the transaction fee.
    /// Transaction fees are withdrawn before the transaction is executed.
    /// After the transaction was executed the transaction weight can be
    /// adjusted, depending on the used resources by the transaction. If the
    /// transaction weight is lower than expected, parts of the transaction fee
    /// might be refunded. In the end the fees can be deposited.
    type OnChargeTransaction: OnChargeTransaction<Self>;
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin{}
}

/// Require the transactor pay for themselves and maybe include a tip to gain additional priority
/// in the queue.
#[derive(Encode, Decode, Clone, Eq, PartialEq)]
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

    fn withdraw_fee(
        &self,
        who: &T::AccountId,
        call: &T::Call,
        info: &DispatchInfoOf<T::Call>,
        _len: usize
    ) -> Result<
        (
            BalanceOf<T>,
            <<T as Config>::OnChargeTransaction as OnChargeTransaction<T>>::LiquidityInfo
        ),
        TransactionValidityError
    > {
        <<T as Config>::OnChargeTransaction as OnChargeTransaction<T>>::withdraw_fee(
            who,
            call,
            info,
            Zero::zero(),
            Zero::zero()
        )
        .map(|i| (Zero::zero(), i))
    }

    /// Get an appropriate priority for a transaction with the given length and info.
    ///
    /// This will try and optimise the `fee/weight` `fee/length`, whichever is consuming more of the
    /// maximum corresponding limit.
    ///
    /// For example, if a transaction consumed 1/4th of the block length and half of the weight, its
    /// final priority is `fee * min(2, 4) = fee * 2`. If it consumed `1/4th` of the block length
    /// and the entire block weight `(1/1)`, its priority is `fee * min(1, 4) = fee * 1`. This means
    ///  that the transaction which consumes more resources (either length or weight) with the same
    /// `fee` ends up having lower priority.
    fn get_priority(len: usize, info: &DispatchInfoOf<T::Call>, final_fee: BalanceOf<T>) -> TransactionPriority {
        let weight_saturation = T::BlockWeights::get().max_block / info.weight.max(1);
        let max_block_length = *T::BlockLength::get().max.get(DispatchClass::Normal);
        let len_saturation = max_block_length as u64 / (len as u64).max(1);
        let coefficient: BalanceOf<T> = weight_saturation.min(len_saturation).saturated_into::<BalanceOf<T>>();
        final_fee
            .saturating_mul(coefficient)
            .saturated_into::<TransactionPriority>()
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
        // tip
        BalanceOf<T>,
        // who paid the fee
        Self::AccountId,
        // imbalance resulting from withdrawing the fee
        <<T as Config>::OnChargeTransaction as OnChargeTransaction<T>>::LiquidityInfo
    );
    fn additional_signed(&self) -> sp_std::result::Result<(), TransactionValidityError> {
        Ok(())
    }

    fn validate(
        &self,
        who: &Self::AccountId,
        call: &Self::Call,
        info: &DispatchInfoOf<Self::Call>,
        len: usize
    ) -> TransactionValidity {
        let (fee, _) = self.withdraw_fee(who, call, info, len)?;
        Ok(ValidTransaction {
            priority: Self::get_priority(len, info, fee),
            ..Default::default()
        })
    }

    fn pre_dispatch(
        self,
        who: &Self::AccountId,
        call: &Self::Call,
        info: &DispatchInfoOf<Self::Call>,
        len: usize
    ) -> Result<Self::Pre, TransactionValidityError> {
        let (_fee, imbalance) = self.withdraw_fee(who, call, info, len)?;
        Ok((self.0, who.clone(), imbalance))
    }
}
