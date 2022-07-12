// This file contains the different types of restrictions that can be evaluated during
// a call to `validate_process`

use codec::{Decode, Encode, MaxEncodedLen};
use dscp_pallet_traits::ProcessIO;
use frame_support::Parameter;
use scale_info::TypeInfo;
// use sp_std::boxed::Box;
use sp_std::vec::Vec;

// #[derive(Encode, Decode, Clone, MaxEncodedLen, TypeInfo, PartialEq)]
// #[cfg_attr(feature = "std", derive(Debug))]
// pub enum BinaryOperator {
//     AND,
//     OR,
//     XOR,
//     NAND,
//     NOR
// }

#[derive(Encode, Decode, Clone, MaxEncodedLen, TypeInfo, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum Restriction<RoleKey, TokenMetadataKey, TokenMetadataValue, TokenMetadataValueDiscriminator> {
    None,
    // Combined {
    //     operator: BinaryOperator,
    //     restriction_a: Box<Restriction<RoleKey, TokenMetadataKey, TokenMetadataValue, TokenMetadataValueDiscriminator>>,
    //     restriction_b: Box<Restriction<RoleKey, TokenMetadataKey, TokenMetadataValue, TokenMetadataValueDiscriminator>>
    // },
    SenderOwnsAllInputs,
    SenderHasInputRole {
        index: u32,
        role_key: RoleKey
    },
    SenderHasOutputRole {
        index: u32,
        role_key: RoleKey
    },
    OutputHasRole {
        index: u32,
        role_key: RoleKey
    },
    MatchInputOutputRole {
        input_index: u32,
        input_role_key: RoleKey,
        output_index: u32,
        output_role_key: RoleKey
    },
    MatchInputOutputMetadataValue {
        input_index: u32,
        input_metadata_key: TokenMetadataKey,
        output_index: u32,
        output_metadata_key: TokenMetadataKey
    },
    FixedNumberOfInputs {
        num_inputs: u32
    },
    FixedNumberOfOutputs {
        num_outputs: u32
    },
    FixedInputMetadataValue {
        index: u32,
        metadata_key: TokenMetadataKey,
        metadata_value: TokenMetadataValue
    },
    FixedOutputMetadataValue {
        index: u32,
        metadata_key: TokenMetadataKey,
        metadata_value: TokenMetadataValue
    },
    FixedOutputMetadataValueType {
        index: u32,
        metadata_key: TokenMetadataKey,
        metadata_value_type: TokenMetadataValueDiscriminator
    }
}

impl<RoleKey, TokenMetadataKey, TokenMetadataValue, TokenMetadataValueDiscriminator> Default
    for Restriction<RoleKey, TokenMetadataKey, TokenMetadataValue, TokenMetadataValueDiscriminator>
where
    RoleKey: Parameter + Default + Ord,
    TokenMetadataKey: Parameter + Default + Ord,
    TokenMetadataValue: Parameter + Default,
    TokenMetadataValueDiscriminator: Parameter + Default + From<TokenMetadataValue>
{
    fn default() -> Self {
        Restriction::None
    }
}

