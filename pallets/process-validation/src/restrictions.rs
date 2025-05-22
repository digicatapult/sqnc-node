// This file contains the different types of restrictions that can be evaluated during
// a call to `validate_process`

use frame_support::Parameter;
use frame_system::RawOrigin;
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_std::vec::Vec;
use sqnc_pallet_traits::ProcessIO;

#[derive(Copy, Encode, Decode, Debug, Clone, MaxEncodedLen, TypeInfo, PartialEq, Serialize, Deserialize)]
pub enum ArgType {
    Input,
    Output,
    Reference,
}

impl Default for ArgType {
    fn default() -> Self {
        ArgType::Input
    }
}

#[derive(Encode, Decode, Debug, Clone, MaxEncodedLen, TypeInfo, PartialEq, Serialize, Deserialize)]
pub enum Restriction<RoleKey, TokenMetadataKey, TokenMetadataValue, TokenMetadataValueDiscriminator> {
    None,
    Fail,
    SenderIsRoot,
    SenderHasArgRole {
        arg_type: ArgType,
        index: u32,
        role_key: RoleKey,
    },
    ArgHasRole {
        arg_type: ArgType,
        index: u32,
        role_key: RoleKey,
    },
    ArgHasMetadata {
        arg_type: ArgType,
        index: u32,
        metadata_key: TokenMetadataKey,
    },
    MatchArgsRole {
        left_arg_type: ArgType,
        left_index: u32,
        left_role_key: RoleKey,
        right_arg_type: ArgType,
        right_index: u32,
        right_role_key: RoleKey,
    },
    MatchArgsMetadataValue {
        left_arg_type: ArgType,
        left_index: u32,
        left_metadata_key: TokenMetadataKey,
        right_arg_type: ArgType,
        right_index: u32,
        right_metadata_key: TokenMetadataKey,
    },
    MatchArgIdToMetadataValue {
        left_arg_type: ArgType,
        left_index: u32,
        right_arg_type: ArgType,
        right_index: u32,
        right_metadata_key: TokenMetadataKey,
    },
    FixedArgCount {
        arg_type: ArgType,
        count: u32,
    },
    FixedArgMetadataValue {
        arg_type: ArgType,
        index: u32,
        metadata_key: TokenMetadataKey,
        metadata_value: TokenMetadataValue,
    },
    FixedArgMetadataValueType {
        arg_type: ArgType,
        index: u32,
        metadata_key: TokenMetadataKey,
        metadata_value_type: TokenMetadataValueDiscriminator,
    },
}

impl<RoleKey, TokenMetadataKey, TokenMetadataValue, TokenMetadataValueDiscriminator> Default
    for Restriction<RoleKey, TokenMetadataKey, TokenMetadataValue, TokenMetadataValueDiscriminator>
where
    RoleKey: Parameter + Default + Ord,
    TokenMetadataKey: Parameter + Default + Ord,
    TokenMetadataValue: Parameter + Default,
    TokenMetadataValueDiscriminator: Parameter + Default + From<TokenMetadataValue>,
{
    fn default() -> Self {
        Restriction::None
    }
}

