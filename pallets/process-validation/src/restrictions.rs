// This file contains the different types of restrictions that can be evaluated during
// a call to `validate_process`

use codec::{Decode, Encode};
use frame_support::Parameter;
use sp_std::vec::Vec;
use vitalam_pallet_traits::ProcessIO;

#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum Restriction {
    None,
    SenderOwnsAllInputs,
    FixedNumberOfInputs { num_inputs: u32 },
}

impl Default for Restriction {
    fn default() -> Self {
        Restriction::None
    }
}

pub fn validate_restriction<A, R, T, V>(
    restriction: &Restriction,
    sender: &A,
    inputs: &Vec<ProcessIO<A, R, T, V>>,
    _outputs: &Vec<ProcessIO<A, R, T, V>>,
) -> bool
where
    A: Parameter + Default,
    R: Parameter + Default + Ord,
    T: Parameter + Default + Ord,
    V: Parameter + Default,
{
    match *restriction {
        Restriction::None => true, // TODO implement some actual restrictions
        Restriction::FixedNumberOfInputs { num_inputs } => return inputs.len() == num_inputs as usize,
        Restriction::SenderOwnsAllInputs => {
            for input in inputs {
                let is_owned = match input.roles.get(&Default::default()) {
                    Some(account) => sender == account,
                    None => false,
                };
                if !is_owned {
                    return false;
                }
            }
            true
        }
    }
}