pub fn validate_restriction<A, R, T, V, D>(
    restriction: Restriction<R, T, V, D>,
    sender: &A,
    inputs: &Vec<ProcessIO<A, R, T, V>>,
    outputs: &Vec<ProcessIO<A, R, T, V>>
) -> bool
where
    A: Parameter,
    R: Parameter + Default + Ord,
    T: Parameter + Default + Ord,
    V: Parameter,
    D: Parameter + From<V>
{
    match restriction {
        Restriction::<R, T, V, D>::None => true,
        // Restriction::Combined {
        //     operator,
        //     restriction_a,
        //     restriction_b
        // } => {
        //     let res_a = *restriction_a;
        //     let res_b = *restriction_b;
        //     match operator {
        //         BinaryOperator::AND => {
        //             validate_restriction(res_a, sender, inputs, outputs)
        //                 && validate_restriction(res_b, sender, inputs, outputs)
        //         }
        //         BinaryOperator::OR => {
        //             validate_restriction(res_a, sender, inputs, outputs)
        //                 || validate_restriction(res_b, sender, inputs, outputs)
        //         }
        //         BinaryOperator::XOR => {
        //             (validate_restriction(res_a, sender, inputs, outputs))
        //                 ^ (validate_restriction(res_b, sender, inputs, outputs))
        //         }
        //         BinaryOperator::NAND => {
        //             !(validate_restriction(res_a, sender, inputs, outputs)
        //                 && validate_restriction(res_b, sender, inputs, outputs))
        //         }
        //         BinaryOperator::NOR => {
        //             !(validate_restriction(res_a, sender, inputs, outputs)
        //                 || validate_restriction(res_b, sender, inputs, outputs))
        //         }
        //     }
        // }
        Restriction::FixedNumberOfInputs { num_inputs } => return inputs.len() == num_inputs as usize,
        Restriction::FixedNumberOfOutputs { num_outputs } => return outputs.len() == num_outputs as usize,
        Restriction::FixedInputMetadataValue {
            index,
            metadata_key,
            metadata_value
        } => {
            let selected_input = &inputs[index as usize];
            let meta = selected_input.metadata.get(&metadata_key);
            meta == Some(&metadata_value)
        }
        Restriction::FixedOutputMetadataValue {
            index,
            metadata_key,
            metadata_value
        } => {
            let selected_output = &outputs[index as usize];
            let meta = selected_output.metadata.get(&metadata_key);
            meta == Some(&metadata_value)
        }
        Restriction::FixedOutputMetadataValueType {
            index,
            metadata_key,
            metadata_value_type
        } => {
            let selected_output = &outputs[index as usize];
            match selected_output.metadata.get(&metadata_key) {
                Some(meta) => D::from(meta.clone()) == metadata_value_type,
                None => false
            }
        }
        Restriction::SenderHasInputRole { index, role_key } => {
            let selected_input = &inputs[index as usize];
            match selected_input.roles.get(&role_key) {
                Some(account) => sender == account,
                None => false
            }
        }
        Restriction::SenderHasOutputRole { index, role_key } => {
            let selected_output = &outputs[index as usize];
            match selected_output.roles.get(&role_key) {
                Some(account) => sender == account,
                None => false
            }
        }
        Restriction::MatchInputOutputRole {
            input_index,
            input_role_key,
            output_index,
            output_role_key
        } => {
            let selected_input = &inputs[input_index as usize];
            let selected_output = &outputs[output_index as usize];
            match (
                selected_input.roles.get(&input_role_key),
                selected_output.roles.get(&output_role_key)
            ) {
                (Some(input_account), Some(output_account)) => input_account == output_account,
                _ => false
            }
        }
        Restriction::MatchInputOutputMetadataValue {
            input_index,
            input_metadata_key,
            output_index,
            output_metadata_key
        } => {
            let selected_input = &inputs[input_index as usize];
            let selected_output = &outputs[output_index as usize];
            match (
                selected_input.metadata.get(&input_metadata_key),
                selected_output.metadata.get(&output_metadata_key)
            ) {
                (Some(input_value), Some(output_value)) => input_value == output_value,
                _ => false
            }
        }
        Restriction::OutputHasRole { index, role_key } => {
            let selected_output = &outputs[index as usize];
            selected_output.roles.get(&role_key).is_some()
        }
        Restriction::SenderOwnsAllInputs => {
            for input in inputs {
                let is_owned = match input.roles.get(&Default::default()) {
                    Some(account) => sender == account,
                    None => false
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
    use sp_std::iter::FromIterator;

    #[test]
    fn no_restriction_succeeds() {
        let result =
            validate_restriction::<u64, u32, u32, u64, u64>(Restriction::None, &1u64, &Vec::new(), &Vec::new());
        assert!(result);
    }

    #[test]
    fn sender_owns_inputs_restriction_no_inputs_succeeds() {
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::SenderOwnsAllInputs,
            &1u64,
            &Vec::new(),
            &Vec::new()
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
                parent_index: None
            },
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None
            },
        ];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::SenderOwnsAllInputs,
            &1u64,
            &inputs,
            &Vec::new()
        );
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
                parent_index: None
            },
            ProcessIO {
                roles: is_not_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None
            },
        ];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::SenderOwnsAllInputs,
            &1u64,
            &inputs,
            &Vec::new()
        );
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
                parent_index: None
            },
            ProcessIO {
                roles: is_not_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None
            },
        ];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::SenderOwnsAllInputs,
            &1u64,
            &inputs,
            &Vec::new()
        );
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
                parent_index: None
            },
            ProcessIO {
                roles: is_not_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None
            },
        ];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::SenderOwnsAllInputs,
            &1u64,
            &inputs,
            &Vec::new()
        );
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
                parent_index: None
            },
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None
            },
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None
            },
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None
            },
        ];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::FixedNumberOfInputs { num_inputs: 4 },
            &1u64,
            &inputs,
            &Vec::new()
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
                parent_index: None
            },
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None
            },
        ];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::FixedNumberOfInputs { num_inputs: 1 },
            &1u64,
            &inputs,
            &Vec::new()
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
                parent_index: None
            },
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None
            },
        ];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::FixedNumberOfOutputs { num_outputs: 2 },
            &1u64,
            &Vec::new(),
            &outputs
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
                parent_index: None
            },
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None
            },
        ];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::FixedNumberOfOutputs { num_outputs: 1 },
            &1u64,
            &Vec::new(),
            &outputs
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
                parent_index: None
            },
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None
            },
            ProcessIO {
                roles: is_owner.clone(),
                metadata: real_metadata,
                parent_index: None
            },
        ];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::FixedInputMetadataValue {
                index: 2,
                metadata_key: 2,
                metadata_value: 110
            },
            &1u64,
            &inputs,
            &Vec::new()
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
                parent_index: None
            },
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None
            },
            ProcessIO {
                roles: is_owner.clone(),
                metadata: real_metadata,
                parent_index: None
            },
        ];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::FixedInputMetadataValue {
                index: 1,
                metadata_key: 2,
                metadata_value: 110
            },
            &1u64,
            &inputs,
            &Vec::new()
        );
        assert!(!result);
    }

    #[test]
    fn fixed_metadata_value_wrong_value_under_right_key_fail() {
        let mut is_owner: BTreeMap<u32, u64> = BTreeMap::new();
        is_owner.insert(Default::default(), 1u64);
        let mut real_metadata = BTreeMap::new();
        real_metadata.insert(2, 110);
        let inputs = vec![
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None
            },
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None
            },
            ProcessIO {
                roles: is_owner.clone(),
                metadata: real_metadata,
                parent_index: None
            },
        ];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::FixedInputMetadataValue {
                index: 2,
                metadata_key: 2,
                metadata_value: 45
            },
            &1u64,
            &inputs,
            &Vec::new()
        );
        assert!(!result);
    }

    #[test]
    fn fixed_metadata_value_correct_value_under_incorrect_key_fail() {
        let mut is_owner: BTreeMap<u32, u64> = BTreeMap::new();
        is_owner.insert(Default::default(), 1u64);
        let mut real_metadata = BTreeMap::new();
        real_metadata.insert(1, 200);
        real_metadata.insert(2, 110);
        real_metadata.insert(3, 300);
        let inputs = vec![
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None
            },
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None
            },
            ProcessIO {
                roles: is_owner.clone(),
                metadata: real_metadata,
                parent_index: None
            },
        ];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::FixedInputMetadataValue {
                index: 2,
                metadata_key: 3,
                metadata_value: 110
            },
            &1u64,
            &inputs,
            &Vec::new()
        );
        assert!(!result);
    }

    #[test]
    fn fixed_metadata_value_correct_value_under_correct_key_on_wrong_input_fail() {
        let mut is_owner: BTreeMap<u32, u64> = BTreeMap::new();
        is_owner.insert(Default::default(), 1u64);
        let mut real_metadata = BTreeMap::new();
        real_metadata.insert(1, 200);
        real_metadata.insert(2, 110);
        real_metadata.insert(3, 300);
        let inputs = vec![
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None
            },
            ProcessIO {
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
                parent_index: None
            },
            ProcessIO {
                roles: is_owner.clone(),
                metadata: real_metadata,
                parent_index: None
            },
        ];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::FixedInputMetadataValue {
                index: 1,
                metadata_key: 2,
                metadata_value: 110
            },
            &1u64,
            &inputs,
            &Vec::new()
        );
        assert!(!result);
    }

    #[test]
    fn output_fixed_metadata_value_succeeds() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let outputs = vec![
            ProcessIO {
                roles: roles.clone(),
                metadata: BTreeMap::new(),
                parent_index: None
            },
            ProcessIO {
                roles: roles.clone(),
                metadata: BTreeMap::from_iter(vec![(1, 100)]),
                parent_index: None
            },
        ];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::FixedOutputMetadataValue {
                index: 1,
                metadata_key: 1,
                metadata_value: 100
            },
            &1u64,
            &Vec::new(),
            &outputs
        );
        assert!(result);
    }

    #[test]
    fn output_fixed_metadata_value_incorrect_index_correct_key_correct_value_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let outputs = vec![
            ProcessIO {
                roles: roles.clone(),
                metadata: BTreeMap::from_iter(vec![(1, 100)]),
                parent_index: None
            },
            ProcessIO {
                roles: roles.clone(),
                metadata: BTreeMap::new(),
                parent_index: None
            },
        ];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::FixedOutputMetadataValue {
                index: 1,
                metadata_key: 1,
                metadata_value: 100
            },
            &1u64,
            &Vec::new(),
            &outputs
        );
        assert!(!result);
    }

    #[test]
    fn output_fixed_metadata_value_correct_index_correct_key_incorrect_value_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let outputs = vec![
            ProcessIO {
                roles: roles.clone(),
                metadata: BTreeMap::from_iter(vec![(1, 100)]),
                parent_index: None
            },
            ProcessIO {
                roles: roles.clone(),
                metadata: BTreeMap::new(),
                parent_index: None
            },
        ];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::FixedOutputMetadataValue {
                index: 0,
                metadata_key: 1,
                metadata_value: 99
            },
            &1u64,
            &Vec::new(),
            &outputs
        );
        assert!(!result);
    }

    #[test]
    fn output_fixed_metadata_value_correct_index_incorrect_key_correct_value_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let outputs = vec![
            ProcessIO {
                roles: roles.clone(),
                metadata: BTreeMap::from_iter(vec![(1, 100)]),
                parent_index: None
            },
            ProcessIO {
                roles: roles.clone(),
                metadata: BTreeMap::new(),
                parent_index: None
            },
        ];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::FixedOutputMetadataValue {
                index: 0,
                metadata_key: 0,
                metadata_value: 100
            },
            &1u64,
            &Vec::new(),
            &outputs
        );
        assert!(!result);
    }

    #[derive(Encode, Decode, Clone, PartialEq, TypeInfo, MaxEncodedLen, Debug, Eq)]
    pub enum MetadataValue {
        A,
        B
    }
    impl Default for MetadataValue {
        fn default() -> Self {
            return MetadataValue::A;
        }
    }

    #[derive(Encode, Decode, Clone, PartialEq, TypeInfo, MaxEncodedLen, Debug, Eq)]
    pub enum MetadataValueDisc {
        AA,
        BB
    }
    impl Default for MetadataValueDisc {
        fn default() -> Self {
            return MetadataValueDisc::AA;
        }
    }

    impl From<MetadataValue> for MetadataValueDisc {
        fn from(v: MetadataValue) -> MetadataValueDisc {
            match v {
                MetadataValue::A => MetadataValueDisc::AA,
                MetadataValue::B => MetadataValueDisc::BB
            }
        }
    }

    #[test]
    fn output_restrict_metadata_type_succeeds() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let outputs = vec![
            ProcessIO {
                roles: roles.clone(),
                metadata: BTreeMap::new(),
                parent_index: None
            },
            ProcessIO {
                roles: roles.clone(),
                metadata: BTreeMap::from_iter(vec![(1, MetadataValue::A)]),
                parent_index: None
            },
        ];
        let result = validate_restriction::<u64, u32, u32, MetadataValue, MetadataValueDisc>(
            Restriction::FixedOutputMetadataValueType {
                index: 1,
                metadata_key: 1,
                metadata_value_type: MetadataValueDisc::AA
            },
            &1u64,
            &Vec::new(),
            &outputs
        );
        assert!(result);
    }

    #[test]
    fn output_restrict_metadata_type_incorrect_type_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let outputs = vec![
            ProcessIO {
                roles: roles.clone(),
                metadata: BTreeMap::new(),
                parent_index: None
            },
            ProcessIO {
                roles: roles.clone(),
                metadata: BTreeMap::from_iter(vec![(1, MetadataValue::A)]),
                parent_index: None
            },
        ];
        let result = validate_restriction::<u64, u32, u32, MetadataValue, MetadataValueDisc>(
            Restriction::FixedOutputMetadataValueType {
                index: 1,
                metadata_key: 1,
                metadata_value_type: MetadataValueDisc::BB
            },
            &1u64,
            &Vec::new(),
            &outputs
        );
        assert!(!result);
    }

    #[test]
    fn output_restrict_metadata_type_incorrect_index_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let outputs = vec![
            ProcessIO {
                roles: roles.clone(),
                metadata: BTreeMap::new(),
                parent_index: None
            },
            ProcessIO {
                roles: roles.clone(),
                metadata: BTreeMap::from_iter(vec![(1, MetadataValue::A)]),
                parent_index: None
            },
        ];
        let result = validate_restriction::<u64, u32, u32, MetadataValue, MetadataValueDisc>(
            Restriction::FixedOutputMetadataValueType {
                index: 0,
                metadata_key: 1,
                metadata_value_type: MetadataValueDisc::AA
            },
            &1u64,
            &Vec::new(),
            &outputs
        );
        assert!(!result);
    }

    #[test]
    fn output_restrict_metadata_type_incorrect_key_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let outputs = vec![
            ProcessIO {
                roles: roles.clone(),
                metadata: BTreeMap::new(),
                parent_index: None
            },
            ProcessIO {
                roles: roles.clone(),
                metadata: BTreeMap::from_iter(vec![(1, MetadataValue::A)]),
                parent_index: None
            },
        ];
        let result = validate_restriction::<u64, u32, u32, MetadataValue, MetadataValueDisc>(
            Restriction::FixedOutputMetadataValueType {
                index: 1,
                metadata_key: 0,
                metadata_value_type: MetadataValueDisc::AA
            },
            &1u64,
            &Vec::new(),
            &outputs
        );
        assert!(!result);
    }

    #[test]
    fn sender_has_input_role_succeeds() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let inputs = vec![ProcessIO {
            roles: roles.clone(),
            metadata: BTreeMap::new(),
            parent_index: None
        }];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::SenderHasInputRole {
                index: 0,
                role_key: Default::default()
            },
            &1,
            &inputs,
            &Vec::new()
        );
        assert!(result);
    }

    #[test]
    fn sender_has_input_role_incorrect_account_id_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 2)]);
        let inputs = vec![ProcessIO {
            roles: roles.clone(),
            metadata: BTreeMap::new(),
            parent_index: None
        }];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::SenderHasInputRole {
                index: 0,
                role_key: Default::default()
            },
            &1,
            &inputs,
            &Vec::new()
        );
        assert!(!result);
    }

    #[test]
    fn sender_has_input_role_incorrect_index_fails() {
        let roles0 = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let roles1 = BTreeMap::from_iter(vec![(Default::default(), 2)]);
        let inputs = vec![
            ProcessIO {
                roles: roles0.clone(),
                metadata: BTreeMap::new(),
                parent_index: None
            },
            ProcessIO {
                roles: roles1.clone(),
                metadata: BTreeMap::new(),
                parent_index: None
            },
        ];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::SenderHasInputRole {
                index: 1,
                role_key: Default::default()
            },
            &1,
            &inputs,
            &Vec::new()
        );
        assert!(!result);
    }

    #[test]
    fn sender_has_input_role_incorrect_role_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1), (0, 1), (1, 2)]);
        let inputs = vec![ProcessIO {
            roles: roles.clone(),
            metadata: BTreeMap::new(),
            parent_index: None
        }];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::SenderHasInputRole { index: 0, role_key: 1 },
            &1,
            &inputs,
            &Vec::new()
        );
        assert!(!result);
    }

    #[test]
    fn sender_has_output_role_succeeds() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let outputs = vec![ProcessIO {
            roles: roles.clone(),
            metadata: BTreeMap::new(),
            parent_index: None
        }];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::SenderHasOutputRole {
                index: 0,
                role_key: Default::default()
            },
            &1,
            &Vec::new(),
            &outputs
        );
        assert!(result);
    }

    #[test]
    fn sender_has_output_role_incorrect_account_id_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 2)]);
        let outputs = vec![ProcessIO {
            roles: roles.clone(),
            metadata: BTreeMap::new(),
            parent_index: None
        }];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::SenderHasOutputRole {
                index: 0,
                role_key: Default::default()
            },
            &1,
            &Vec::new(),
            &outputs
        );
        assert!(!result);
    }

    #[test]
    fn sender_has_output_role_incorrect_index_fails() {
        let roles0 = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let roles1 = BTreeMap::from_iter(vec![(Default::default(), 2)]);
        let outputs = vec![
            ProcessIO {
                roles: roles0.clone(),
                metadata: BTreeMap::new(),
                parent_index: None
            },
            ProcessIO {
                roles: roles1.clone(),
                metadata: BTreeMap::new(),
                parent_index: None
            },
        ];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::SenderHasOutputRole {
                index: 1,
                role_key: Default::default()
            },
            &1,
            &Vec::new(),
            &outputs
        );
        assert!(!result);
    }

    #[test]
    fn sender_has_output_role_incorrect_role_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1), (0, 1), (1, 2)]);
        let outputs = vec![ProcessIO {
            roles: roles.clone(),
            metadata: BTreeMap::new(),
            parent_index: None
        }];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::SenderHasOutputRole { index: 0, role_key: 1 },
            &1,
            &Vec::new(),
            &outputs
        );
        assert!(!result);
    }

    #[test]
    fn output_has_role_succeeds() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1), (1, 1)]);
        let outputs = vec![ProcessIO {
            roles: roles.clone(),
            metadata: BTreeMap::new(),
            parent_index: None
        }];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::OutputHasRole { index: 0, role_key: 1 },
            &1,
            &Vec::new(),
            &outputs
        );
        assert!(result);
    }

    #[test]
    fn output_has_role_incorrect_role_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1), (1, 1)]);
        let outputs = vec![ProcessIO {
            roles: roles.clone(),
            metadata: BTreeMap::new(),
            parent_index: None
        }];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::OutputHasRole { index: 0, role_key: 2 },
            &1,
            &Vec::new(),
            &outputs
        );
        assert!(!result);
    }

    #[test]
    fn output_has_role_incorrect_index_fails() {
        let roles0 = BTreeMap::from_iter(vec![(Default::default(), 1), (1, 1)]);
        let roles1 = BTreeMap::from_iter(vec![(Default::default(), 1), (2, 1)]);
        let outputs = vec![
            ProcessIO {
                roles: roles0.clone(),
                metadata: BTreeMap::new(),
                parent_index: None
            },
            ProcessIO {
                roles: roles1.clone(),
                metadata: BTreeMap::new(),
                parent_index: None
            },
        ];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::OutputHasRole { index: 1, role_key: 1 },
            &1,
            &Vec::new(),
            &outputs
        );
        assert!(!result);
    }
    #[test]
    fn match_input_output_role_same_role_keys_succeeds() {
        let input_roles = BTreeMap::from_iter(vec![(0, 1)]);
        let output_roles = BTreeMap::from_iter(vec![(0, 1)]);
        let inputs = vec![ProcessIO {
            roles: input_roles.clone(),
            metadata: BTreeMap::new(),
            parent_index: None
        }];
        let outputs = vec![ProcessIO {
            roles: output_roles.clone(),
            metadata: BTreeMap::new(),
            parent_index: None
        }];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::MatchInputOutputRole {
                input_index: 0,
                input_role_key: 0,
                output_index: 0,
                output_role_key: 0
            },
            &1,
            &inputs,
            &outputs
        );
        assert!(result);
    }

    #[test]
    fn match_input_output_role_different_role_keys_succeeds() {
        let input_roles = BTreeMap::from_iter(vec![(0, 2)]);
        let output_roles = BTreeMap::from_iter(vec![(0, 1), (1, 2)]);
        let inputs = vec![ProcessIO {
            roles: input_roles.clone(),
            metadata: BTreeMap::new(),
            parent_index: None
        }];
        let outputs = vec![ProcessIO {
            roles: output_roles.clone(),
            metadata: BTreeMap::new(),
            parent_index: None
        }];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::MatchInputOutputRole {
                input_index: 0,
                input_role_key: 0,
                output_index: 0,
                output_role_key: 1
            },
            &1,
            &inputs,
            &outputs
        );
        assert!(result);
    }

    #[test]
    fn match_input_output_role_only_one_has_key_fails() {
        let input_roles = BTreeMap::from_iter(vec![(0, 1)]);
        let output_roles = BTreeMap::from_iter(vec![(0, 1), (1, 2)]);
        let inputs = vec![ProcessIO {
            roles: input_roles.clone(),
            metadata: BTreeMap::new(),
            parent_index: None
        }];
        let outputs = vec![ProcessIO {
            roles: output_roles.clone(),
            metadata: BTreeMap::new(),
            parent_index: None
        }];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::MatchInputOutputRole {
                input_index: 0,
                input_role_key: 1,
                output_index: 0,
                output_role_key: 1
            },
            &1,
            &inputs,
            &outputs
        );
        assert!(!result);
    }

    #[test]
    fn match_input_output_role_neither_has_key_fails() {
        let input_roles = BTreeMap::from_iter(vec![(0, 1)]);
        let output_roles = BTreeMap::from_iter(vec![(0, 1)]);
        let inputs = vec![ProcessIO {
            roles: input_roles.clone(),
            metadata: BTreeMap::new(),
            parent_index: None
        }];
        let outputs = vec![ProcessIO {
            roles: output_roles.clone(),
            metadata: BTreeMap::new(),
            parent_index: None
        }];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::MatchInputOutputRole {
                input_index: 0,
                input_role_key: 1,
                output_index: 0,
                output_role_key: 1
            },
            &1,
            &inputs,
            &outputs
        );
        assert!(!result);
    }

    #[test]
    fn match_input_output_role_wrong_index_fails() {
        let input_roles0 = BTreeMap::from_iter(vec![(0, 1), (1, 2)]);
        let input_roles1 = BTreeMap::from_iter(vec![(0, 1)]);
        let output_roles = BTreeMap::from_iter(vec![(0, 1), (1, 2)]);
        let inputs = vec![
            ProcessIO {
                roles: input_roles0.clone(),
                metadata: BTreeMap::new(),
                parent_index: None
            },
            ProcessIO {
                roles: input_roles1.clone(),
                metadata: BTreeMap::new(),
                parent_index: None
            },
        ];
        let outputs = vec![ProcessIO {
            roles: output_roles.clone(),
            metadata: BTreeMap::new(),
            parent_index: None
        }];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::MatchInputOutputRole {
                input_index: 1,
                input_role_key: 1,
                output_index: 0,
                output_role_key: 1
            },
            &1,
            &inputs,
            &outputs
        );
        assert!(!result);
    }

    #[test]
    fn match_input_output_metadata_value_same_metadata_keys_succeeds() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let input_metadata = BTreeMap::from_iter(vec![(0, 0)]);
        let output_metadata = BTreeMap::from_iter(vec![(0, 0)]);
        let inputs = vec![ProcessIO {
            roles: roles.clone(),
            metadata: input_metadata.clone(),
            parent_index: None
        }];
        let outputs = vec![ProcessIO {
            roles: roles.clone(),
            metadata: output_metadata.clone(),
            parent_index: None
        }];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::MatchInputOutputMetadataValue {
                input_index: 0,
                input_metadata_key: 0,
                output_index: 0,
                output_metadata_key: 0
            },
            &1,
            &inputs,
            &outputs
        );
        assert!(result);
    }

    #[test]
    fn match_input_output_metadata_value_different_metadata_keys_succeeds() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let input_metadata = BTreeMap::from_iter(vec![(0, 0)]);
        let output_metadata = BTreeMap::from_iter(vec![(1, 0)]);
        let inputs = vec![ProcessIO {
            roles: roles.clone(),
            metadata: input_metadata.clone(),
            parent_index: None
        }];
        let outputs = vec![ProcessIO {
            roles: roles.clone(),
            metadata: output_metadata.clone(),
            parent_index: None
        }];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::MatchInputOutputMetadataValue {
                input_index: 0,
                input_metadata_key: 0,
                output_index: 0,
                output_metadata_key: 1
            },
            &1,
            &inputs,
            &outputs
        );
        assert!(result);
    }

    #[test]
    fn match_input_output_metadata_value_only_one_has_key_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let input_metadata = BTreeMap::from_iter(vec![(0, 0)]);
        let output_metadata = BTreeMap::new();
        let inputs = vec![ProcessIO {
            roles: roles.clone(),
            metadata: input_metadata.clone(),
            parent_index: None
        }];
        let outputs = vec![ProcessIO {
            roles: roles.clone(),
            metadata: output_metadata.clone(),
            parent_index: None
        }];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::MatchInputOutputMetadataValue {
                input_index: 0,
                input_metadata_key: 0,
                output_index: 0,
                output_metadata_key: 0
            },
            &1,
            &inputs,
            &outputs
        );
        assert!(!result);
    }

    #[test]
    fn match_input_output_metadata_value_neither_has_key_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let input_metadata = BTreeMap::new();
        let output_metadata = BTreeMap::new();
        let inputs = vec![ProcessIO {
            roles: roles.clone(),
            metadata: input_metadata.clone(),
            parent_index: None
        }];
        let outputs = vec![ProcessIO {
            roles: roles.clone(),
            metadata: output_metadata.clone(),
            parent_index: None
        }];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::MatchInputOutputMetadataValue {
                input_index: 0,
                input_metadata_key: 0,
                output_index: 0,
                output_metadata_key: 0
            },
            &1,
            &inputs,
            &outputs
        );
        assert!(!result);
    }

    #[test]
    fn match_input_output_metadata_value_wrong_index_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let input_metadata0 = BTreeMap::from_iter(vec![(0, 0)]);
        let input_metadata1 = BTreeMap::new();
        let output_metadata = BTreeMap::from_iter(vec![(0, 0)]);
        let inputs = vec![
            ProcessIO {
                roles: roles.clone(),
                metadata: input_metadata0.clone(),
                parent_index: None
            },
            ProcessIO {
                roles: roles.clone(),
                metadata: input_metadata1.clone(),
                parent_index: None
            },
        ];
        let outputs = vec![ProcessIO {
            roles: roles.clone(),
            metadata: output_metadata.clone(),
            parent_index: None
        }];
        let result = validate_restriction::<u64, u32, u32, u64, u64>(
            Restriction::MatchInputOutputMetadataValue {
                input_index: 1,
                input_metadata_key: 0,
                output_index: 0,
                output_metadata_key: 0
            },
            &1,
            &inputs,
            &outputs
        );
        assert!(!result);
    }

    // #[test]
    // fn combined_and_succeeds() {
    //     let outputs = vec![ProcessIO {
    //         roles: BTreeMap::from_iter(vec![(Default::default(), 1)]),
    //         metadata: BTreeMap::from_iter(vec![(0, 0), (1, 1)]),
    //         parent_index: None
    //     }];
    //     let result = validate_restriction::<u64, u32, u32, u64, u64>(
    //         Restriction::Combined {
    //             operator: BinaryOperator::AND,
    //             restriction_a: {
    //                 Box::new(Restriction::FixedOutputMetadataValue {
    //                     index: 0,
    //                     metadata_key: 0,
    //                     metadata_value: 0
    //                 })
    //             },
    //             restriction_b: {
    //                 Box::new(Restriction::FixedOutputMetadataValue {
    //                     index: 0,
    //                     metadata_key: 1,
    //                     metadata_value: 1
    //                 })
    //             }
    //         },
    //         &1,
    //         &Vec::new(),
    //         &outputs
    //     );
    //     assert!(result);
    // }

    // #[test]
    // fn combined_and_fails() {
    //     let outputs = vec![ProcessIO {
    //         roles: BTreeMap::from_iter(vec![(Default::default(), 1)]),
    //         metadata: BTreeMap::from_iter(vec![(0, 0)]),
    //         parent_index: None
    //     }];
    //     let result = validate_restriction::<u64, u32, u32, u64, u64>(
    //         Restriction::Combined {
    //             operator: BinaryOperator::AND,
    //             restriction_a: {
    //                 Box::new(Restriction::FixedOutputMetadataValue {
    //                     index: 0,
    //                     metadata_key: 0,
    //                     metadata_value: 0
    //                 })
    //             },
    //             restriction_b: {
    //                 Box::new(Restriction::FixedOutputMetadataValue {
    //                     index: 0,
    //                     metadata_key: 1,
    //                     metadata_value: 1
    //                 })
    //             }
    //         },
    //         &1,
    //         &Vec::new(),
    //         &outputs
    //     );
    //     assert!(!result);
    // }

    // #[test]
    // fn combined_or_succeeds() {
    //     let outputs = vec![ProcessIO {
    //         roles: BTreeMap::from_iter(vec![(Default::default(), 1)]),
    //         metadata: BTreeMap::from_iter(vec![(0, 0)]),
    //         parent_index: None
    //     }];
    //     let result = validate_restriction::<u64, u32, u32, u64, u64>(
    //         Restriction::Combined {
    //             operator: BinaryOperator::OR,
    //             restriction_a: {
    //                 Box::new(Restriction::FixedOutputMetadataValue {
    //                     index: 0,
    //                     metadata_key: 0,
    //                     metadata_value: 0
    //                 })
    //             },
    //             restriction_b: {
    //                 Box::new(Restriction::FixedOutputMetadataValue {
    //                     index: 0,
    //                     metadata_key: 1,
    //                     metadata_value: 1
    //                 })
    //             }
    //         },
    //         &1,
    //         &Vec::new(),
    //         &outputs
    //     );
    //     assert!(result);
    // }

    // #[test]
    // fn combined_or_fails() {
    //     let outputs = vec![ProcessIO {
    //         roles: BTreeMap::from_iter(vec![(Default::default(), 1)]),
    //         metadata: BTreeMap::from_iter(vec![(0, 0)]),
    //         parent_index: None
    //     }];
    //     let result = validate_restriction::<u64, u32, u32, u64, u64>(
    //         Restriction::Combined {
    //             operator: BinaryOperator::OR,
    //             restriction_a: {
    //                 Box::new(Restriction::FixedOutputMetadataValue {
    //                     index: 0,
    //                     metadata_key: 1,
    //                     metadata_value: 1
    //                 })
    //             },
    //             restriction_b: {
    //                 Box::new(Restriction::FixedOutputMetadataValue {
    //                     index: 0,
    //                     metadata_key: 2,
    //                     metadata_value: 2
    //                 })
    //             }
    //         },
    //         &1,
    //         &Vec::new(),
    //         &outputs
    //     );
    //     assert!(!result);
    // }

    // #[test]
    // fn combined_xor_succeeds() {
    //     let outputs = vec![ProcessIO {
    //         roles: BTreeMap::from_iter(vec![(Default::default(), 1)]),
    //         metadata: BTreeMap::from_iter(vec![(0, 0)]),
    //         parent_index: None
    //     }];
    //     let result = validate_restriction::<u64, u32, u32, u64, u64>(
    //         Restriction::Combined {
    //             operator: BinaryOperator::XOR,
    //             restriction_a: {
    //                 Box::new(Restriction::FixedOutputMetadataValue {
    //                     index: 0,
    //                     metadata_key: 0,
    //                     metadata_value: 0
    //                 })
    //             },
    //             restriction_b: {
    //                 Box::new(Restriction::FixedOutputMetadataValue {
    //                     index: 0,
    //                     metadata_key: 1,
    //                     metadata_value: 1
    //                 })
    //             }
    //         },
    //         &1,
    //         &Vec::new(),
    //         &outputs
    //     );
    //     assert!(result);
    // }

    // #[test]
    // fn combined_xor_fails() {
    //     let outputs = vec![ProcessIO {
    //         roles: BTreeMap::from_iter(vec![(Default::default(), 1)]),
    //         metadata: BTreeMap::from_iter(vec![(0, 0), (1, 1)]),
    //         parent_index: None
    //     }];
    //     let result = validate_restriction::<u64, u32, u32, u64, u64>(
    //         Restriction::Combined {
    //             operator: BinaryOperator::XOR,
    //             restriction_a: {
    //                 Box::new(Restriction::FixedOutputMetadataValue {
    //                     index: 0,
    //                     metadata_key: 0,
    //                     metadata_value: 0
    //                 })
    //             },
    //             restriction_b: {
    //                 Box::new(Restriction::FixedOutputMetadataValue {
    //                     index: 0,
    //                     metadata_key: 1,
    //                     metadata_value: 1
    //                 })
    //             }
    //         },
    //         &1,
    //         &Vec::new(),
    //         &outputs
    //     );
    //     assert!(!result);
    // }

    // #[test]
    // fn combined_nand_succeeds() {
    //     let outputs = vec![ProcessIO {
    //         roles: BTreeMap::from_iter(vec![(Default::default(), 1)]),
    //         metadata: BTreeMap::new(),
    //         parent_index: None
    //     }];
    //     let result = validate_restriction::<u64, u32, u32, u64, u64>(
    //         Restriction::Combined {
    //             operator: BinaryOperator::NAND,
    //             restriction_a: {
    //                 Box::new(Restriction::FixedOutputMetadataValue {
    //                     index: 0,
    //                     metadata_key: 0,
    //                     metadata_value: 0
    //                 })
    //             },
    //             restriction_b: {
    //                 Box::new(Restriction::FixedOutputMetadataValue {
    //                     index: 0,
    //                     metadata_key: 1,
    //                     metadata_value: 1
    //                 })
    //             }
    //         },
    //         &1,
    //         &Vec::new(),
    //         &outputs
    //     );
    //     assert!(result);
    // }

    // #[test]
    // fn combined_nand_fails() {
    //     let outputs = vec![ProcessIO {
    //         roles: BTreeMap::from_iter(vec![(Default::default(), 1)]),
    //         metadata: BTreeMap::from_iter(vec![(0, 0), (1, 1)]),
    //         parent_index: None
    //     }];
    //     let result = validate_restriction::<u64, u32, u32, u64, u64>(
    //         Restriction::Combined {
    //             operator: BinaryOperator::NAND,
    //             restriction_a: {
    //                 Box::new(Restriction::FixedOutputMetadataValue {
    //                     index: 0,
    //                     metadata_key: 0,
    //                     metadata_value: 0
    //                 })
    //             },
    //             restriction_b: {
    //                 Box::new(Restriction::FixedOutputMetadataValue {
    //                     index: 0,
    //                     metadata_key: 1,
    //                     metadata_value: 1
    //                 })
    //             }
    //         },
    //         &1,
    //         &Vec::new(),
    //         &outputs
    //     );
    //     assert!(!result);
    // }

    // #[test]
    // fn combined_nor_succeeds() {
    //     let outputs = vec![ProcessIO {
    //         roles: BTreeMap::from_iter(vec![(Default::default(), 1)]),
    //         metadata: BTreeMap::new(),
    //         parent_index: None
    //     }];
    //     let result = validate_restriction::<u64, u32, u32, u64, u64>(
    //         Restriction::Combined {
    //             operator: BinaryOperator::NOR,
    //             restriction_a: {
    //                 Box::new(Restriction::FixedOutputMetadataValue {
    //                     index: 0,
    //                     metadata_key: 0,
    //                     metadata_value: 0
    //                 })
    //             },
    //             restriction_b: {
    //                 Box::new(Restriction::FixedOutputMetadataValue {
    //                     index: 0,
    //                     metadata_key: 1,
    //                     metadata_value: 1
    //                 })
    //             }
    //         },
    //         &1,
    //         &Vec::new(),
    //         &outputs
    //     );
    //     assert!(result);
    // }

    // #[test]
    // fn combined_nor_fails() {
    //     let outputs = vec![ProcessIO {
    //         roles: BTreeMap::from_iter(vec![(Default::default(), 1)]),
    //         metadata: BTreeMap::from_iter(vec![(0, 0)]),
    //         parent_index: None
    //     }];
    //     let result = validate_restriction::<u64, u32, u32, u64, u64>(
    //         Restriction::Combined {
    //             operator: BinaryOperator::NOR,
    //             restriction_a: {
    //                 Box::new(Restriction::FixedOutputMetadataValue {
    //                     index: 0,
    //                     metadata_key: 0,
    //                     metadata_value: 0
    //                 })
    //             },
    //             restriction_b: {
    //                 Box::new(Restriction::FixedOutputMetadataValue {
    //                     index: 0,
    //                     metadata_key: 1,
    //                     metadata_value: 1
    //                 })
    //             }
    //         },
    //         &1,
    //         &Vec::new(),
    //         &outputs
    //     );
    //     assert!(!result);
    // }
}