pub fn validate_restriction<'a, I, A, R, T, V, D, F>(
    restriction: Restriction<R, T, V, D>,
    origin: &RawOrigin<A>,
    get_args: F,
) -> bool
where
    I: 'a,
    A: Parameter + 'a,
    R: Parameter + Default + Ord + 'a,
    T: Parameter + Default + Ord + 'a,
    V: Parameter + PartialEq<I> + 'a,
    D: Parameter + From<V> + 'a,
    F: Fn(ArgType) -> &'a Vec<ProcessIO<I, A, R, T, V>>,
{
    match restriction {
        Restriction::None => true,
        Restriction::Fail => false,
        Restriction::SenderIsRoot => match origin {
            RawOrigin::Root => true,
            _ => false,
        },
        Restriction::FixedArgCount { arg_type, count } => get_args(arg_type).len() == count as usize,
        Restriction::FixedArgMetadataValue {
            arg_type,
            index,
            metadata_key,
            metadata_value,
        } => {
            let args = get_args(arg_type);
            let Some(arg) = args.get(index as usize) else {
                return false;
            };
            let meta = arg.metadata.get(&metadata_key);
            meta == Some(&metadata_value)
        }
        Restriction::FixedArgMetadataValueType {
            arg_type,
            index,
            metadata_key,
            metadata_value_type,
        } => {
            let args = get_args(arg_type);
            let Some(arg) = args.get(index as usize) else {
                return false;
            };
            match arg.metadata.get(&metadata_key) {
                Some(meta) => D::from(meta.clone()) == metadata_value_type,
                None => false,
            }
        }
        Restriction::SenderHasArgRole {
            arg_type,
            index,
            role_key,
        } => {
            let args = get_args(arg_type);
            let Some(arg) = args.get(index as usize) else {
                return false;
            };
            match arg.roles.get(&role_key) {
                Some(account) => match origin {
                    RawOrigin::Signed(acc) => acc == account,
                    _ => false,
                },
                None => false,
            }
        }
        Restriction::MatchArgsRole {
            left_arg_type,
            left_index,
            left_role_key,
            right_arg_type,
            right_index,
            right_role_key,
        } => {
            let left_args = get_args(left_arg_type);
            let right_args = get_args(right_arg_type);
            let (Some(left), Some(right)) = (left_args.get(left_index as usize), right_args.get(right_index as usize))
            else {
                return false;
            };
            match (left.roles.get(&left_role_key), right.roles.get(&right_role_key)) {
                (Some(left_account), Some(right_account)) => left_account == right_account,
                _ => false,
            }
        }
        Restriction::MatchArgsMetadataValue {
            left_arg_type,
            left_index,
            left_metadata_key,
            right_arg_type,
            right_index,
            right_metadata_key,
        } => {
            let left_args = get_args(left_arg_type);
            let right_args = get_args(right_arg_type);
            let (Some(left_arg), Some(right_arg)) =
                (left_args.get(left_index as usize), right_args.get(right_index as usize))
            else {
                return false;
            };
            match (
                left_arg.metadata.get(&left_metadata_key),
                right_arg.metadata.get(&right_metadata_key),
            ) {
                (Some(input_value), Some(output_value)) => input_value == output_value,
                _ => false,
            }
        }
        Restriction::MatchArgIdToMetadataValue {
            left_arg_type,
            left_index,
            right_arg_type,
            right_index,
            right_metadata_key,
        } => {
            let left_args = get_args(left_arg_type);
            let right_args = get_args(right_arg_type);
            let (Some(left_arg), Some(right_arg)) =
                (left_args.get(left_index as usize), right_args.get(right_index as usize))
            else {
                return false;
            };

            match right_arg.metadata.get(&right_metadata_key) {
                Some(v) => v == &left_arg.id,
                _ => false,
            }
        }
        Restriction::ArgHasRole {
            arg_type,
            index,
            role_key,
        } => {
            let args = get_args(arg_type);
            let Some(arg) = args.get(index as usize) else {
                return false;
            };
            arg.roles.get(&role_key).is_some()
        }
        Restriction::ArgHasMetadata {
            arg_type,
            index,
            metadata_key,
        } => {
            let args = get_args(arg_type);
            let Some(arg) = args.get(index as usize) else {
                return false;
            };
            arg.metadata.get(&metadata_key).is_some()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sp_std::collections::btree_map::BTreeMap;
    use sp_std::iter::FromIterator;

    type TestProcess = ProcessIO<u64, u64, u32, u32, u64>;
    static EMPTY_ARGS: Vec<TestProcess> = Vec::<TestProcess>::new();

    #[test]
    fn none_restriction_succeeds() {
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::None,
            &RawOrigin::Signed(1u64),
            |_| &EMPTY_ARGS,
        );
        assert!(result);
    }

    #[test]
    fn fail_restriction_fails() {
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::Fail,
            &RawOrigin::Signed(1u64),
            |_| &EMPTY_ARGS,
        );
        assert!(!result);
    }

    #[test]
    fn sender_is_root_succeeds_as_root() {
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::SenderIsRoot,
            &RawOrigin::Root,
            |_| &EMPTY_ARGS,
        );
        assert!(result);
    }

    #[test]
    fn sender_is_root_fails_as_none() {
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::SenderIsRoot,
            &RawOrigin::None,
            |_| &EMPTY_ARGS,
        );
        assert!(!result);
    }

    #[test]
    fn sender_is_root_fails_as_signed() {
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::SenderIsRoot,
            &RawOrigin::Signed(1u64),
            |_| &EMPTY_ARGS,
        );
        assert!(!result);
    }

    #[test]
    fn fixed_number_of_inputs_restriction_matches_fixed_input_total() {
        let mut is_owner: BTreeMap<u32, u64> = BTreeMap::new();
        is_owner.insert(Default::default(), 1u64);
        let inputs = vec![
            ProcessIO {
                id: 0u64,
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
            },
            ProcessIO {
                id: 0u64,
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
            },
            ProcessIO {
                id: 0u64,
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
            },
            ProcessIO {
                id: 0u64,
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
            },
        ];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::FixedArgCount {
                arg_type: ArgType::Input,
                count: 4,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(result);
    }

    #[test]
    fn fixed_number_of_inputs_restriction_matches_fixed_input_total_fail() {
        let mut is_owner: BTreeMap<u32, u64> = BTreeMap::new();
        is_owner.insert(Default::default(), 1u64);
        let inputs = vec![
            ProcessIO {
                id: 0u64,
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
            },
            ProcessIO {
                id: 0u64,
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
            },
        ];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::FixedArgCount {
                arg_type: ArgType::Input,
                count: 1,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn fixed_number_of_outputs_restriction_matches_fixed_output_total() {
        let mut is_owner: BTreeMap<u32, u64> = BTreeMap::new();
        is_owner.insert(Default::default(), 1u64);
        let outputs = vec![
            ProcessIO {
                id: 0u64,
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
            },
            ProcessIO {
                id: 0u64,
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
            },
        ];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::FixedArgCount {
                arg_type: ArgType::Output,
                count: 2,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(result);
    }

    #[test]
    fn fixed_number_of_output_restriction_matches_fixed_output_total_fail() {
        let mut is_owner: BTreeMap<u32, u64> = BTreeMap::new();
        is_owner.insert(Default::default(), 1u64);
        let outputs = vec![
            ProcessIO {
                id: 0u64,
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
            },
            ProcessIO {
                id: 0u64,
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
            },
        ];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::FixedArgCount {
                arg_type: ArgType::Output,
                count: 1,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn fixed_input_metadata_value_succeeds() {
        let mut is_owner: BTreeMap<u32, u64> = BTreeMap::new();
        is_owner.insert(Default::default(), 1u64);
        let mut real_metadata = BTreeMap::new();
        real_metadata.insert(2, 110);
        let inputs = vec![
            ProcessIO {
                id: 0u64,
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
            },
            ProcessIO {
                id: 0u64,
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
            },
            ProcessIO {
                id: 0u64,
                roles: is_owner.clone(),
                metadata: real_metadata,
            },
        ];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::FixedArgMetadataValue {
                arg_type: ArgType::Input,
                index: 2,
                metadata_key: 2,
                metadata_value: 110,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(result);
    }

    #[test]
    fn fixed_input_metadata_value_missing_value_fail() {
        let mut is_owner: BTreeMap<u32, u64> = BTreeMap::new();
        is_owner.insert(Default::default(), 1u64);
        let mut real_metadata = BTreeMap::new();
        real_metadata.insert(2, 110);
        let inputs = vec![
            ProcessIO {
                id: 0u64,
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
            },
            ProcessIO {
                id: 0u64,
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
            },
            ProcessIO {
                id: 0u64,
                roles: is_owner.clone(),
                metadata: real_metadata,
            },
        ];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::FixedArgMetadataValue {
                arg_type: ArgType::Input,
                index: 1,
                metadata_key: 2,
                metadata_value: 110,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn fixed_input_metadata_value_wrong_value_under_right_key_fail() {
        let mut is_owner: BTreeMap<u32, u64> = BTreeMap::new();
        is_owner.insert(Default::default(), 1u64);
        let mut real_metadata = BTreeMap::new();
        real_metadata.insert(2, 110);
        let inputs = vec![
            ProcessIO {
                id: 0u64,
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
            },
            ProcessIO {
                id: 0u64,
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
            },
            ProcessIO {
                id: 0u64,
                roles: is_owner.clone(),
                metadata: real_metadata,
            },
        ];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::FixedArgMetadataValue {
                arg_type: ArgType::Input,
                index: 2,
                metadata_key: 2,
                metadata_value: 45,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn fixed_input_metadata_value_correct_value_under_incorrect_key_fail() {
        let mut is_owner: BTreeMap<u32, u64> = BTreeMap::new();
        is_owner.insert(Default::default(), 1u64);
        let mut real_metadata = BTreeMap::new();
        real_metadata.insert(1, 200);
        real_metadata.insert(2, 110);
        real_metadata.insert(3, 300);
        let inputs = vec![
            ProcessIO {
                id: 0u64,
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
            },
            ProcessIO {
                id: 0u64,
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
            },
            ProcessIO {
                id: 0u64,
                roles: is_owner.clone(),
                metadata: real_metadata,
            },
        ];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::FixedArgMetadataValue {
                arg_type: ArgType::Input,
                index: 2,
                metadata_key: 3,
                metadata_value: 110,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn fixed_input_metadata_value_correct_value_under_correct_key_on_wrong_input_fail() {
        let mut is_owner: BTreeMap<u32, u64> = BTreeMap::new();
        is_owner.insert(Default::default(), 1u64);
        let mut real_metadata = BTreeMap::new();
        real_metadata.insert(1, 200);
        real_metadata.insert(2, 110);
        real_metadata.insert(3, 300);
        let inputs = vec![
            ProcessIO {
                id: 0u64,
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
            },
            ProcessIO {
                id: 0u64,
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
            },
            ProcessIO {
                id: 0u64,
                roles: is_owner.clone(),
                metadata: real_metadata,
            },
        ];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::FixedArgMetadataValue {
                arg_type: ArgType::Input,
                index: 3,
                metadata_key: 2,
                metadata_value: 110,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn fixed_input_metadata_value_bad_index_fail() {
        let mut is_owner: BTreeMap<u32, u64> = BTreeMap::new();
        is_owner.insert(Default::default(), 1u64);
        let mut real_metadata = BTreeMap::new();
        real_metadata.insert(1, 200);
        real_metadata.insert(2, 110);
        real_metadata.insert(3, 300);
        let inputs = vec![
            ProcessIO {
                id: 0u64,
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
            },
            ProcessIO {
                id: 0u64,
                roles: is_owner.clone(),
                metadata: BTreeMap::new(),
            },
            ProcessIO {
                id: 0u64,
                roles: is_owner.clone(),
                metadata: real_metadata,
            },
        ];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::FixedArgMetadataValue {
                arg_type: ArgType::Input,
                index: 1,
                metadata_key: 2,
                metadata_value: 110,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn fixed_output_metadata_value_succeeds() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let outputs = vec![
            ProcessIO {
                id: 0u64,
                roles: roles.clone(),
                metadata: BTreeMap::new(),
            },
            ProcessIO {
                id: 0u64,
                roles: roles.clone(),
                metadata: BTreeMap::from_iter(vec![(1, 100)]),
            },
        ];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::FixedArgMetadataValue {
                arg_type: ArgType::Output,
                index: 1,
                metadata_key: 1,
                metadata_value: 100,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(result);
    }

    #[test]
    fn fixed_output_metadata_value_incorrect_index_correct_key_correct_value_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let outputs = vec![
            ProcessIO {
                id: 0u64,
                roles: roles.clone(),
                metadata: BTreeMap::from_iter(vec![(1, 100)]),
            },
            ProcessIO {
                id: 0u64,
                roles: roles.clone(),
                metadata: BTreeMap::new(),
            },
        ];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::FixedArgMetadataValue {
                arg_type: ArgType::Output,
                index: 1,
                metadata_key: 1,
                metadata_value: 100,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn fixed_output_metadata_value_correct_index_correct_key_incorrect_value_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let outputs = vec![
            ProcessIO {
                id: 0u64,
                roles: roles.clone(),
                metadata: BTreeMap::from_iter(vec![(1, 100)]),
            },
            ProcessIO {
                id: 0u64,
                roles: roles.clone(),
                metadata: BTreeMap::new(),
            },
        ];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::FixedArgMetadataValue {
                arg_type: ArgType::Output,
                index: 0,
                metadata_key: 1,
                metadata_value: 99,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn foxed_output_metadata_value_correct_index_incorrect_key_correct_value_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let outputs = vec![
            ProcessIO {
                id: 0u64,
                roles: roles.clone(),
                metadata: BTreeMap::from_iter(vec![(1, 100)]),
            },
            ProcessIO {
                id: 0u64,
                roles: roles.clone(),
                metadata: BTreeMap::new(),
            },
        ];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::FixedArgMetadataValue {
                arg_type: ArgType::Output,
                index: 0,
                metadata_key: 0,
                metadata_value: 100,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn fixed_output_metadata_value_bad_index_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let outputs = vec![
            ProcessIO {
                id: 0u64,
                roles: roles.clone(),
                metadata: BTreeMap::from_iter(vec![(1, 100)]),
            },
            ProcessIO {
                id: 0u64,
                roles: roles.clone(),
                metadata: BTreeMap::new(),
            },
        ];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::FixedArgMetadataValue {
                arg_type: ArgType::Output,
                index: 2,
                metadata_key: 1,
                metadata_value: 100,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[derive(Encode, Decode, Clone, PartialEq, TypeInfo, MaxEncodedLen, Debug, Eq)]
    pub enum MetadataValue {
        A,
        B,
    }
    impl Default for MetadataValue {
        fn default() -> Self {
            return MetadataValue::A;
        }
    }

    impl PartialEq<u64> for MetadataValue {
        fn eq(&self, _: &u64) -> bool {
            false
        }
    }

    #[derive(Encode, Decode, Clone, PartialEq, TypeInfo, MaxEncodedLen, Debug, Eq)]
    pub enum MetadataValueDisc {
        AA,
        BB,
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
                MetadataValue::B => MetadataValueDisc::BB,
            }
        }
    }

    #[test]
    fn output_restrict_metadata_type_succeeds() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let other_args = Vec::new();
        let outputs = vec![
            ProcessIO {
                id: 0u64,
                roles: roles.clone(),
                metadata: BTreeMap::new(),
            },
            ProcessIO {
                id: 0u64,
                roles: roles.clone(),
                metadata: BTreeMap::from_iter(vec![(1, MetadataValue::A)]),
            },
        ];
        let result = validate_restriction(
            Restriction::<u32, u32, MetadataValue, MetadataValueDisc>::FixedArgMetadataValueType {
                arg_type: ArgType::Output,
                index: 1,
                metadata_key: 1,
                metadata_value_type: MetadataValueDisc::AA,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                _ => &other_args,
            },
        );
        assert!(result);
    }

    #[test]
    fn output_restrict_metadata_type_incorrect_type_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let other_args = Vec::new();
        let outputs = vec![
            ProcessIO {
                id: 0u64,
                roles: roles.clone(),
                metadata: BTreeMap::new(),
            },
            ProcessIO {
                id: 0u64,
                roles: roles.clone(),
                metadata: BTreeMap::from_iter(vec![(1, MetadataValue::A)]),
            },
        ];
        let result = validate_restriction(
            Restriction::<u32, u32, MetadataValue, MetadataValueDisc>::FixedArgMetadataValueType {
                arg_type: ArgType::Output,
                index: 1,
                metadata_key: 1,
                metadata_value_type: MetadataValueDisc::BB,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                _ => &other_args,
            },
        );
        assert!(!result);
    }

    #[test]
    fn output_restrict_metadata_type_incorrect_index_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let other_args = Vec::new();
        let outputs = vec![
            ProcessIO {
                id: 0u64,
                roles: roles.clone(),
                metadata: BTreeMap::new(),
            },
            ProcessIO {
                id: 0u64,
                roles: roles.clone(),
                metadata: BTreeMap::from_iter(vec![(1, MetadataValue::A)]),
            },
        ];
        let result = validate_restriction(
            Restriction::<u32, u32, MetadataValue, MetadataValueDisc>::FixedArgMetadataValueType {
                arg_type: ArgType::Output,
                index: 0,
                metadata_key: 1,
                metadata_value_type: MetadataValueDisc::AA,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                _ => &other_args,
            },
        );
        assert!(!result);
    }

    #[test]
    fn output_restrict_metadata_type_incorrect_key_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let other_args = Vec::new();
        let outputs = vec![
            ProcessIO {
                id: 0u64,
                roles: roles.clone(),
                metadata: BTreeMap::new(),
            },
            ProcessIO {
                id: 0u64,
                roles: roles.clone(),
                metadata: BTreeMap::from_iter(vec![(1, MetadataValue::A)]),
            },
        ];
        let result = validate_restriction(
            Restriction::<u32, u32, MetadataValue, MetadataValueDisc>::FixedArgMetadataValueType {
                arg_type: ArgType::Output,
                index: 1,
                metadata_key: 0,
                metadata_value_type: MetadataValueDisc::AA,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                _ => &other_args,
            },
        );
        assert!(!result);
    }

    #[test]
    fn output_restrict_metadata_type_bad_index_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let other_args = Vec::new();
        let outputs = vec![
            ProcessIO {
                id: 0u64,
                roles: roles.clone(),
                metadata: BTreeMap::new(),
            },
            ProcessIO {
                id: 0u64,
                roles: roles.clone(),
                metadata: BTreeMap::from_iter(vec![(1, MetadataValue::A)]),
            },
        ];
        let result = validate_restriction(
            Restriction::<u32, u32, MetadataValue, MetadataValueDisc>::FixedArgMetadataValueType {
                arg_type: ArgType::Output,
                index: 2,
                metadata_key: 0,
                metadata_value_type: MetadataValueDisc::AA,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                _ => &other_args,
            },
        );
        assert!(!result);
    }

    #[test]
    fn input_restrict_metadata_type_succeeds() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let other_args = Vec::new();
        let inputs = vec![
            ProcessIO {
                id: 0u64,
                roles: roles.clone(),
                metadata: BTreeMap::new(),
            },
            ProcessIO {
                id: 0u64,
                roles: roles.clone(),
                metadata: BTreeMap::from_iter(vec![(1, MetadataValue::A)]),
            },
        ];
        let result = validate_restriction(
            Restriction::<u32, u32, MetadataValue, MetadataValueDisc>::FixedArgMetadataValueType {
                arg_type: ArgType::Input,
                index: 1,
                metadata_key: 1,
                metadata_value_type: MetadataValueDisc::AA,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Input => &inputs,
                _ => &other_args,
            },
        );
        assert!(result);
    }

    #[test]
    fn input_restrict_metadata_type_incorrect_type_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let other_args = Vec::new();
        let inputs = vec![
            ProcessIO {
                id: 0u64,
                roles: roles.clone(),
                metadata: BTreeMap::new(),
            },
            ProcessIO {
                id: 0u64,
                roles: roles.clone(),
                metadata: BTreeMap::from_iter(vec![(1, MetadataValue::A)]),
            },
        ];
        let result = validate_restriction(
            Restriction::<u32, u32, MetadataValue, MetadataValueDisc>::FixedArgMetadataValueType {
                arg_type: ArgType::Input,
                index: 1,
                metadata_key: 1,
                metadata_value_type: MetadataValueDisc::BB,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Input => &inputs,
                _ => &other_args,
            },
        );
        assert!(!result);
    }

    #[test]
    fn input_restrict_metadata_type_incorrect_index_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let other_args = Vec::new();
        let inputs = vec![
            ProcessIO {
                id: 0u64,
                roles: roles.clone(),
                metadata: BTreeMap::new(),
            },
            ProcessIO {
                id: 0u64,
                roles: roles.clone(),
                metadata: BTreeMap::from_iter(vec![(1, MetadataValue::A)]),
            },
        ];
        let result = validate_restriction(
            Restriction::<u32, u32, MetadataValue, MetadataValueDisc>::FixedArgMetadataValueType {
                arg_type: ArgType::Input,
                index: 0,
                metadata_key: 1,
                metadata_value_type: MetadataValueDisc::AA,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Input => &inputs,
                _ => &other_args,
            },
        );
        assert!(!result);
    }

    #[test]
    fn input_restrict_metadata_type_incorrect_key_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let other_args = Vec::new();
        let inputs = vec![
            ProcessIO {
                id: 0u64,
                roles: roles.clone(),
                metadata: BTreeMap::new(),
            },
            ProcessIO {
                id: 0u64,
                roles: roles.clone(),
                metadata: BTreeMap::from_iter(vec![(1, MetadataValue::A)]),
            },
        ];
        let result = validate_restriction(
            Restriction::<u32, u32, MetadataValue, MetadataValueDisc>::FixedArgMetadataValueType {
                arg_type: ArgType::Input,
                index: 1,
                metadata_key: 0,
                metadata_value_type: MetadataValueDisc::AA,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Input => &inputs,
                _ => &other_args,
            },
        );
        assert!(!result);
    }

    #[test]
    fn input_restrict_metadata_type_bad_index_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let other_args = Vec::new();
        let inputs = vec![
            ProcessIO {
                id: 0u64,
                roles: roles.clone(),
                metadata: BTreeMap::new(),
            },
            ProcessIO {
                id: 0u64,
                roles: roles.clone(),
                metadata: BTreeMap::from_iter(vec![(1, MetadataValue::A)]),
            },
        ];
        let result = validate_restriction(
            Restriction::<u32, u32, MetadataValue, MetadataValueDisc>::FixedArgMetadataValueType {
                arg_type: ArgType::Input,
                index: 2,
                metadata_key: 0,
                metadata_value_type: MetadataValueDisc::AA,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Input => &inputs,
                _ => &other_args,
            },
        );
        assert!(!result);
    }

    #[test]
    fn sender_has_input_role_succeeds() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let inputs = vec![ProcessIO {
            id: 0u64,
            roles: roles.clone(),
            metadata: BTreeMap::new(),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::SenderHasArgRole {
                arg_type: ArgType::Input,
                index: 0,
                role_key: Default::default(),
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(result);
    }

    #[test]
    fn sender_has_input_role_incorrect_account_id_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 2)]);
        let inputs = vec![ProcessIO {
            id: 0u64,
            roles: roles.clone(),
            metadata: BTreeMap::new(),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::SenderHasArgRole {
                arg_type: ArgType::Input,
                index: 0,
                role_key: Default::default(),
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn sender_has_input_role_incorrect_index_fails() {
        let roles0 = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let roles1 = BTreeMap::from_iter(vec![(Default::default(), 2)]);
        let inputs = vec![
            ProcessIO {
                id: 0u64,
                roles: roles0.clone(),
                metadata: BTreeMap::new(),
            },
            ProcessIO {
                id: 0u64,
                roles: roles1.clone(),
                metadata: BTreeMap::new(),
            },
        ];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::SenderHasArgRole {
                arg_type: ArgType::Input,
                index: 1,
                role_key: Default::default(),
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn sender_has_input_role_incorrect_role_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1), (0, 1), (1, 2)]);
        let inputs = vec![ProcessIO {
            id: 0u64,
            roles: roles.clone(),
            metadata: BTreeMap::new(),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::SenderHasArgRole {
                arg_type: ArgType::Input,
                index: 0,
                role_key: 1,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn sender_has_input_role_bad_index_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1), (0, 1), (1, 2)]);
        let inputs = vec![ProcessIO {
            id: 0u64,
            roles: roles.clone(),
            metadata: BTreeMap::new(),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::SenderHasArgRole {
                arg_type: ArgType::Input,
                index: 1,
                role_key: 1,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn sender_has_output_role_succeeds() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let outputs = vec![ProcessIO {
            id: 0u64,
            roles: roles.clone(),
            metadata: BTreeMap::new(),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::SenderHasArgRole {
                arg_type: ArgType::Output,
                index: 0,
                role_key: Default::default(),
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(result);
    }

    #[test]
    fn sender_has_output_role_incorrect_account_id_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 2)]);
        let outputs = vec![ProcessIO {
            id: 0u64,
            roles: roles.clone(),
            metadata: BTreeMap::new(),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::SenderHasArgRole {
                arg_type: ArgType::Output,
                index: 0,
                role_key: Default::default(),
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn sender_has_output_role_incorrect_index_fails() {
        let roles0 = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let roles1 = BTreeMap::from_iter(vec![(Default::default(), 2)]);
        let outputs = vec![
            ProcessIO {
                id: 0u64,
                roles: roles0.clone(),
                metadata: BTreeMap::new(),
            },
            ProcessIO {
                id: 0u64,
                roles: roles1.clone(),
                metadata: BTreeMap::new(),
            },
        ];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::SenderHasArgRole {
                arg_type: ArgType::Output,
                index: 1,
                role_key: Default::default(),
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn sender_has_output_role_incorrect_role_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1), (0, 1), (1, 2)]);
        let outputs = vec![ProcessIO {
            id: 0u64,
            roles: roles.clone(),
            metadata: BTreeMap::new(),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::SenderHasArgRole {
                arg_type: ArgType::Output,
                index: 0,
                role_key: 1,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn sender_has_output_role_bad_index_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1), (0, 1), (1, 2)]);
        let outputs = vec![ProcessIO {
            id: 0u64,
            roles: roles.clone(),
            metadata: BTreeMap::new(),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::SenderHasArgRole {
                arg_type: ArgType::Output,
                index: 1,
                role_key: 1,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn output_has_role_succeeds() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1), (1, 1)]);
        let outputs = vec![ProcessIO {
            id: 0u64,
            roles: roles.clone(),
            metadata: BTreeMap::new(),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::ArgHasRole {
                arg_type: ArgType::Output,
                index: 0,
                role_key: 1,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(result);
    }

    #[test]
    fn output_has_role_incorrect_role_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1), (1, 1)]);
        let outputs = vec![ProcessIO {
            id: 0u64,
            roles: roles.clone(),
            metadata: BTreeMap::new(),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::ArgHasRole {
                arg_type: ArgType::Output,
                index: 0,
                role_key: 2,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn output_has_role_incorrect_index_fails() {
        let roles0 = BTreeMap::from_iter(vec![(Default::default(), 1), (1, 1)]);
        let roles1 = BTreeMap::from_iter(vec![(Default::default(), 1), (2, 1)]);
        let outputs = vec![
            ProcessIO {
                id: 0u64,
                roles: roles0.clone(),
                metadata: BTreeMap::new(),
            },
            ProcessIO {
                id: 0u64,
                roles: roles1.clone(),
                metadata: BTreeMap::new(),
            },
        ];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::ArgHasRole {
                arg_type: ArgType::Output,
                index: 1,
                role_key: 1,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn output_has_role_bad_index_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1), (1, 1)]);
        let outputs = vec![ProcessIO {
            id: 0u64,
            roles: roles.clone(),
            metadata: BTreeMap::new(),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::ArgHasRole {
                arg_type: ArgType::Output,
                index: 1,
                role_key: 1,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn match_input_output_role_same_role_keys_succeeds() {
        let input_roles = BTreeMap::from_iter(vec![(0, 1)]);
        let output_roles = BTreeMap::from_iter(vec![(0, 1)]);
        let inputs = vec![ProcessIO {
            id: 0u64,
            roles: input_roles.clone(),
            metadata: BTreeMap::new(),
        }];
        let outputs = vec![ProcessIO {
            id: 0u64,
            roles: output_roles.clone(),
            metadata: BTreeMap::new(),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::MatchArgsRole {
                left_arg_type: ArgType::Input,
                right_arg_type: ArgType::Output,
                left_index: 0,
                left_role_key: 0,
                right_index: 0,
                right_role_key: 0,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(result);
    }

    #[test]
    fn match_input_output_role_different_role_keys_succeeds() {
        let input_roles = BTreeMap::from_iter(vec![(0, 2)]);
        let output_roles = BTreeMap::from_iter(vec![(0, 1), (1, 2)]);
        let inputs = vec![ProcessIO {
            id: 0u64,
            roles: input_roles.clone(),
            metadata: BTreeMap::new(),
        }];
        let outputs = vec![ProcessIO {
            id: 0u64,
            roles: output_roles.clone(),
            metadata: BTreeMap::new(),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::MatchArgsRole {
                left_arg_type: ArgType::Input,
                right_arg_type: ArgType::Output,
                left_index: 0,
                left_role_key: 0,
                right_index: 0,
                right_role_key: 1,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(result);
    }

    #[test]
    fn match_input_output_role_only_one_has_key_fails() {
        let input_roles = BTreeMap::from_iter(vec![(0, 1)]);
        let output_roles = BTreeMap::from_iter(vec![(0, 1), (1, 2)]);
        let inputs = vec![ProcessIO {
            id: 0u64,
            roles: input_roles.clone(),
            metadata: BTreeMap::new(),
        }];
        let outputs = vec![ProcessIO {
            id: 0u64,
            roles: output_roles.clone(),
            metadata: BTreeMap::new(),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::MatchArgsRole {
                left_arg_type: ArgType::Input,
                right_arg_type: ArgType::Output,
                left_index: 0,
                left_role_key: 1,
                right_index: 0,
                right_role_key: 1,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn match_input_output_role_neither_has_key_fails() {
        let input_roles = BTreeMap::from_iter(vec![(0, 1)]);
        let output_roles = BTreeMap::from_iter(vec![(0, 1)]);
        let inputs = vec![ProcessIO {
            id: 0u64,
            roles: input_roles.clone(),
            metadata: BTreeMap::new(),
        }];
        let outputs = vec![ProcessIO {
            id: 0u64,
            roles: output_roles.clone(),
            metadata: BTreeMap::new(),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::MatchArgsRole {
                left_arg_type: ArgType::Input,
                right_arg_type: ArgType::Output,
                left_index: 0,
                left_role_key: 1,
                right_index: 0,
                right_role_key: 1,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
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
                id: 0u64,
                roles: input_roles0.clone(),
                metadata: BTreeMap::new(),
            },
            ProcessIO {
                id: 0u64,
                roles: input_roles1.clone(),
                metadata: BTreeMap::new(),
            },
        ];
        let outputs = vec![ProcessIO {
            id: 0u64,
            roles: output_roles.clone(),
            metadata: BTreeMap::new(),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::MatchArgsRole {
                left_arg_type: ArgType::Input,
                right_arg_type: ArgType::Output,
                left_index: 1,
                left_role_key: 1,
                right_index: 0,
                right_role_key: 1,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn match_input_output_role_bad_input_index_fails() {
        let input_roles = BTreeMap::from_iter(vec![(0, 1)]);
        let output_roles = BTreeMap::from_iter(vec![(0, 1)]);
        let inputs = vec![ProcessIO {
            id: 0u64,
            roles: input_roles.clone(),
            metadata: BTreeMap::new(),
        }];
        let outputs = vec![ProcessIO {
            id: 0u64,
            roles: output_roles.clone(),
            metadata: BTreeMap::new(),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::MatchArgsRole {
                left_arg_type: ArgType::Input,
                right_arg_type: ArgType::Output,
                left_index: 1,
                left_role_key: 0,
                right_index: 0,
                right_role_key: 0,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn match_input_output_role_bad_right_index_fails() {
        let input_roles = BTreeMap::from_iter(vec![(0, 1)]);
        let output_roles = BTreeMap::from_iter(vec![(0, 1)]);
        let inputs = vec![ProcessIO {
            id: 0u64,
            roles: input_roles.clone(),
            metadata: BTreeMap::new(),
        }];
        let outputs = vec![ProcessIO {
            id: 0u64,
            roles: output_roles.clone(),
            metadata: BTreeMap::new(),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::MatchArgsRole {
                left_arg_type: ArgType::Input,
                right_arg_type: ArgType::Output,
                left_index: 0,
                left_role_key: 0,
                right_index: 1,
                right_role_key: 0,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn match_input_output_metadata_value_same_metadata_keys_succeeds() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let input_metadata = BTreeMap::from_iter(vec![(0, 0)]);
        let output_metadata = BTreeMap::from_iter(vec![(0, 0)]);
        let inputs = vec![ProcessIO {
            id: 0u64,
            roles: roles.clone(),
            metadata: input_metadata.clone(),
        }];
        let outputs = vec![ProcessIO {
            id: 0u64,
            roles: roles.clone(),
            metadata: output_metadata.clone(),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::MatchArgsMetadataValue {
                left_arg_type: ArgType::Input,
                right_arg_type: ArgType::Output,
                left_index: 0,
                left_metadata_key: 0,
                right_index: 0,
                right_metadata_key: 0,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(result);
    }

    #[test]
    fn match_input_output_metadata_value_different_metadata_keys_succeeds() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let input_metadata = BTreeMap::from_iter(vec![(0, 0)]);
        let output_metadata = BTreeMap::from_iter(vec![(1, 0)]);
        let inputs = vec![ProcessIO {
            id: 0u64,
            roles: roles.clone(),
            metadata: input_metadata.clone(),
        }];
        let outputs = vec![ProcessIO {
            id: 0u64,
            roles: roles.clone(),
            metadata: output_metadata.clone(),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::MatchArgsMetadataValue {
                left_arg_type: ArgType::Input,
                right_arg_type: ArgType::Output,
                left_index: 0,
                left_metadata_key: 0,
                right_index: 0,
                right_metadata_key: 1,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(result);
    }

    #[test]
    fn match_input_output_metadata_value_only_one_has_key_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let input_metadata = BTreeMap::from_iter(vec![(0, 0)]);
        let output_metadata = BTreeMap::new();
        let inputs = vec![ProcessIO {
            id: 0u64,
            roles: roles.clone(),
            metadata: input_metadata.clone(),
        }];
        let outputs = vec![ProcessIO {
            id: 0u64,
            roles: roles.clone(),
            metadata: output_metadata.clone(),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::MatchArgsMetadataValue {
                left_arg_type: ArgType::Input,
                right_arg_type: ArgType::Output,
                left_index: 0,
                left_metadata_key: 0,
                right_index: 0,
                right_metadata_key: 0,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn match_input_output_metadata_value_neither_has_key_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let input_metadata = BTreeMap::new();
        let output_metadata = BTreeMap::new();
        let inputs = vec![ProcessIO {
            id: 0u64,
            roles: roles.clone(),
            metadata: input_metadata.clone(),
        }];
        let outputs = vec![ProcessIO {
            id: 0u64,
            roles: roles.clone(),
            metadata: output_metadata.clone(),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::MatchArgsMetadataValue {
                left_arg_type: ArgType::Input,
                right_arg_type: ArgType::Output,
                left_index: 0,
                left_metadata_key: 0,
                right_index: 0,
                right_metadata_key: 0,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
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
                id: 0u64,
                roles: roles.clone(),
                metadata: input_metadata0.clone(),
            },
            ProcessIO {
                id: 0u64,
                roles: roles.clone(),
                metadata: input_metadata1.clone(),
            },
        ];
        let outputs = vec![ProcessIO {
            id: 0u64,
            roles: roles.clone(),
            metadata: output_metadata.clone(),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::MatchArgsMetadataValue {
                left_arg_type: ArgType::Input,
                right_arg_type: ArgType::Output,
                left_index: 1,
                left_metadata_key: 0,
                right_index: 0,
                right_metadata_key: 0,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn match_input_output_metadata_value_bad_left_index_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let input_metadata = BTreeMap::from_iter(vec![(0, 0)]);
        let output_metadata = BTreeMap::from_iter(vec![(0, 0)]);
        let inputs = vec![ProcessIO {
            id: 0u64,
            roles: roles.clone(),
            metadata: input_metadata.clone(),
        }];
        let outputs = vec![ProcessIO {
            id: 0u64,
            roles: roles.clone(),
            metadata: output_metadata.clone(),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::MatchArgsMetadataValue {
                left_arg_type: ArgType::Input,
                right_arg_type: ArgType::Output,
                left_index: 1,
                left_metadata_key: 0,
                right_index: 0,
                right_metadata_key: 0,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn match_input_output_metadata_value_bad_right_index_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let input_metadata = BTreeMap::from_iter(vec![(0, 0)]);
        let output_metadata = BTreeMap::from_iter(vec![(0, 0)]);
        let inputs = vec![ProcessIO {
            id: 0u64,
            roles: roles.clone(),
            metadata: input_metadata.clone(),
        }];
        let outputs = vec![ProcessIO {
            id: 0u64,
            roles: roles.clone(),
            metadata: output_metadata.clone(),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::MatchArgsMetadataValue {
                left_arg_type: ArgType::Input,
                right_arg_type: ArgType::Output,
                left_index: 0,
                left_metadata_key: 0,
                right_index: 1,
                right_metadata_key: 0,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn match_input_id_output_metadata_value_simple_succeeds() {
        let inputs = vec![ProcessIO {
            id: 42u64,
            roles: BTreeMap::new(),
            metadata: BTreeMap::new(),
        }];
        let outputs = vec![ProcessIO {
            id: 43u64,
            roles: BTreeMap::new(),
            metadata: BTreeMap::from_iter(vec![(1, 42)]),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::MatchArgIdToMetadataValue {
                left_arg_type: ArgType::Input,
                right_arg_type: ArgType::Output,
                left_index: 0,
                right_index: 0,
                right_metadata_key: 1,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(result);
    }

    #[test]
    fn match_input_id_output_metadata_value_incorrect_metadata_key_fails() {
        let inputs = vec![ProcessIO {
            id: 42u64,
            roles: BTreeMap::new(),
            metadata: BTreeMap::new(),
        }];
        let outputs = vec![ProcessIO {
            id: 43u64,
            roles: BTreeMap::new(),
            metadata: BTreeMap::from_iter(vec![(2, 42)]),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::MatchArgIdToMetadataValue {
                left_arg_type: ArgType::Input,
                right_arg_type: ArgType::Output,
                left_index: 0,
                right_index: 0,
                right_metadata_key: 1,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn match_input_id_output_metadata_value_incorrect_id_fails() {
        let inputs = vec![ProcessIO {
            id: 42u64,
            roles: BTreeMap::new(),
            metadata: BTreeMap::new(),
        }];
        let outputs = vec![ProcessIO {
            id: 43u64,
            roles: BTreeMap::new(),
            metadata: BTreeMap::from_iter(vec![(1, 40)]),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::MatchArgIdToMetadataValue {
                left_arg_type: ArgType::Input,
                right_arg_type: ArgType::Output,
                left_index: 0,
                right_index: 0,
                right_metadata_key: 1,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn match_input_id_output_metadata_value_incorrect_input_index_fails() {
        let inputs = vec![
            ProcessIO {
                id: 42u64,
                roles: BTreeMap::new(),
                metadata: BTreeMap::new(),
            },
            ProcessIO {
                id: 41u64,
                roles: BTreeMap::new(),
                metadata: BTreeMap::new(),
            },
        ];
        let outputs = vec![ProcessIO {
            id: 43u64,
            roles: BTreeMap::new(),
            metadata: BTreeMap::from_iter(vec![(1, 42)]),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::MatchArgIdToMetadataValue {
                left_arg_type: ArgType::Input,
                right_arg_type: ArgType::Output,
                left_index: 1,
                right_index: 0,
                right_metadata_key: 1,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn match_input_id_output_metadata_value_incorrect_output_index_fails() {
        let inputs = vec![ProcessIO {
            id: 42u64,
            roles: BTreeMap::new(),
            metadata: BTreeMap::new(),
        }];
        let outputs = vec![
            ProcessIO {
                id: 43u64,
                roles: BTreeMap::new(),
                metadata: BTreeMap::from_iter(vec![(1, 42)]),
            },
            ProcessIO {
                id: 43u64,
                roles: BTreeMap::new(),
                metadata: BTreeMap::from_iter(vec![(1, 41)]),
            },
        ];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::MatchArgIdToMetadataValue {
                left_arg_type: ArgType::Input,
                right_arg_type: ArgType::Output,
                left_index: 0,
                right_index: 1,
                right_metadata_key: 1,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn match_input_id_output_metadata_value_bad_input_index_fails() {
        let inputs = vec![ProcessIO {
            id: 42u64,
            roles: BTreeMap::new(),
            metadata: BTreeMap::new(),
        }];
        let outputs = vec![ProcessIO {
            id: 43u64,
            roles: BTreeMap::new(),
            metadata: BTreeMap::from_iter(vec![(1, 42)]),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::MatchArgIdToMetadataValue {
                left_arg_type: ArgType::Input,
                right_arg_type: ArgType::Output,
                left_index: 1,
                right_index: 0,
                right_metadata_key: 1,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn match_input_id_output_metadata_value_bad_output_index_fails() {
        let inputs = vec![ProcessIO {
            id: 42u64,
            roles: BTreeMap::new(),
            metadata: BTreeMap::new(),
        }];
        let outputs = vec![ProcessIO {
            id: 43u64,
            roles: BTreeMap::new(),
            metadata: BTreeMap::from_iter(vec![(1, 42)]),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::MatchArgIdToMetadataValue {
                left_arg_type: ArgType::Input,
                right_arg_type: ArgType::Output,
                left_index: 0,
                right_index: 1,
                right_metadata_key: 1,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn output_has_metadata_value_succeeds() {
        let metadata = BTreeMap::from_iter(vec![(Default::default(), 1), (1, 1)]);
        let outputs = vec![ProcessIO {
            id: 0u64,
            roles: BTreeMap::new(),
            metadata: metadata.clone(),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::ArgHasMetadata {
                arg_type: ArgType::Output,
                index: 0,
                metadata_key: 1,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(result);
    }

    #[test]
    fn output_has_metadata_value_incorrect_role_fails() {
        let metadata = BTreeMap::from_iter(vec![(Default::default(), 1), (1, 1)]);
        let outputs = vec![ProcessIO {
            id: 0u64,
            roles: BTreeMap::new(),
            metadata: metadata.clone(),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::ArgHasMetadata {
                arg_type: ArgType::Output,
                index: 0,
                metadata_key: 2,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn output_has_metadata_value_incorrect_index_fails() {
        let metadata0 = BTreeMap::from_iter(vec![(Default::default(), 1), (1, 1)]);
        let metadata1 = BTreeMap::from_iter(vec![(Default::default(), 1), (2, 1)]);
        let outputs = vec![
            ProcessIO {
                id: 0u64,
                roles: BTreeMap::new(),
                metadata: metadata0.clone(),
            },
            ProcessIO {
                id: 0u64,
                roles: BTreeMap::new(),
                metadata: metadata1.clone(),
            },
        ];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::ArgHasMetadata {
                arg_type: ArgType::Output,
                index: 1,
                metadata_key: 1,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn output_has_metadata_value_bad_index_fails() {
        let metadata = BTreeMap::from_iter(vec![(Default::default(), 1), (1, 1)]);
        let outputs = vec![ProcessIO {
            id: 0u64,
            roles: BTreeMap::new(),
            metadata: metadata.clone(),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::ArgHasMetadata {
                arg_type: ArgType::Output,
                index: 1,
                metadata_key: 1,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Output => &outputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn input_has_role_succeeds() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1), (1, 1)]);
        let inputs = vec![ProcessIO {
            id: 0u64,
            roles: roles.clone(),
            metadata: BTreeMap::new(),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::ArgHasRole {
                arg_type: ArgType::Input,
                index: 0,
                role_key: 1,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(result);
    }

    #[test]
    fn input_has_role_incorrect_role_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1), (1, 1)]);
        let inputs = vec![ProcessIO {
            id: 0u64,
            roles: roles.clone(),
            metadata: BTreeMap::new(),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::ArgHasRole {
                arg_type: ArgType::Input,
                index: 0,
                role_key: 2,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn input_has_role_incorrect_index_fails() {
        let roles0 = BTreeMap::from_iter(vec![(Default::default(), 1), (1, 1)]);
        let roles1 = BTreeMap::from_iter(vec![(Default::default(), 1), (2, 1)]);
        let inputs = vec![
            ProcessIO {
                id: 0u64,
                roles: roles0.clone(),
                metadata: BTreeMap::new(),
            },
            ProcessIO {
                id: 0u64,
                roles: roles1.clone(),
                metadata: BTreeMap::new(),
            },
        ];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::ArgHasRole {
                arg_type: ArgType::Input,
                index: 1,
                role_key: 1,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn input_has_role_bad_index_fails() {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1), (1, 1)]);
        let inputs = vec![ProcessIO {
            id: 0u64,
            roles: roles.clone(),
            metadata: BTreeMap::new(),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::ArgHasRole {
                arg_type: ArgType::Input,
                index: 1,
                role_key: 1,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn input_has_metadata_value_succeeds() {
        let metadata = BTreeMap::from_iter(vec![(Default::default(), 1), (1, 1)]);
        let inputs = vec![ProcessIO {
            id: 0u64,
            roles: BTreeMap::new(),
            metadata: metadata.clone(),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::ArgHasMetadata {
                arg_type: ArgType::Input,
                index: 0,
                metadata_key: 1,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(result);
    }

    #[test]
    fn input_has_metadata_value_incorrect_role_fails() {
        let metadata = BTreeMap::from_iter(vec![(Default::default(), 1), (1, 1)]);
        let inputs = vec![ProcessIO {
            id: 0u64,
            roles: BTreeMap::new(),
            metadata: metadata.clone(),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::ArgHasMetadata {
                arg_type: ArgType::Input,
                index: 0,
                metadata_key: 2,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn input_has_metadata_value_incorrect_index_fails() {
        let metadata0 = BTreeMap::from_iter(vec![(Default::default(), 1), (1, 1)]);
        let metadata1 = BTreeMap::from_iter(vec![(Default::default(), 1), (2, 1)]);
        let inputs = vec![
            ProcessIO {
                id: 0u64,
                roles: BTreeMap::new(),
                metadata: metadata0.clone(),
            },
            ProcessIO {
                id: 0u64,
                roles: BTreeMap::new(),
                metadata: metadata1.clone(),
            },
        ];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::ArgHasMetadata {
                arg_type: ArgType::Input,
                index: 1,
                metadata_key: 1,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }

    #[test]
    fn input_has_metadata_value_bad_index_fails() {
        let metadata = BTreeMap::from_iter(vec![(Default::default(), 1), (1, 1)]);
        let inputs = vec![ProcessIO {
            id: 0u64,
            roles: BTreeMap::new(),
            metadata: metadata.clone(),
        }];
        let result = validate_restriction(
            Restriction::<u32, u32, u64, u64>::ArgHasMetadata {
                arg_type: ArgType::Input,
                index: 1,
                metadata_key: 1,
            },
            &RawOrigin::Signed(1u64),
            |a| match a {
                ArgType::Input => &inputs,
                _ => &EMPTY_ARGS,
            },
        );
        assert!(!result);
    }
}
