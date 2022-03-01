// This file contains the different types of restrictions that can be evaluated during
// a call to `validate_process`

use codec::{Decode, Encode};
use frame_support::Parameter;
use sp_std::vec::Vec;
use vitalam_pallet_traits::ProcessIO;

#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum Restriction<TokenMetadataKey, TokenMetadataValue>
where
    TokenMetadataKey: Parameter + Default + Ord,
    TokenMetadataValue: Parameter + Default,
{
    None,
    SenderOwnsAllInputs,
    FixedNumberOfInputs {
        num_inputs: u32,
    },
    FixedNumberOfOutputs {
        num_outputs: u32,
    },
    FixedMetadataValue {
        input_index: u32,
        metadata_key: TokenMetadataKey,
        metadata_value: TokenMetadataValue,
    },
}

impl<TokenMetadataKey, TokenMetadataValue> Default for Restriction<TokenMetadataKey, TokenMetadataValue>
where
    TokenMetadataKey: Parameter + Default + Ord,
    TokenMetadataValue: Parameter + Default,
{
    fn default() -> Self {
        Restriction::None
    }
}

pub fn validate_restriction<A, R, T, V>(
    restriction: &Restriction<T, V>,
    sender: &A,
    inputs: &Vec<ProcessIO<A, R, T, V>>,
    outputs: &Vec<ProcessIO<A, R, T, V>>,
) -> bool
where
    A: Parameter + Default,
    R: Parameter + Default + Ord,
    T: Parameter + Default + Ord,
    V: Parameter + Default,
{
    match &*restriction {
        Restriction::<T, V>::None => true,
        Restriction::FixedNumberOfInputs { num_inputs } => return inputs.len() == *num_inputs as usize,
        Restriction::FixedNumberOfOutputs { num_outputs } => return outputs.len() == *num_outputs as usize,
        Restriction::FixedMetadataValue {
            input_index,
            metadata_key,
            metadata_value,
        } => {
            let selected_input = &inputs[*input_index as usize];
            let meta = selected_input.metadata.get(&metadata_key);
            meta == Some(&metadata_value)
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use sp_std::collections::btree_map::BTreeMap;

    #[test]
    fn no_restriction_succeeds() {
        let result = validate_restriction::<u64, u32, u32, u64>(&Restriction::None, &1u64, &Vec::new(), &Vec::new());
        assert!(result);
    }

    #[test]
    fn sender_owns_inputs_restriction_no_inputs_succeeds() {
        let result = validate_restriction::<u64, u32, u32, u64>(
            &Restriction::SenderOwnsAllInputs,
            &1u64,
            &Vec::new(),
            &Vec::new(),
        );
        assert!(result);
    }

    #[test]
    fn sender_owns_inputs_restriction_owns_all_inputs_succeeds() {
        let mut is_owner: BTreeMap<u32, u64> = BTreeMap::new();
        is_owner.insert(Default::default(), 1u64);
        let inputs = vec![
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
        ];
        let result =
            validate_restriction::<u64, u32, u32, u64>(&Restriction::SenderOwnsAllInputs, &1u64, &inputs, &Vec::new());
        assert!(result);
    }

    #[test]
    fn sender_owns_inputs_restriction_owns_no_inputs_fails() {
        let mut is_not_owner: BTreeMap<u32, u64> = BTreeMap::new();
        is_not_owner.insert(Default::default(), 2u64);
        let inputs = vec![
            ProcessIO {
                roles: is_not_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
            ProcessIO {
                roles: is_not_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
        ];
        let result =
            validate_restriction::<u64, u32, u32, u64>(&Restriction::SenderOwnsAllInputs, &1u64, &inputs, &Vec::new());
        assert!(!result);
    }

    #[test]
    fn sender_owns_inputs_restriction_owns_some_inputs_fails() {
        let mut is_owner: BTreeMap<u32, u64> = BTreeMap::new();
        is_owner.insert(Default::default(), 1u64);
        let mut is_not_owner: BTreeMap<u32, u64> = BTreeMap::new();
        is_not_owner.insert(Default::default(), 2u64);
        let inputs = vec![
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
            ProcessIO {
                roles: is_not_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
        ];
        let result =
            validate_restriction::<u64, u32, u32, u64>(&Restriction::SenderOwnsAllInputs, &1u64, &inputs, &Vec::new());
        assert!(!result);
    }

    #[test]
    fn sender_owns_inputs_restriction_incorrect_role_fails() {
        let mut is_owner: BTreeMap<u32, u64> = BTreeMap::new();
        is_owner.insert(Default::default(), 1u64);
        let mut is_not_owner: BTreeMap<u32, u64> = BTreeMap::new();
        is_not_owner.insert(1u32, 1u64);
        let inputs = vec![
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
            ProcessIO {
                roles: is_not_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
        ];
        let result =
            validate_restriction::<u64, u32, u32, u64>(&Restriction::SenderOwnsAllInputs, &1u64, &inputs, &Vec::new());
        assert!(!result);
    }

    #[test]
    fn fixed_number_of_inputs_restriction_matches_fixed_input_total() {
        let mut is_owner: BTreeMap<u32, u64> = BTreeMap::new();
        is_owner.insert(Default::default(), 1u64);
        let inputs = vec![
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
        ];
        let result = validate_restriction::<u64, u32, u32, u64>(
            &Restriction::FixedNumberOfInputs { num_inputs: 4 },
            &1u64,
            &inputs,
            &Vec::new(),
        );
        assert!(result);
    }

    #[test]
    fn fixed_number_of_inputs_restriction_matches_fixed_input_total_fail() {
        let mut is_owner: BTreeMap<u32, u64> = BTreeMap::new();
        is_owner.insert(Default::default(), 1u64);
        let inputs = vec![
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
        ];
        let result = validate_restriction::<u64, u32, u32, u64>(
            &Restriction::FixedNumberOfInputs { num_inputs: 1 },
            &1u64,
            &inputs,
            &Vec::new(),
        );
        assert!(!result);
    }

    #[test]
    fn fixed_number_of_outputs_restriction_matches_fixed_output_total() {
        let mut is_owner: BTreeMap<u32, u64> = BTreeMap::new();
        is_owner.insert(Default::default(), 1u64);
        let outputs = vec![
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
        ];
        let result = validate_restriction::<u64, u32, u32, u64>(
            &Restriction::FixedNumberOfOutputs { num_outputs: 2 },
            &1u64,
            &Vec::new(),
            &outputs,
        );
        assert!(result);
    }

    #[test]
    fn fixed_number_of_output_restriction_matches_fixed_output_total_fail() {
        let mut is_owner: BTreeMap<u32, u64> = BTreeMap::new();
        is_owner.insert(Default::default(), 1u64);
        let outputs = vec![
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
        ];
        let result = validate_restriction::<u64, u32, u32, u64>(
            &Restriction::FixedNumberOfOutputs { num_outputs: 1 },
            &1u64,
            &Vec::new(),
            &outputs,
        );
        assert!(!result);
    }

    #[test]
    fn fixed_metadata_value_outputs_total() {
        let mut is_owner: BTreeMap<u32, u64> = BTreeMap::new();
        is_owner.insert(Default::default(), 1u64);
        let mut real_metadata = BTreeMap::new();
        real_metadata.insert(2, 110);
        let inputs = vec![
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
            ProcessIO {
                roles: is_owner.clone(),
                metadata: real_metadata,
                parent_index: None,
            },
        ];
        let result = validate_restriction::<u64, u32, u32, u64>(
            &Restriction::FixedMetadataValue {
                input_index: 2,
                metadata_key: 2,
                metadata_value: 110,
            },
            &1u64,
            &inputs,
            &Vec::new(),
        );
        assert!(result);
    }

    #[test]
    fn fixed_metadata_value_outputs_total_fail() {
        let mut is_owner: BTreeMap<u32, u64> = BTreeMap::new();
        is_owner.insert(Default::default(), 1u64);
        let mut real_metadata = BTreeMap::new();
        real_metadata.insert(2, 110);
        let inputs = vec![
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None,
            },
            ProcessIO {
                roles: is_owner.clone(),
                metadata: real_metadata,
                parent_index: None,
            },
        ];
        let result = validate_restriction::<u64, u32, u32, u64>(
            &Restriction::FixedMetadataValue {
                input_index: 1,
                metadata_key: 2,
                metadata_value: 110,
            },
            &1u64,
            &inputs,
            &Vec::new(),
        );
        assert!(!result);
    }
}
